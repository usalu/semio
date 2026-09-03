# Hub Canonical Artifact Authority Audit

## Decision

**The hub must own a live, headless canonical artifact authority backed by a
trusted package catalog and registered codecs.** It must derive, validate, and
persist every public `(pack, spr)` checkpoint from accepted operations. The
hub must not publish either bytes or a hash supplied by a client as canonical.

A client hash proves at most that that client hashed some bytes; it does not
bind those bytes to the immutable descriptor, the declared package code, the
causal frontier, or the accepted operation history. A signed remote
attestation could eventually be an alternative, but there is currently no
attestation schema, public-key/trust policy, signer lifecycle, replay rule, or
semantic verifier. It is not a substitute for the first implementation.

The existing public protocol and client-side atomic installer are usable
transport/consumer pieces. The missing producer is a server-owned authority,
not another client bootstrap format.

## Audit Boundary and Evidence

This is a read-only audit. The following umbrella reports were read in full:

- `📓️terra-hub-snapshot-workspace-audit.md`
- `📓️sol-document-descriptor.md`
- `📓️sol-artifact-bootstrap-protocol.md`
- `📓️sol-atomic-bootstrap-restore.md`

`bun nx show project os-hub` was used only to inspect the registered targets;
no build or test was run by this audit. Source anchors below refer to the
current working tree, which is concurrently edited.

| Observed fact | Source anchor | Consequence |
| --- | --- | --- |
| Public `Bootstrap::ArtifactBootstrap`, chunk/done frames, a bounded assembler, hashes, a required tail frontier, cancellation/deadline control and progress already exist. | `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:64-370, 489-549, 643-746, 960-968` | The hub can produce the already-public format. It must not introduce a second snapshot wire. |
| Protocol defaults are format v1, 4 KiB chunks, 64 MiB total, and 16,384 chunks. | `.../📡️wire/🦀️.rs:69-72, 108-110, 163-165` | Sender policy must remain inside these hard limits. |
| `DocumentDescriptor` is immutable and is announced once through `document.announced`; a distinct descriptor for the same `(space, document)` is rejected. | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:93-114, 136-149, 230-252`; `🌎️hub/📇️directory/🦀️.rs:493-499` | A changing checkpoint cannot be stored in the descriptor. It needs an append-only checkpoint stream/read model. |
| `HubDirectory` currently exposes only descriptor reads for documents. | `🌎️hub/📇️directory/🦀️.rs:603-636, 892-910` | There is no checkpoint authority port or projection today. |
| SQLite and PostgreSQL key descriptor projection rows by `(space_id, document_id)`; Neo4j uses a length-prefixed `scopeKey`. | `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:58-61, 253-256, 406-421`; `🌎️hub/📇️directory/🐘️postgres/🦀️.rs:54-59, 263-267, 436-452`; `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:43, 180-187, 378-387` | A checkpoint must also use structural `(space_id, document_id)` identity in every backend. Same document text in different spaces is intentionally distinct. |
| The hub converts scope to `format!("{space}:{document}")` for the flat DB catalog/fanout. | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:118-127` | This is ambiguous if either ID can contain `:`. The authority must use structural scope; the hub DB key needs a new unambiguous, byte-length-prefixed v1 encoding before it represents authority. |
| WS authenticates, loads the descriptor, checks Hello schema/hash, then calls `Database::hello`; it has no artifact authority lookup or `ArtifactBootstrap` emission. | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:759-895` | A validated client codec identity is not a server checkpoint. Current cold path remains DB-private snapshot/tail. |
| The WS broadcast loop silently ignores `RecvError::Lagged`. The directory WS loop does the same. | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:961-964, 1359-1362` | A lagged recipient can silently miss mutations. It must be terminated/rebootstrapped, never allowed to resume as if contiguous. |
| DB's public `ArtifactHandle::snapshot_now` is explicitly `Unimplemented`; the lower `db_artifact::ArtifactEngine::snapshot_now` creates internal DB state pages. | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:1-37, 9150-9155`; `.../🛢️db/📄️artifact/🦀️.rs:1588-1597, 4040-4042` | `.spk` pages are not generic OS artifact pack/SPR and cannot become public bootstrap by relabelling. |
| DB snapshot retention is generation/page retention with its own lease/fencing; the sync path uses a private `Snapshot` bootstrap and WAL floor. | `.../🛢️db/📸️snapshot/🦀️.rs:1198-1257, 1340-1425`; `📓️terra-hub-snapshot-workspace-audit.md` | Public artifact retention must be coordinated with, but never inferred from, DB snapshot/WAL retention. |
| Store has a process-local schema-keyed `ArtifactCodec`; `print_mirror` semantically reads a pair and `apply_ops_binary` applies encoded operations to a pair. | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9245-9400, 9403-9509` | A process which does not load/register the pinned plugin codec cannot validate or derive a canonical pair. |
| Hub Cargo dependencies include replication, directory, and DB, but not store/plugin/plugin-host/run. | `🌎️hub/📦️packages/🦀️rust/Cargo.toml:1-49` | Today’s hub has no code registry, package loader, or materializer. |
| MCP hub workspace only knows locally opened probes/folder state; `open_hub` merely records origin and `read_artifact_bytes` has no hub cold read. Inference discovery deliberately returns `channel.not-wired`. | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1138-1288`; `.../🌉️mcp/💡️inference/🦀️.rs:2-20, 136-160, 312-315` | Hub workspace and real map inference remain blocked after bootstrap producer work unless they get a descriptor-authorized read and a real inference route. |

