# W3b — `HubInstance: ServerInstance` + durable storage profiles

Slice W3b, executing **§B.7 steps 4–5** of `📓️h2-server-crate-and-wave3-memo.md` on top of W3a's
`ServerInstance` seams (`📓️w3a-server-instance-seams.md`). All commands foreground, from
`/Users/ueli/Documents/semio`, no sub-agents, no git-modifying command, no worktree.
Captures: `🗑️generated/w3b-*.txt`.

---

## 1. Inherited state (measured)

`git status --short` on `🌎️hub` and `🧰️framework/🛍️products/🖥️server` at slice start showed **no
partial W3b work**: nothing under `🌎️hub/🗄️stores`, no `semio-framework-server` entry in hub's
manifest, and `🗑️generated/` empty (a peer's `clean` had swept it — the previous worker's captures
are gone). The framework side carried W3a's finished step 3 only. So step 4 started from zero.

Two baselines were measured before any edit:

| gate | result | capture |
|---|---|---|
| `cargo check -p semio-hub --lib` (default features) | **red, not mine**: `semio-s-artifact-stdio-semio` fails with 4 errors (`PdfSnapshot` missing 25 fields, `PdfPage::text` field-vs-method) — it arrives through hub's default `native-artifact-execution` feature | `w3b-baseline-hub-check.txt` |
| `cargo check -p semio-hub --lib --no-default-features --features sqlite` | **green**, 37 s | `w3b-baseline-hub-check-nodefault.txt` |

Everything below is therefore gated on the `--no-default-features --features sqlite` lane. The PDF
breakage is another slice's (`📓️project-draw-pdf-export-path` territory) and was red before this
slice touched anything.

## 2. `StorageProfile` gets a second, real variant

`🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs:45-82` (region `🔖️Profile`):

```rust
pub enum StorageProfile {
    Ephemeral,
    Embedded { data_dir: String },
}

impl StorageProfile {
    pub fn data_dir(&self) -> Option<&str>
    pub fn is_durable(&self) -> bool
}
```

Two variants and not three, because two are *implemented* — inventing profile names no backend
answers is exactly what W3a spent its slice undoing. `data_dir()` exists so no caller has to `match`:
`ServerBuilder::build` used to destructure the one-variant enum irrefutably
(`let StorageProfile::Embedded { data_dir } = &self.profile;`), which a second variant turns into a
compile error at the one place that must not care.

Three consequential edits followed in `📡️gateway/🦀️.rs`:

- `:686` — `ServerState::data_dir: Arc<PathBuf>` became **`profile: Arc<StorageProfile>`**. A handler
  that asks "where is my durable state" must be able to get the honest answer *nowhere*;
  `Arc<PathBuf>` could only have answered with a fabricated empty path.
- `:1352` — the irrefutable `let` is gone; `build` no longer reads the profile at all beyond handing
  it to `I::open`.
- `:1381` — state now carries `Arc::new(self.profile.clone())`.

`#[serde(rename_all_fields = "camelCase")]` was added alongside the existing `rename_all` on the
enum: an internally-tagged enum's `rename_all` renames the **variants**, not the fields inside them,
so `dataDir` was serializing as `data_dir`. Caught by the new test, not by reading
(`w3b-fw-test-2.txt`).

## 3. `HubInstance`

New file `🌎️hub/🗄️stores/🦀️.rs` (742 lines incl. the test hook), mounted at
`🌎️hub/📦️packages/🦀️rust/🦀️.rs:33-35` as `pub mod stores;`. Hub's manifest gains
`semio-framework-server = { workspace = true }` (`🌎️hub/📦️packages/🦀️rust/Cargo.toml`, next to the
`db` entry) — the dependency the memo's §B.4 measured as *absent* and the plan wrongly assumed
present. The crate's lib is named `server`, so hub writes `use server::gateway::ServerInstance;`.

All ten ports are named (`🗄️stores/🦀️.rs:59-87`):

