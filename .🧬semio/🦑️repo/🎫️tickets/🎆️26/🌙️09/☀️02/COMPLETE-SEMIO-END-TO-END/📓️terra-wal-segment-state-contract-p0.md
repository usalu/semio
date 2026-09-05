# WAL Segment State Contract — Implementation Map

## Decision

Add a small, read-only storage query:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalSegmentState { Active, Sealed }

async fn segment_state(
    &self,
    document: &ArtifactId,
    index: u64,
) -> Result<WalSegmentState, DbError>;
```

`Active` means that this exact logical segment existed and was observed not sealed; `Sealed` means it existed and was observed sealed. It is an observation, not authority to append after the return: the caller's existing document ownership/fence must still serialize recovery with another writer. It must return `DbError::NotFound` when the segment does not exist, make no write, allocate no page owner, and never infer state by attempting `append`, `truncate_tail`, or `seal`.

This is the minimum correct seam for the rotation crash window: `ArtifactWal::rotate` fsyncs and seals the old segment before it creates the successor (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1768`). On restart, a sealed highest segment requires creation of the next index, while an active highest segment can use the new verified-SPR resume path. `list_segments` plus a destructive probe cannot distinguish those cases.

The existing trait wording says exactly one segment is active (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4562-4565`). Amend it to allow the transient recovery state “all existing segments are sealed”; `ArtifactWal::open`, while holding its document authority, repairs it by creating the successor. Do not make `segment_state` invent an active segment.

## Typed-lane plumbing

| Surface | Exact current location | Required delta |
|---|---|---|
| Public type and trait | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4561-4605` | Put `WalSegmentState` beside `WalStorage`; add the method after `segment_len` (or after `seal`). |
| Task/result taxonomy | same file `2010-2057` | Add `DbIoTask::WalState { backend, document, index }` and scalar `DbIoResult::WalSegmentState(WalSegmentState)`. |
| Task ownership cleanup | same file `2130-2302` | Add `WalState` to `backend`, `close_step`, and `terminal_is_empty` document-only arms. It needs neither pages nor list backing. |
| Result cleanup | same file `2306-2351` | Add the scalar result to `close_step`, `terminal_is_empty`, and `attach_result_handback` no-owner arms. |
| Memory executor/facade | same file `5830-5968`, `6337-6392` | Read `MemWalSegment.sealed` (`5429-5434`) under the existing WAL lock; missing owner is `NotFound`; wrap/unwrap the new task/result. |
| Filesystem executor/facade | same file `6548-6570`, `6607-6613`, `6835-6923`, `7242-7283` | Stat the `.bin` then the `.sealed` marker as detailed below; route via the existing worker-lane facade. |
| Facet enum dispatch | same file `4872-5022` | Add the one `WalRef::segment_state` match, including the boxed `Fault` arm and all current feature gates. `DbBackend::wal` already selects this facet (`4753-4768`). |
| SQLite | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:18-25,225-306,633-671` | One row query from the existing `wal_segment.sealed` column, executor task arm, typed facade decoder. |
| PostgreSQL | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs:34-40,211-320,680-718,915-949` | One non-locking `SELECT sealed`, async-native task arm, facade decoder. |
| Neo4j | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs:221-248,393-531,842-882,1068-1102` | Add a small state Cypher constant returning only `n.sealed`; task arm and facade decoder. |
| Fault wrapper | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️.rs:331-388` | Direct delegation only. It is an observation, not a scripted write and must not touch `append_calls` or `sync_delegated_calls`. |

The result type is deliberately a typed scalar rather than `Exists(bool)` or an overloaded integer: absence needs to remain `NotFound`, and both recovery branches must be explicit at the call site.

## Exact backend representation and implementation

| Backend | Current sealed representation | Minimal `segment_state` read | Missing/error behavior |
|---|---|---|---|
| Memory | `MemWalSegment { sealed: bool }` at `storage/🦀️.rs:5429-5434`; `seal` flips it at `5865-5869`. | Find the existing `(document,index)` owner under `self.wal`, map `false/true` to `Active/Sealed`. | The same missing-owner path as append/length (`5851`, `5917-5920`): `DbError::NotFound`. |
| FS | `wal/<document>/segment-<20-digit-index>.bin` plus `segment-<20-digit-index>.sealed`; builders are `storage/🦀️.rs:6607-6613`; `seal` creates the marker at `6879-6885`. | `metadata(bin).map_err(|e| open_err(e, ...))?`; then `metadata(marker)`: `Ok(_) => Sealed`, `Err(e) if e.kind()==NotFound => Active`, any other error through `io_err`. | A missing binary is `NotFound` even if a stale marker remains: `list_segments` defines existence by `.bin` (`6896-6901`). Permission, sharing violation, malformed path, I/O, and directory failure must be `DbError::Io`, never silently `Active`. |
| SQLite | `wal_segment.sealed INTEGER NOT NULL DEFAULT 0` (`storage/🪶️sqlite/🦀️.rs:18-25`); create writes 0 (`225-231`), seal writes 1 (`255-261`). | `SELECT sealed FROM wal_segment WHERE document=?1 AND segment_index=?2`, `.optional()`, decode `0 => Active`, `1 => Sealed`. | `None => NotFound`; any other integer is `DbError::Corrupt`. The current writes only generate 0/1 but schema lacks a CHECK, so do not silently call arbitrary nonzero/negative values active as append presently does at `239-244`. Reuse one decoder in the new read and existing append/truncate paths if that malformed-row rule is adopted. |
| PostgreSQL | `db_wal_segment.sealed BOOLEAN NOT NULL DEFAULT FALSE` (`storage/🐘️postgres/🦀️.rs:34-40`); seal returns `TRUE` (`263-268`). | `SELECT sealed FROM db_wal_segment WHERE document_id=$1 AND segment_index=$2`, `fetch_optional(&self.pool)`, boolean mapping. Do **not** use `FOR UPDATE`: it is an observational recovery read, unlike mutation guards at `229` and `306`. | `None => NotFound`; database failures retain `map_sqlx_error`. The current boolean schema cannot encode a third state. |
| Neo4j | `WalSegment.n.sealed`: created false (`CYPHER_WAL_CREATE_SEGMENT`, `222-226`), sealed true (`CYPHER_WAL_SEAL`, `236-239`). | Add `CYPHER_WAL_STATE = "MATCH ... RETURN n.sealed AS sealed"`; `fetch_one(query(...))`, `row.get::<bool>("sealed")`, map the bool. It avoids fetching retained bytes/length as `CYPHER_WAL_READ_ROW` does (`228-230`, `474-479`). | No row is `NotFound`; driver errors use `map_neo4rs_error`; an ill-typed property uses `map_de_error` (`Corrupt`). |
| Fault | No separate state; delegates `self.inner.wal().await`. | `self.inner.wal().await.segment_state(document,index).await`. | Preserve inner errors exactly; no injected failure/counter. |
| `WalRef` enum | Variants are Memory, feature-gated FS/SQLite/Postgres/Neo4j, Fault (`storage/🦀️.rs:4874-4885`). | Same match style as `segment_len` (`4963-4976`). | All feature guards must match the enum declaration; Fault needs `Box::pin` as its recursive facade calls do. |

### Filesystem cross-platform rule

Use `std::fs::metadata`, not `Path::exists()`. Current WAL append/truncate and seal use `exists()` (`6854-6858`, `6879-6884`, `6903-6909`), but that API collapses access-denied, transient I/O, and missing into `false`; that is unacceptable for restart state selection. The new query can be correct independently:

1. Build paths only through `document_dir("wal", document)` and `segment_path`/`sealed_marker_path`, retaining the existing cross-platform path-component validation (`6572-6584`).
2. `metadata(bin)` through `open_err` gives portable `NotFound` only for `ErrorKind::NotFound`, and `DbError::Io` for every other OS failure (`6560-6569`).
3. Only after the binary exists, `metadata(marker)` treats exactly `ErrorKind::NotFound` as `Active`; any other error is `io_err` (`6552-6558`).

The marker is a state marker, not a durability receipt. Current `WalSeal` creates it but does not sync the containing directory (`6879-6885`), so a restart recovery must use the state observed after reopening and retained record verification; it must not claim that a pre-crash marker was durable merely because `seal` returned before a power loss.

## Test registration

No cold build was run for this read-only audit. These are the smallest tests to add before the WAL-open caller consumes the new API.

1. Extend the shared `exercise_wal_storage` law at `storage/🦀️.rs:9009-9042`: immediately after create assert `Active`, after `seal(0)` assert `Sealed`, assert `segment_state(document,99) -> NotFound`, and confirm state calls leave length and bytes unchanged. Its existing registrations cover Memory (`9044-9047`) and native FS (`9049-9053`). This is the durable language-agnostic storage law.
2. Extend the `DbBackend` facet tests at `storage/🦀️.rs:9222-9249` so a call through `storage.wal()` observes `Active` then `Sealed`; this is the `WalRef` dispatch law. Keep the FS assertion under its current `feature = "fs"` gate.
3. Add a SQLite in-memory lane test beside `typed_lane_is_lossless_at_page_boundary_and_zero` (`storage/🪶️sqlite/🦀️.rs:858-869`): create, assert active, seal, assert sealed, and query a missing index. It exercises the registered executor rather than raw SQL.
4. Add `fault_storage_segment_state_delegates_without_write_accounting` beside the fault tests (`db/🧪️testkit/🦀️.rs:1115-1169`): use `DbBackend::Fault`, observe active/sealed, and assert both counters are unchanged. This catches an accidental fault-script write path.
5. Add backend query-shape/mapping laws for unavailable live services: PostgreSQL has only non-live pure tests (`storage/🐘️postgres/🦀️.rs:1092-1096`), so factor a boolean mapper or a narrow query constant and test `false/true`, plus no `FOR UPDATE`/no `bytes` selection. Neo4j's existing Cypher inspection law (`storage/🌐️neo4j/🦀️.rs:1368-1374`) should include `CYPHER_WAL_STATE`, require `WalSegment`, `sealed`, and reject `bytes`; test both boolean outcomes in a pure mapper. A future live integration test can additionally check database round-trip, but neither current module has a live service fixture.

## Recovery consumer boundary

`ArtifactWal::open` should first use the retained full-segment verifier and `WalSegmentChain`, then query the highest segment's state while still holding the document lease. The cases are:

| Recovered segment set | Highest state | Correct next action |
|---|---|---|
| Empty | n/a | Create index 0 with the normal fresh header path. |
| Valid highest with an uncommitted/torn tail | Active | Copy the verified prefix, call `truncate_tail` only to that verified end, then resume the writer. |
| Valid highest exactly at a committed boundary | Active | Copy exactly the verified prefix and call `SprWriter::resume_verified`; do not rewrite/delete the segment. |
| Valid highest sealed, including the rotate crash gap | Sealed | Leave it immutable; create `highest + 1` and begin a successor chained to the verified tip. |

`highest.checked_add(1)` must be fallible; a sealed `u64::MAX` is a limit error, not a wrap to index zero. This state API supplies only the final branch discriminator. It does not replace full-SPR validation, `WalSegmentChain`, storage authority, tail truncation, or the resumed writer's `sink.position()==span.end()` guard.

## Scope and current-source check

This report is read-only. At inspection, no `WalSegmentState`, `WalState`, or `segment_state` symbol existed under the OS DB module, and the above is based on the current sources rather than the earlier destructive-reopen audit. No runtime, source, fixture, or build command was changed or run.