The descriptor has bootstrap identity/frontier/hash fields, while the public
frame carries `descriptor_hash`. The audited descriptor schema provides no
specified canonical descriptor-byte algorithm. That is a schema gap: a
checkpoint cannot invent an implementation-dependent JSON hash and expect
native, wasm, browser, and hub to agree.

## Authority Model

### Mandatory invariants

1. `DocumentScope = { space_id, document_id }` is structural throughout the
   authority, event log, blob authorization, and projections. A DB adapter may
   derive `db_key_v1 = "<utf8-byte-length>:<space><document>"`, but may not use
   colon concatenation.
2. `descriptor_digest_v1` is SHA-256 over one specified, field-ordered binary
   encoding of *all* immutable `DocumentDescriptor` fields, prefixed by a
   protocol domain string. It is a derived value, not a mutable descriptor
   field and not JSON serialization. The shared fixture must define it.
3. A checkpoint is valid only if its descriptor digest matches the durable
   descriptor; `artifact_kind`, `artifact_schema`, `pack_schema_hash`, and the
   exact owner package hash resolve in the hub's trusted package catalog; the
   codec registered by that package accepts `(pack, spr)` through
   `print_mirror`; and the pair is a deterministic materialization of the
   accepted causal operation prefix at its frontier.
4. A client can submit an operation but cannot submit a checkpoint. A
   per-scope hub actor validates/materializes a candidate through the pinned
   codec, commits the accepted operation to DB, then advances its live pair and
   frontier. A crash after DB commit is safe because restart replays from the
   last durable public checkpoint plus retained tail; a crash before DB commit
   discards the candidate.
5. Pair blobs are written/staged before publication, but are not observable as
   a checkpoint until the checkpoint event commits. A missing/corrupt blob,
   missing codec, descriptor conflict, or failed semantic validation is
   fail-closed: no bootstrap and no floor advance.

### Chosen server shape

Create a hub-owned `CanonicalArtifactAuthority` behind a local Rust port. It
is initialized from the immutable descriptor and a **trusted, package-hash
pinned** server catalog. Its materializer owns the mapping from accepted
`MutationEnvelope` payloads to the `encode_ops_vec` input expected by
`ArtifactCodec::apply_ops_binary`; this mapping must be explicit, order
preserving, schema-checked, and fixture-tested. Existing `ArtifactCodec`
methods are sufficient primitives, but no hub code currently calls them.

The catalog may be implemented by a headless plugin-host/component runtime
rather than statically linking all codecs, but the trust boundary is unchanged:
the server verifies the exact descriptor package hash before executing/registering
that codec. An untrusted client renderer or an arbitrary downloaded package is
not an authority. A future attestation alternative would require a new,
schema-first `CheckpointAttestation` signed by an allowlisted package-key and
server replay/semantic-verification policy; it must not be added as a client
hash escape hatch.