| associated type | hub's answer | state |
|---|---|---|
| `AuthorityStore` | `HubAuthorityStore` | **real**, durable |
| `ProjectionStore` | `HubProjectionStore` | **real**, durable |
| `BlobStore` | `HubBlobStore` | **real**, durable |
| `SessionStore` | `HubSessionStore` | **real**, durable |
| `Queries` | `server::gateway::NoQueryHandler` | framework's explicit "none" |
| `Documents` | `server::gateway::NoDocumentAuthority` | framework's explicit "none" — the document WS stays `notFound` by construction until §B.7 step 6 |
| `Modules` | `HubModules` (uninhabited) | none registered yet |
| `Deciders` | `HubDeciders` (uninhabited) | none registered yet |
| `Sagas` | `HubSagas` (uninhabited) | none registered yet |
| `Resolvers` | `HubResolvers` (uninhabited) | empty ladder ⇒ anonymous principal |

The four uninhabited enums are the same shape the framework itself ships for `Documents`/`Queries`:
`match *self {}` bodies, so they are statements ("hub registers none of these yet"), not stubs with
fake behaviour, and each becomes an enum over real variants when §B.7 steps 8–10 move a subsystem.
No `dyn_enum_close!` was needed anywhere in hub — every set is either a single concrete type or
empty, which is precisely the shape W3a said needs no enum at all.

`HubInstance::open` (`🗄️stores/🦀️.rs:74-86`) is the one method step 4 had to write: `Ephemeral`
returns four sinkless stores, `Embedded { data_dir }` creates the directory and opens
`authority/`, `projections/`, `blobs/`, `sessions/` under it.

## 4. Durable stores

**The memo said "over `db::Database`/`SqliteDirectory`"; that was not done, and the reason is
measured, not stylistic.** `db` is a document-oriented engine: its storage seam
(`🛢️db/🗄️storage/🦀️.rs:1-48`) is `WalStorage`/`SnapshotStorage`/`PayloadStorage`/`CatalogStorage`/
`IndexStorage`/`LeaseStorage` keyed by *document*, its blocking backends may only be constructed
against the one process-wide `WorkerPool`, and its facade is an admission/generation/terminal
ceremony (`⚙️engine/🦀️.rs`, `DatabaseCapabilityOpen*`, `DatabaseCatalogRead*`). It has no
`ActorKey`-keyed event stream, no idempotency inbox and no transactional outbox — the three things
`AuthorityStore` *is*. Mapping four CQRS roles onto it is a translation layer between two unrelated
vocabularies, which this repo forbids by name. Hub owns its own on-disk format instead, in ~500
lines, and swapping it onto `db` later is a change behind four unchanged traits.

**Shape.** One append-only JSON-line journal per journalled role, folded into memory on open:

```
<data_dir>/authority/log.jsonl      AuthorityRecord: receiptRecorded | eventsAppended |
                                    snapshotPut | outboxEnqueued | outboxDelivered | leaseAcquired
<data_dir>/projections/log.jsonl    ProjectionRecord: put | checkpointed | cleared
<data_dir>/blobs/<64-hex-hash>      immutable content-addressed bytes, one file per hash
<data_dir>/sessions/<hex-id>.json   one file per live session
```

Five decisions worth naming:

1. **Journal-then-fold, never the reverse** (`🗄️stores/🦀️.rs:428` `HubAuthorityStore::commit`). The
   record is written and `sync_data()`d *before* it touches memory, so a process that dies between
   the two replays the fact instead of losing it. Validation (sequence contiguity, receipt
   re-binding, backwards snapshot) happens before the write; `fold` never re-judges, because a
   replay must reproduce history rather than re-decide it.
2. **Outbox ids are stamped before journalling** (`:404` `stamped`), so a replayed queue carries the
   identical positions a publisher may already have acknowledged. The restart test asserts exactly
   this.
3. **One code path for both profiles.** `Journal::sink` is `Option<File>`; `Ephemeral` gives `None`
   and every fold still runs. There is no second "memory backend" type to drift from the durable
   one — which is the drift the conformance suite exists to catch.
4. **A torn tail is dropped, not fatal** (`:164-193` `Journal::replay`). A process killed mid-append
   leaves an unterminated final line; replay stops at the last complete parseable record and the
   file is truncated back to that byte. A record that was never fsynced is a fact that never
   happened.
5. **Sessions are deliberately *not* event-sourced** (`:651-660`). The storage contract requires that
   a revoked session leave no replayable trace, which an append-only journal cannot promise, so a
   session is one file that exists or does not. This is the one role where CQRS/event-sourcing is the
   wrong answer and the contract says so in its own docstring.

Projection journals are **compacted at open** (`:479` / `:531` `compacted`): after the fold, the file
is rewritten as one `put` per live entry plus one `checkpointed` per projection, so a hot key does
not cost a line forever. Legitimate precisely because a projection is rebuildable by definition.

