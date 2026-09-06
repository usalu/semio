# Hub Concurrent First-Document Mount Race

## Outcome

Two authorized socket hellos can legitimately fail before Welcome. The cause is an unowned first-mount race below Hub, not a reason to serialize the test. Database records only completed ArtifactAuthority instances. During a durable catalog publication and actor startup it has no per-document Opening state.

No build was started. The diagnostic rerun is behind active typed-WAL compilation, so this audit does not attribute the observed failure to one particular ServerFrame. Current source proves two failure schedules.

## Evidence

| Path | Current behavior | Result |
| --- | --- | --- |
| 🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1758 | ensure_document calls document, then create_document only after NotFound; it retries only AlreadyExists. | It neither owns nor joins an in-flight mount. |
| 🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3408 | A socket sends storage error then returns on ensure error. | The error necessarily precedes Welcome. |
| 🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9169 | The recovery law opens B and C, sends both hellos, then awaits either Welcome (9171–9180). | The test is correctly concurrent and must stay so. |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7677 | create_document publishes its catalog transaction, awaits spawn_authority_create, then registers at 7694. | A published-but-unmounted interval exists. |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7702 | document checks open_artifacts and catalog separately, awaits spawn_authority_open, then blindly registers at 7713. | Two readers of a known unopened document can construct authorities. |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7665 | register_handle does a plain HashMap insert. | It can overwrite a live authority without selecting a winner or shutting down the replaced one. |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:11207 | Existing catalog contention law proves one of two concurrent creates returns DbError::Fenced; only an already-published duplicate returns AlreadyExists. | Hub's AlreadyExists retry is insufficient. |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2412 | ArtifactWal create and open both acquire the document writer. | Duplicate mount becomes a real writer conflict rather than shared actor admission. |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:94 | WalWriterTable rejects an existing document writer with DbError::Conflict. | Durable mutation is contained, but first open can fail nondeterministically. |

The test at engine.rs:11717 deliberately proves the intermediate state: its catalog transaction has published yet open_artifacts is empty. This is not theoretical.

### Schedules

First, B and C can both get NotFound, then race catalog creation. The existing law proves that one result can be Fenced, which Hub returns as storage failure before Welcome.

Second, B can finish catalog publication and still await spawn_authority_create. C can receive AlreadyExists, call document, see the catalog entry but no open actor, and independently call spawn_authority_open. Both then contend for the exclusive writer. Under the current writer table C normally gets a conflict-carrying construction rejection and no Welcome. If a backend admitted both, register_handle would overwrite one authority. The Hub connection retains a successful ArtifactHandle in its full connection loop at bin.rs:3655, so such overwriting is not benign.

## Retained rejection

DatabaseDocumentOpenRejected is explicitly must-close at engine.rs:7375. Its retry_close at 7401 can retain the writer until terminal. The database and artifact test modules both use a retry-to-terminal helper at engine.rs:9594 and artifact.rs:4199.

There is also a current static mismatch in Hub: ensure_document declares Result with DbError but matches the error returned by Database::document as DbError. The typed rejection has no source From-to-DbError or Display implementation. The CLI correctly owns a close helper at db/⌨️cli/🦀️.rs:183. This is a source finding, not a new build result. Dropping or lossy-converting a rejection would abandon the owner it protects.

## Smallest correct correction

Place a single-flight per-document mount owner inside Database; do not serialize Hub test traffic and do not repair only Hub.

Replace the ready-only map with bounded, generation-stamped slots:

    DocumentMountSlot =
      Opening { owner: DocumentMountOwner, fixed waiters }
    | Ready   { authority: Arc<ArtifactAuthority> }

The owner exclusively decides catalog create versus open and owns exactly one authority construction future. It must outlive the initiating socket request. Bounded per-waiter replies can use db_actor oneshot at db/🎭️actor/🦀️.rs:492; each sender is write-once, so store one per waiter. DbError is Clone at db/🆔️ids/🦀️.rs:72 for terminal error fanout.

Required transitions:

1. Under the mount mutex, return Ready, join exact Opening, or install Opening with a new generation before any await. Never hold that mutex across catalog, WAL, worker, or reply work.
2. The owner reads the catalog once. A new Database ensure_document chooses create if absent and open if present. Existing create_document and document must use the same primitive with their respective AlreadyExists and NotFound policy.
3. An Ensure catalog loser re-reads after Fenced or AlreadyExists and continues as open in the same owner; it must not expose ordinary first-open contention to a socket.
4. Replace the matching Opening generation with Ready while locked, then release waiters with handles cloned from that exact Arc.
5. On construction error, retain DatabaseDocumentOpenRejected in the owner and drive retry_close to a terminal DbError. Until then the exact Opening stays installed and no later caller may create a second mount or receive a terminal failure.
6. Initiator cancellation removes only its waiter. Database shutdown includes Opening owners and drives their close before releasing waiters.

Make this one Database ensure_document API returning the typed retained rejection, then remove the Hub TOCTOU helper. Hub may map the typed rejection to DbError only after its terminal retry_close loop, before emitting a storage frame.

## Laws

1. DB native cold concurrent ensure: start two ensure calls against an empty document without serializing. Both return handles from one mounted actor, catalog has one entry, and direct writer acquisition is Conflict while handles live.
2. DB native published-not-registered: hold the first owner after the proven catalog publication point, join another ensure, and prove one actor, no overwrite, same post-ready frontier.
3. DB native retained failure: inject construction failure after writer acquisition. All joiners wait through a first close failure; after terminal close a new ensure gets exactly one writer.
4. Hub mounted SQLite: retain the existing concurrent B/C hello order, require both Welcomes, submit via B, then observe the changed frontier via C.
5. Hub typed-rejection: force a retained WAL construction error; error frame occurs only after terminal cleanup, and a later connection can mount.

## Scope

This packet does not change presence, plan authorization, SQLite recovery semantics, or the closed browser actor bundle. It isolates the missing Database-owned document mount lifetime.