## Durable Checkpoint Event and Read Model

### New schema-first vocabulary

Add, beside—not inside—`DocumentDescriptor`:

```text
ArtifactCheckpoint {
  scope: DocumentScope,
  checkpoint_id: CheckpointId,              // SHA-256 canonical checkpoint identity
  parent_checkpoint_id: Option<CheckpointId>,
  descriptor_digest_v1: [u8; 32],
  baseline_frontier: FrontierSummary,       // includes head/commit/epoch/head edit/chain
  pack: ArtifactBlobRef { sha256, byte_length, storage_key },
  spr:  ArtifactBlobRef { sha256, byte_length, storage_key },
  aggregate_sha256: [u8; 32],
  published_at_ms: u64
}
ArtifactRetention {
  scope: DocumentScope,
  retained_checkpoint_id: CheckpointId,
  retained_floor: FrontierSummary,
  checkpoint_lineage_head: CheckpointId
}
```

`storage_key` is private implementation data. It can refer to the existing
content-addressed payload facility, but public integrity is the protocol's
SHA-256 values, not a globally-addressable payload key. Blob reads must only
be reached through scope authorization and a referenced checkpoint.

Add server-produced `DirectoryEventBody` variants
`ArtifactCheckpointPublished { checkpoint }` and
`ArtifactRetentionAdvanced { retention }`. They are appended by an authority
service command/internal decider, never by `POST /directory/commands` from a
client. `DocumentAnnounced` remains the sole creator of descriptor identity.
The decider rejects:

- an unknown/deleted scope, missing descriptor, or differing digest;
- a checkpoint whose parent is not the current lineage head (except genesis);
- non-increasing/non-proven frontier, wrong pair aggregate, or missing staged
  blobs;
- a retention advance not naming an existing full checkpoint at exactly its
  retained floor.

Projection is a read model, not authority. It needs `active_checkpoint` and
`retention_floor` lookup methods on `HubDirectory`/`DirectoryService`, plus a
checkpoint lineage list for repair/audit. The event log remains the recovery
source.

### Backend projection requirements

| Backend | Required projection |
| --- | --- |
| SQLite | `hub_artifact_checkpoint` with composite `(space_id, document_id, checkpoint_id)` primary key and unique active/checkpoint lineage constraints; `hub_artifact_retention` keyed by `(space_id, document_id)`. Both are folded in the same transaction as `hub_directory_event`. |
| PostgreSQL | Equivalent composite tables/indexes and event-transaction folding. JSONB may retain diagnostic checkpoint detail, but the scope, checkpoint id, digest, parent, frontier, and active/floor selection fields need indexed scalar columns. |
| Neo4j | `(:DocumentDescriptor)-[:HAS_CHECKPOINT]->(:ArtifactCheckpoint)` with a unique compound scope/checkpoint key and one `:RETENTION_FLOOR` relation/property; use the same length-prefixed structural scope encoding Neo4j already uses. Projection rebuild must delete/replay checkpoint nodes with descriptor nodes. |

Space deletion revokes serving immediately and removes projections/authorized
references. Blob deletion is delayed garbage collection after no retained
event/reference remains; do not delete a blob merely because a projection was
rebuilt. Descriptor conflict remains a conflict, not a new authority branch.

### Publication and restart ordering

There is no demonstrated distributed transaction between directory backends
and DB/WAL. Use this conservative, idempotent sequence:

1. Under the per-scope authority lease, materialize and semantically validate
   the pair at frontier `B`; calculate all SHA-256 values.
2. Stage immutable pack and SPR blobs and verify a read-back hash.
3. Append `ArtifactCheckpointPublished(B)` and update the directory projection.
   Only this makes the pair publicly selectable.
4. Persist a DB/WAL retention marker referring to the checkpoint id and exact
   `B`, then prune DB commands/snapshots only after that marker succeeds.