## 5. Conformance suite across instances + restart durability

The framework's ten storage tests were **not copied** into hub — copying a contract produces two
contracts that drift. They were lifted into one backend-generic suite:

- New `🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️conformance/🦀️.rs` (179 lines): ten
  `pub async fn <law>(store: &mut impl <Role>Store)` bodies plus four fixture constructors
  (`actor_key`, `event_record`, `receipt_for`, `session_record`).
- Gated by a new `conformance` Cargo feature (`📦️packages/🦀️rust/Cargo.toml`) and mounted
  `#[cfg(any(test, feature = "conformance"))]` (`📦️packages/🦀️rust/🦀️.rs`). A default build of the
  product contains none of it — W3a's rule that fixtures are not product surface holds.
- `🔨️modules/🗄️storage/🧪️tests/🔬️unit/🦀️.rs` now opens its stores through
  `TestInstance::open(&StorageProfile::Ephemeral)` and calls the suite, so the framework's ten laws
  judge **`TestInstance` itself**, not four types that happen to sit next to it.
- Hub declares the feature on its **dev**-dependency only, and
  `🌎️hub/🗄️stores/🧪️tests/🔬️unit/🦀️.rs` runs each of the ten laws **twice** — once on
  `HubInstance::open(Ephemeral)` and once on `HubInstance::open(Embedded)` against a fresh temp
  directory.

Net: each of the ten storage laws now executes **three** times per full run (TestInstance,
HubInstance ephemeral, HubInstance embedded) from **one** body.

Eight further hub laws were added on top of the ten:

| law | what it pins |
|---|---|
| `the_profile_decides_whether_a_store_is_durable_at_all` | all four roles report durability from the profile; the four on-disk paths exist |
| `an_ephemeral_instance_keeps_nothing_for_the_next_open` | a second `Ephemeral` open starts at frontier 0 |
| `a_durable_authority_reopens_at_the_frontier_it_was_left_at` | **the restart law**: write events/receipt/snapshot/lease/outbox → drop → reopen → same frontier per actor, same events byte-for-byte, receipt, snapshot and lease intact; a stale `seq` is still a `SequenceGap` after reopen and the stream continues at `seq+1` |
| `a_durable_outbox_reopens_with_its_acknowledgements_intact` | delivered rows stay delivered across restart and the id cursor continues past them |
| `durable_projections_blobs_and_sessions_survive_a_restart` | last-write-wins per key, checkpoints, cleared projections stay cleared, blob bytes, live/deleted sessions, and revocation surviving a further reopen |
| `a_projection_journal_is_compacted_to_its_current_state_on_open` | 17 lines written → 2 lines after reopen, same values |
| `a_torn_final_record_is_dropped_rather_than_failing_the_open` | a truncated final line is discarded, the file is repaired to the last good byte, and appending resumes |
| `the_hub_instance_builds_a_server_over_its_durable_stores` | `Server::<HubInstance>::builder(Embedded).build()` succeeds, reports its identity, exposes the profile, and its `ServerState` stores are the durable ones |

## 6. Gates run

| gate | result | capture |
|---|---|---|
| `cargo test -p semio-framework-server --lib` (W3a baseline was 77) | **78 passed, 0 failed** | `w3b-fw-test-final-lib.txt` |
| `cargo check -p semio-framework-server` (no features) | green — the conformance module is absent from a default build | `w3b-fw-check-default.txt` |
| `cargo check -p semio-framework-server --features conformance` | green | `w3b-fw-check-conformance.txt` |
| `cargo check -p semio-hub --lib --no-default-features --features sqlite` | green | `w3b-hub-check-1.txt` |
| `cargo check -p semio-hub --bins --no-default-features --features sqlite` | green (`os-hub` still builds) | `w3b-hub-bin-check.txt` |
| `cargo test -p semio-hub --lib … stores::` | **18 passed, 0 failed**, 118 filtered out | `w3b-hub-test-1.txt` |
| `cargo test -p semio-hub --lib …` (whole suite, `SEMIO_TEST_ARTIFACT_DIR` set) | **136 passed, 0 failed** = 118 pre-existing + 18 new | `w3b-hub-test-full-env.txt` |
| same, re-run at slice end (peers had added 15 laws meanwhile) | **151 passed, 0 failed** | `w3b-hub-test-final.txt` |

