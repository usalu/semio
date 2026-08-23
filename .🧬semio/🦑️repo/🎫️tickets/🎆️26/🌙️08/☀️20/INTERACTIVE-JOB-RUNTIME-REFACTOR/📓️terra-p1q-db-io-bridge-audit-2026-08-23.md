# Terra Audit — Phase 1q DB I/O Bridge — 2026-08-23

## Verdict

**REJECT — source packet only. Phase 1 remains RED.**

The retained bridge itself has the claimed bounded-admission and owner-retention shape, but the active product call chain still has two synchronous filesystem operations outside it. This prevents accepting the claim that UI/product-reachable FS/SQLite setup is wholly on the injected process `WorkerPool` I/O lane.

### Blocking evidence

`🌎️hub/📦️packages/🦀️rust/📦️bin.rs` has an `async fn connect_db` which does the following before reaching the strong-pool constructors:

- FS branch, line 1622: `std::fs::create_dir_all(&root)?`, immediately followed by `Database::open_at(pool, &root, profile).await`.
- SQLite branch, line 1629: `std::fs::create_dir_all(parent)?`, immediately followed by `SqliteStorage::open(pool.clone(), ...).await`.

Those are caller-thread blocking operations on the live Hub path, not retained `Lane::Io` work. They are also redundant: `FsStorage::open` performs its root `create_dir_all` inside `run_blocking_op` at storage lines 2017–2022, and `SqliteStorage::open` performs its parent creation in its own retained operation at SQLite lines 160–173. The adjacent Hub comment claiming that SQLite storage calls no longer stall the caller's Tokio worker is therefore too broad.

Required repair before a new audit: remove those redundant Hub pre-creates (leaving the storage constructors as the sole bootstrap authorities), then prove the live Hub FS and SQLite branches reach only the pool-backed constructors. Correct the stale DB documentation at storage lines 1836–1838 and 2011–2016 as part of the same repair: it still says `HostAsyncRuntime`, a `pool is None` inline route, and a synchronous open mkdir, while the current implementation has none of those behaviours.

## Independent findings

Snapshot inspected: `HEAD` `9d7cabfd9c`, plus the concurrent working/index state at audit time. This report attributes only the DB-bridge packet; unrelated Phase 3/8 edits were present and were not modified.

| Gate | Independent result |
| --- | --- |
| Retained-call census | **PASS.** `rg` counted 30 authored FS submissions before its test module and 30 SQLite submissions: 60 total = 1 FS open + 2 SQLite opens + 57 storage-trait calls. Every counted submission calls `run_blocking_op(... Lane::Io ...)`. |
| Strong pool construction | **PARTIAL / blocker above.** `Database::{open,open_at}`, `FsStorage::open`, and `SqliteStorage::{open,open_in_memory}` require `Arc<WorkerPool>`; CLI and Hub pass it. No authored DB code match for `Option<Arc<WorkerPool>>`, `None => work/job`, `open_inline`, or DB `.with_pool`. The two Hub pre-creates nevertheless bypass the lane before those constructors. |
| Input ownership and limits | **PASS.** `DbIoPages` takes a `Vec` without copy and returns it on constructor rejection. It uses 16 KiB pages, 496 KiB maximum input/blob, 64 operation slots, 64 pages/1 MiB per operation, 1024 pages/16 MiB aggregate, and a 4096-item list ceiling with nested-byte preflight. All five trait write APIs (`append`, `write_generation`, `put`, `cas_root`, `write_run`) take `DbIoPages`, not caller slices or `Vec`s. |
| Exact rejection/terminal ownership | **PASS by source inspection, not runtime claim.** `DbIoAdmission` checks slot generation and byte count before release; `DbIoState` retains work, rejected job, retry job, terminal work, and terminal result. Saturation uses `error.into_job()` plus one generation-keyed `callback_at` retry. `drive_one` checks generation before clearing `scheduled` or taking work; retrieval APIs and `close_step` expose/release one terminal owner at a time. |
| Fixtures and verifier | **PASS as structural coverage.** The seven DB fixtures exercise missing pool, item/byte cap plus one, exact rejected page owner, cancellation before and during execution, stable post-completion cancellation, stale generation, and shutdown terminal job take/resume/one-owner close. `interactivityDbIoSelfTests` mutates every corresponding required source shape; the command below ran those mutations. This is not a Cargo/runtime execution result. |
| Testkit ownership | **PASS.** The public replay law accepts an injected `Arc<WorkerPool>`; its test-only entrypoint obtains the process singleton via `process_worker_pool`. `rg WorkerPool::new` returned no DB-testkit match, so it does not construct a subsystem pool. |
| P3/P8 containment | **PASS for this packet.** Its DB production paths do not overlap the P3 browser-worker, ShardExecutor, MCP, store-sync, or Writer source paths. Concurrent P3/P8 files and their `📜️script.ts` verifier work remain outside this conclusion. |

## Required checks run

All commands below were read-only and run from `/Users/ueli/Documents/semio`.

```text
rustfmt --edition 2021 --check --config skip_children=true <13 edited DB Rust sources>
PASS

bun ./📜️script.ts verify interactivity --self-test
PASS — severity=deny; one existing allowlisted renderer process-entry finding, zero unlisted failures. DB structural adversarial self-tests ran within the verifier.

rg census/scans for run_blocking_op, optional/inline paths, testkit WorkerPool::new, and five write signatures
PASS for the retained bridge; the independent Hub `std::fs::create_dir_all` blocker above was found by the wider product-path scan.

git diff --check; git diff --cached --check; git diff HEAD --check
PASS

The same three diff checks scoped to the Hub/DB/📜️script.ts packet paths
PASS
```

No Cargo, Nx, Wasm, browser, network, root lint, or runtime-backend test was run, as required.

## Explicit residuals (not passed off as bounded turns)

- A `run_blocking_op` closure is one indivisible backend opportunity. The fixed credits bound admission and retained ownership, not the duration of a `std::fs` or `rusqlite` call. SQLite event-log replacement remains a later Phase 9 concern.
- FS whole-file reads and SQLite whole-blob results remain whole-result backend allocations under the 496 KiB/operation admission caps; SQLite can perform length preflight plus blob fetch inside one retained closure.
- `SharedBuf::snapshot()` still materializes the WAL snapshot before `DbIoPages::try_range`; range submission itself avoids a suffix copy. `DbIoPages::into_vec` can copy a nonzero-start range in non-FS/SQLite consumers.
- `db_engine::ArtifactHandle` retains `db_actor::block_on` inside its WorkerPool job. That is not accepted as P1 runtime evidence.
- Generated `compose/server/hub/rs/bin.rs` still calls the former pool-less `Database::open_at(&root, profile)` and is an unreconciled generated-Compose blocker.
- The Hub direct pre-creates documented under the verdict are additional active product residuals, not the allowed backend-indivisibility residual; they are why this packet is rejected.

The wider Phase 1 compilation, runtime timing/fairness, cancellation-under-real-stall, close, database-I/O, and generated-Compose reconciliation gates remain unexecuted or open. Nothing in this audit accepts Phase 1.