5. Append `ArtifactRetentionAdvanced(B)` only after both the durable public
   checkpoint and the DB marker exist. Failures retain more data; they never
   make data unrecoverable.

On restart, replay directory events/projections, resolve the descriptor and
pinned codec, re-hash/read/semantically validate the active checkpoint, then
reconcile the DB marker. A marker without a valid checkpoint blocks serving;
a valid checkpoint without a marker simply retains extra tail. Existing DB
`.spk` generation retention may continue independently, but can never certify
or replace this public checkpoint.

## WS and API Flow

### WebSocket cold/reconnect flow

`ClientFrame::Hello` already supplies the client frontier. Keep it as the
request—there is no need for a second client bootstrap request frame.

1. Authenticate and authorize the requested `DocumentScope` before reading a
   descriptor, checkpoint metadata, or blob. Existing session/member/share/
   public rules are the policy starting point; checkpoint read is spectator
   permission, mutation is author permission. Revalidate *all* session forms
   during a long transfer/live session, not only the current share-token tick.
2. Load the immutable descriptor, calculate `descriptor_digest_v1`, resolve
   the exact trusted server codec, and acquire the scope read barrier. A codec
   missing for a valid descriptor yields an explicit `artifact-unavailable`,
   never DB `Snapshot` fallback.
3. Capture a stable target `T` and subscribe to the per-scope command stream
   **before** sending bootstrap bytes. Buffer post-`T` messages so no write can
   race between tail selection and live fanout registration.
4. If the advertised frontier is a verifiable prefix at/after retained floor,
   select `Bootstrap::Tail`. Otherwise select the newest valid public
   checkpoint `B` at/below `T` and emit `Welcome { bootstrap:
   ArtifactBootstrap(...B...), server_frontier: T }`, then chunks/done if not
   inline, then exactly the accepted tail `(B, T]`.
5. Drain buffered messages newer than `T`, then enter normal session/live
   broadcast. The P3 client installer remains unobservable until it has
   verified/atomically installed the pair and completed the required tail.

The existing DB `Database::hello` cannot be the selection owner until it can
ask this authority for a public checkpoint and exact tail lease. Initially,
move hello planning to the hub scope actor; retain DB only as the accepted
operation/WAL substrate. Do not emit `Bootstrap::Snapshot`,
`SnapshotChunk`, or `SnapshotDone` to public artifact consumers—the P3 clients
correctly reject those DB-private bytes.

### Frame policy and bounds

This is a proposed sender policy, not a claim that it is implemented today:

- If `pack_length + spr_length <= 4,096`, emit the pair inline in the existing
  `ArtifactBootstrap`; otherwise emit exact-order 4,096-byte-or-smaller
  `ArtifactBootstrapChunk` frames followed by `ArtifactBootstrapDone`.
- Reject a pair whose total exceeds 64 MiB or needs more than 16,384 chunks;
  never let a caller raise wire maxima. Enforce per-scope transfer concurrency,
  bytes-in-flight, blob read, and materialization budgets before allocation.
- Bind every frame to `descriptor_digest_v1`; calculate individual and
  aggregate SHA-256 from server-staged bytes. The baseline frontier in the
  header is `B`; `required_tail_frontier` is the stable target `T`.
- Client receipt progress is already observable from the existing assembler
  control/progress interface. The wire contains no remote progress/cancel
  control frame: socket close/request cancellation must cancel the authority
  read and blob stream; server metrics/audit may record progress but must not
  pretend it was protocol progress. Use an operation deadline at authority,
  blob-read, and stream boundaries.
- Publication becomes server-observable only after step 3 above. Installation
  becomes client-observable only through P3's atomic `SnapshotReplaced` after
  assembler/hash/codec validation and required-tail completion.

### Lag, revocation, and conflict handling

On `broadcast::RecvError::Lagged`, send the existing protocol error
`rebootstrap-required` if writable, then close the document WS with code 1013
and reason `rebootstrap-required`; discard its buffered/live state. The next
Hello re-runs the selection above. Do not silently skip, and do not try to
continue a partial bootstrap. This needs the exact same treatment for the
directory stream when directory continuity matters.