`SEMIO_TEST_ARTIFACT_DIR` is an environment contract the nx test target supplies
(`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🧪️tests/🔬️publication/🦀️.rs:19`
`expect("ticket-owned artifact root")`). Without it 23 pre-existing hub laws fail on the missing
variable, none of them in `stores::` (`w3b-hub-test-full.txt` lists all 23). With it set, the whole
suite is green.

## 7. Honest gaps

- **Two transient peer breakages were observed and are gone.** Mid-slice,
  `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1814` briefly passed a `MutexGuard<Connection>` where a
  `Connection` was expected (`w3b-hub-check-2.txt`); it was fixed by its owner and the final hub run
  is green. Still red at slice end, and **not mine**: slice D1's new integration target
  `🧰️framework/🛍️products/🖥️server/🧪️tests/🔒️closed-ports/🦀️.rs` fails to compile (4 errors —
  `ActorKey` missing `tenant`, two `Result` equality comparisons, `QueryConsistency::Eventual`), so
  `cargo test -p semio-framework-server` without `--lib` cannot run. The framework's 78 lib laws are
  green (`w3b-fw-test-final-lib.txt`).
- **Verified by tests only, not at runtime.** No `os-hub` process was booted against a
  `HubInstance`-built server, and `Server::run` is still unexercised — hub's binary still constructs
  its own axum app (§B.7 step 9 shrinks `🏗️bootstrap`, not this slice). "Survives restart" is proven
  by dropping and reopening the stores in-process, which exercises fsync, replay and truncation, but
  not a real SIGKILL.
- **Default-feature hub is red for an unrelated reason** (`semio-s-artifact-stdio-semio`, §1). Every
  gate here is on the `--no-default-features --features sqlite` lane.
- **Journal encoding is JSON, so a `Vec<u8>` payload is a JSON number array.** Correct, portable and
  inspectable, but roughly 4× the bytes of a binary framing. Every write is fsync-bound, so the cost
  is not on the hot path; a binary framing is a swap behind `Journal`.
- **`ProjectionStore`/`SessionStore` writes return `()`**, so a journal failure cannot be reported to
  the caller. Both stores count it (`journal_faults()`) and the projection store drops its sink
  rather than pretending; a projection is rebuildable so the repair is a replay. The fault path is
  **not** covered by a test — the `Result`-returning `open` path is. Making those two trait methods
  return `Result` is a framework change with handler-side ripple, worth its own slice.
- **`ServerSagas<I>` is still not wired into `ServerState`** (W3a's gap, unchanged here).
- **`🧰️framework/🛍️products/🖥️server/🟦️.ts` is still `export {};`** (§B.5 item 10, untouched).
- **Hub's own `db::Database`/`SqliteDirectory` state is untouched.** Hub now has *two* durable
  substrates: its existing directory/artifact storage and these four server roles. That is not
  duplication yet — nothing writes to both — but it becomes duplication the moment §B.7 steps 6–10
  move a subsystem, and those steps must move the data, not mirror it.

## 8. Files changed

```
A  🌎️hub/🗄️stores/🦀️.rs                                              (742)
A  🌎️hub/🗄️stores/🧪️tests/🔬️unit/🦀️.rs                                (259)
A  🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️conformance/🦀️.rs          (179)
M  🌎️hub/📦️packages/🦀️rust/Cargo.toml                                (+8)
M  🌎️hub/📦️packages/🦀️rust/🦀️.rs                                     (+5)
M  🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/Cargo.toml        (+7)
M  🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/🦀️.rs             (+8)
M  🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs            (profile)
M  🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🧪️tests/🔬️unit/🦀️.rs
M  🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs            (3 sites)
M  🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🧪️tests/🔬️unit/🦀️.rs (1 line)
```

No nx target or `launch.json` row was added: this slice introduces no new runnable command — the
hub and server `test` targets already cover the new laws. `🌎️hub/🗄️stores` owns no `🧬️schema`, so it
needs no entry in `🔣️schema-catalog.json` (same as `🚀️local-relay` and `🤝️integration-harness`).
Other diffs visible in `🎭️authority`, `🛡️policy`, `📡️gateway` and `🧪️tests/🧩️instance` during this
slice belong to slice D1's concurrent `#[dyn_enum]` work, not to W3b.
