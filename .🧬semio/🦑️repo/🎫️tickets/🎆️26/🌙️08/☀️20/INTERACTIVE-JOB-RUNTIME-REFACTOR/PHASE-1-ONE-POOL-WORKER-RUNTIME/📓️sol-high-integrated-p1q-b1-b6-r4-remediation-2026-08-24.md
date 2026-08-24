# Integrated P1q B1–B6 and R4 Remediation

Date: 2026-08-24  
Executor: Sol High  
Input audit: `📓️terra-fresh-integrated-p1q-b1-b6-r4-acceptance-audit-2026-08-24.md`  
Status: source-audit-ready

## Outcome

Every bounded repair packet from the fresh integrated RED audit is implemented. Actual PostgreSQL and Neo4j driver futures are retained in the generation-qualified DB I/O task slot and polled only by jobs submitted to the caller-supplied shared `WorkerPool` on `Lane::Io`. Post-admission artifact adapters and lease results now retain exact typed owners. Snapshot reads identity chunks incrementally. Index, WAL, artifact, query, engine, compaction, and CLI close paths advance one retained owner/page opportunity and hand unfinished work to mounted fixed retirement authority.

## Exact Files

- `📜️script.ts`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs`

## Owner Ledger

| Owner | Admission identity | Live authority | Terminal handback |
| --- | --- | --- | --- |
| Async SQLx/Neo4j future | task operation plus `db_io_async_lease_credit` | `DbIoTaskSlot::async_driver` | ready future returns the same executor/task/result to `db_io_finish_async_driver`; panic retains the future fail-closed in the exact slot |
| Worker turn | generation-qualified `DbIoTaskHandle` | shared pool `Lane::Io` job | slot wake/schedule flags prevent parallel or facade polling |
| Driver trait `ArtifactId` | `DbIoArtifactId::try_from_text(operation, text)` reserves maximum fixed text capacity before `String` construction and observes actual capacity | `DbIoArtifactId` plus `DbIoDriverReservation` | one close drops the artifact allocation; reservation Drop parks exact credit for mounted handback |
| Lease result | `DbIoLeaseResult` fixed `DbIoText` resource/holder | `LeaseInfo = DbIoLeaseResult` | public `close_step`; unfinished Drop parks `DbIoLostOwner::Lease` |
| Snapshot chunk | manifest `ChunkTableEntry` identity | `PackIdentityChunkCursor` offset, CRC cursor, BLAKE3 cursor | caller-owned fixed fragment buffer; EOF verifies and marks terminal |
| Query rows | fixed `QueryRows` slots | `QUERY_RETIRED_ROWS[64]` | one `query_rows_maintenance_step`; saturation is lossless fail-closed and witnessed by `QUERY_RETIREMENT_SATURATED` |
| Engine query stream | fixed `QueryStream` slots | `ENGINE_QUERY_RETIREMENT[64]` | one `engine_query_maintenance_step`; saturation is lossless fail-closed and witnessed by `ENGINE_QUERY_RETIREMENT_SATURATED` |
| DB pages/lists/results | existing operation/page/result lease | existing fixed lost-owner/task close rings | one mounted maintenance opportunity |

Frozen protocol/wire owning fields were not changed. `SnapshotDescriptor`, WAL text boundaries, and protocol envelopes retain their permitted schema ownership. The rejected raw `LeaseInfo` shape was not treated as a schema exception.

## Close Opportunity Map

| Family | Opportunity | Resume/handback point |
| --- | --- | --- |
| DB async driver | one future poll | waker schedules the same slot on `Lane::Io`; ready returns executor/task/result |
| DB result conversion | one holder/resource/handback step | `DbIoLeaseResult::close_step` or lost-owner ring |
| Snapshot | one pack fragment read or one owner close | `PackIdentityChunkCursor::offset`; unfinished DB owners park in core maintenance |
| Index | one `pages.close_step`, entry value/key step, or list step | remaining owner Drop parks exact page ownership |
| WAL replay | one segment page close | `WalReplayStep::Yield` preserves segment index/pages between calls |
| Artifact/state replay | one record/page/cursor close | retained DB owner Drop and mounted actor cursor |
| Query | one nested value/page/row close | fixed query retirement slot |
| Engine | one path/value/entry close | fixed engine retirement slot |
| Compact/CLI/sync | one record/list/page/cursor close | retained owner handoff; WAL callers recognize `Yield` |

No normal, success, cancellation, stale, fault, or Drop path added by this packet bulk-drains a retained owner graph.

## Hostile Laws

- `db_io_actual_async_driver_future_is_polled_by_the_shared_io_worker`
- `db_io_artifact_and_lease_result_owners_retain_exact_incremental_handback`
- `identity_chunk_cursor_retains_fragment_progress_and_terminal_verification`
- `wal_replay_cancel_resume_close_and_fragment_crc_are_deterministic` now proves two distinct segment-close yields
- `interrupted_query_rows_drop_retains_one_resumable_close_owner`
- `interrupted_query_stream_drop_retains_one_resumable_close_owner`
- Existing corruption, cancellation, retry, ABA, panic, lost-backend, exact page cap, and driver-capacity fixtures remain present.

## Verifier Mutations

The isolated P1q verifier now rejects:

- facade-side actual `drive_task(...).await` restoration;
- loss of the worker-polled async driver fixture;
- uncensused PostgreSQL/Neo4j `ArtifactId(document.as_str().to_string())`;
- raw heap `LeaseInfo` reconstruction and bulk `into_lease_info` conversion;
- snapshot `PackFile::read_chunk` materialization;
- index decode bulk close;
- WAL segment bulk close;
- `QueryRows::Drop` and `QueryStream::Drop` bulk drains;
- artifact/compact/CLI production close sweeps;
- loss of pack, WAL, query, or engine hostile fixtures.

Legacy verifier mutations and tests were retained.

## Validation

- `bun ./📜️script.ts verify interactivity p1q-b1-b6` — PASS: `live-source and hostile mutations clean.`
- Scoped `rustfmt --edition 2021 --config skip_children=true` — PASS.
- Scoped `rustfmt --edition 2021 --check --config skip_children=true` — PASS.
- Scoped `git diff --check` — PASS.

## Residuals and Deferred Gates

- No active same-line P2 Store decode conflict was found. No Store decode region was edited.
- No P5 caller-propagation, P2 job, renderer, stdio, oracle, or peer-owned region was intentionally changed.
- Cargo, Nx, Wasm, browser, and broad runtime/build gates were deliberately not run because other Rust source packets remain active and the coordinator explicitly prohibited those gates for this packet.
- Runtime execution of the new Rust fixtures and feature-matrix compilation therefore remain deferred to the coordinator's quiet-tree validation window. No claim of those deferred gates passing is made here.