Revocation/expiry during chunks or tail stops transmission and closes without
revealing further bytes. A revoked reader reconnects to unauthorized. A
descriptor conflict is rejected at announce time; a package/catalog mismatch
or corrupt authority lineage blocks the scope rather than serving a different
descriptor or client-supplied hash.

## Ordered Implementation Packets

### P2-A — Contract, scope safety, and deterministic materializer (first)

**Prerequisite and blocker:** no safe checkpoint producer exists until the hub
can load a descriptor-pinned codec and deterministically convert accepted
envelopes into `ArtifactCodec::apply_ops_binary` input. This is independent of
WS transport and must land first.

- Add a schema/fixture for `DocumentScope`, `descriptor_digest_v1`,
  `ArtifactCheckpoint`, `ArtifactRetention`, and canonical binary encodings
  beside `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`.
  Put the language-neutral cases under that module's `🧫️fixtures` tree and
  validate them from Rust and the existing TypeScript/AJV harness.
- Add a hub-local authority port/module, e.g.
  `🌎️hub/🗿️artifact-authority/🦀️.rs`, defining
  `TrustedArtifactCatalog`, `CanonicalArtifactAuthority`, `CheckpointCandidate`,
  and an `OperationContext`-based cancellable materialization method. Expose
  no external store/plugin types from this port.
- Add a store/plugin adapter next to
  `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9245-9509` and
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` that resolves
  the descriptor's exact owner package hash and only then invokes the codec.
  It must verify pack/SPR with `print_mirror`, apply ordered operations, and
  reverify the resulting pair.
- Replace `scope_key`/`db_artifact_id` at
  `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:118-127` with a named unambiguous v1 codec
  before checkpoint IDs/DB records depend on it. Greenfield rules permit one
  clean representation; no compatibility/migration branch is needed.
- Tests/oracles: known descriptor digest vectors; colon-containing and
  same-document/different-space scopes; one registered real codec derives
  exactly the expected pair; malformed/mismatched package/hash/operation never
  reaches publication. Cross-check SHA-256 with Node `crypto.createHash` and
  schema values with AJV, while `print_mirror` is the independent semantic
  oracle.

### P2-B — Durable authority and three backend projections

- Extend the directory event schema/decider and `HubDirectory` in
  `🌎️hub/📇️directory/🦀️.rs` with server-only checkpoint publish, active lookup,
  retention advance, lineage lookup, and projection rebuild.
- Add the SQLite tables/fold/read methods in
  `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`; matching PostgreSQL schema/folds in
  `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`; matching Neo4j nodes/relations/rebuild
  in `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`.
- Implement staged pair-blob references through a private authority storage
  port; do not expose `db::PayloadStorage` as a public URL and do not reuse the
  private `db_snapshot` format. Tie event append, active projection, WAL marker,
  and conservative cleanup to the five-step ordering above.
- Tests/oracles: idempotent publish; parent/floor violation rejection;
  projection deletion/rebuild equals event replay; restart after each durable
  boundary; missing/corrupt blob and missing codec fail closed; exact scope
  isolation in SQLite, PostgreSQL, and Neo4j.

### P2-C — Hub hello selection, public producer, and forced rebootstrap

- Add the authority field to `HubState`, dependency wiring to
  `🌎️hub/📦️packages/🦀️rust/Cargo.toml`, and scope-actor admission/hello
  planning in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:759-982`.
- Emit only the existing `ArtifactBootstrap`, chunk, and done frames; use the
  policy above; pin baseline/tail under the read barrier; buffer live events
  before selection; release only after exact tail and drain.
- Replace both silent `Lagged(_) => {}` arms at lines 961 and 1361 with explicit
  resync-close behavior. Apply auth revalidation to member/session/public/share
  access rather than only the existing share-token period.
- Tests/oracles: inline/chunk boundary at 4,096/4,097; hash/chunk/order/budget
  abuse; receiver cancellation/deadline; no command gap across selection;
  forced lag/reconnect returns checkpoint plus exact tail; revoked transfer
  leaks no bytes; native/wasm/browser P3 fixtures install atomically.

### P2-D — Retention, compaction, and recovery

- Integrate a public checkpoint reference with the DB sync/compaction seam in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs` and its storage
  layer, while leaving `📸️snapshot` page retention internal. A DB floor can
  advance only after a matching public checkpoint is durable.
- Tests/oracles: crash between every publication step; no tail is pruned before
  a checkpoint exists; reconnect below floor selects the retained checkpoint;
  restart replays authority and DB marker consistently; public bootstrap never
  contains a DB `.spk` page fixture.

### P2-E — Hub workspace and inference consumers

- Add an authenticated descriptor/checkpoint read adapter to
  `🌉️mcp/🏠️workspace/🦀️.rs` and the directory client, so a cold hub workspace
  opens a real declared document rather than `os.agent.probe/v1`; propagate the
  hub bearer/session identity instead of locally asserted scopes.
- Add an authenticated `artifact-infer` route from the hub/authority to the
  existing `ArtifactInferenceRouter` in
  `🔌️plugin/🖥️host/🦀️.rs:5800-6068`. Supply the authority's pinned checkpoint
  revision, cancellation and progress; preserve the current honest
  `channel.not-wired` error until it is actually available.
- Tests/oracles: MCP reads the same pack/SPR hash as WS; unauthorized/revoked
  MCP cannot discover/read it; a real map artifact inference observes the
  canonical revision, cancels, and rejects stale results.

## Acceptance Evidence Required Before Claiming the Goal Areas

| Area | Required runtime evidence | Cross-backend requirement |
| --- | --- | --- |
| Hub persistence | Seed an announced descriptor and trusted codec, accept operations, publish a public pair, restart the hub, and cold-restore exactly the same pair/frontier with no client checkpoint input and no `.spk` public frame. | SQLite always; PostgreSQL and Neo4j each run the same event/projection/restart oracle against a real configured service. |
| Short outage | Interrupt mid-inline/mid-chunk/mid-tail and while receiver lags; existing local artifact remains intact, reconnect chooses checkpoint plus exact tail, and only P3 atomic completion changes local state. | Repeat DB/directory-backed selection under each backend. |
| Collaboration | Two participants and two spaces sharing the same document string: authorized edits converge by accepted frontier; colon scope cases do not alias; lag forces rebootstrap; revocation stops live and checkpoint reads. | Run the scope/retention/projection cases across all three directory implementations. |
| Hub workspace | A new MCP hub process lists a descriptor-authorized remote document, reads its canonical pair, restarts/reopens it, and refuses unannounced/revoked/foreign-space access. It must not create a synthetic probe to pass. | Exercise the actual hub HTTP/WS adapter with each configured backend. |
| AI map inference | A declared map plugin is resolved by descriptor package hash, receives the authority-pinned canonical revision through a real route, reports progress, honors cancellation, and rejects stale revision output. `channel.not-wired` remains an explicit non-success until then. | At minimum run a real registered map codec/router case on SQLite; run the same when Postgres/Neo4j infrastructure is available. |

Unavailable PostgreSQL/Neo4j infrastructure is **not a green skip**. The test
runner must report it as `UNAVAILABLE`/not accepted with the missing endpoint
or credential, while SQLite-only evidence is labelled SQLite-only. No result
may call the end-to-end area passing until the requested backend evidence has
actually run and emitted its runtime assertions.

## Dependency Graph

```text
immutable DocumentDescriptor + trusted package catalog
                         |
                         v
          headless codec/materializer + scope actor
                         |
                         v
           staged validated (pack, spr) pair at frontier B
                         |
                         v
Directory CheckpointPublished -> active checkpoint projection -> WS Hello plan
                         |                                      |
                         v                                      v
              DB retention marker/floor          ArtifactBootstrap + exact B..T tail
                         |                                      |
                         v                                      v
                conservative compaction             P3 atomic client install
```

P2-A is the first unblocker. P2-B and the codec/pinned-catalog implementation
can proceed in parallel after its schema/port contract is fixed; P2-C requires
both. P2-D requires an actually durable P2-B checkpoint, and P2-E requires
P2-C's authorized producer/read surface.
