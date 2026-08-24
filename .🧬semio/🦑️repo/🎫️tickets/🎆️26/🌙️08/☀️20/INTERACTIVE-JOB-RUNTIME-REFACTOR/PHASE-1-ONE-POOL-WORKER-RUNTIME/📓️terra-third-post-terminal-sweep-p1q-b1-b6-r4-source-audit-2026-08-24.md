# Third Independent P1q Post-Terminal-Sweep Source Audit

Date: 2026-08-24  
Auditor: Terra (fresh, read-only)  
Disposition: **RED — P1q is not accepted.**

## Scope and Method

Read completely: root `AGENTS.md`; the P1q repair contract; the original/fresh Terra RED reports;
the core remediation; the R4 packet and syscall census; the coordinator residual checkpoint; the
post-Sol RED audit; and `📓️sol-high-p1q-r4-terminal-sweep-remediation-2026-08-24.md`.

This is a live-source audit, not a report review. It inspected storage, memory, PostgreSQL, Neo4j,
snapshot, index, WAL, artifact, query, engine, compaction, CLI, and pack. No implementation source
was edited and no Cargo, Nx, build, runtime, database, or browser command was run.

## Result Summary

The terminal-sweep remediation did replace the earlier artifact `for` drain, CLI record/batch/replay/
snapshot close loops, and dynamic `Vec<Page>` compaction collector. The typed Memory facade now takes
an injected pool, and the Postgres/Neo facades install their async driver future for the `Lane::Io`
job to poll.

Those improvements are insufficient. Exact lossless retirement is still false at every saturated
fixed retirement ring, and production storage still performs whole-owner close/copy loops inside a
single async-driver poll. The new hostile-law names are largely shape checks: their bodies do not
exercise the named cancel/stale/fault/saturation cases. Therefore neither B1–B6 nor R4 can be
accepted as a whole.

## B1–B6 Audit

| Gate | Result | Live evidence |
| --- | --- | --- |
| B1 shared typed I/O lane | **RED** | Ordinary Memory/Postgres/Neo task submission is structurally improved: `MemoryStorage::new(pool)` and `memory_execute` use `submit_db_io_task`; PG/Neo facade `execute` calls `start_async_native_on_lane_io`; `db_io_submit_job` uses `Lane::Io`. But lost backend close is not a lane job: `db_io_backend_maintenance_step` directly calls `db_io_backend_close_step` (`storage:2285-2296`), which invokes `PostgresDbIoExecutor::close_backend_step`; that manually polls `pool.close()` with `Waker::noop()` (`postgres:810-830`). Thus a real external backend-close future remains reachable outside `WorkerPool` `Lane::Io`. |
| B2 actual owner/credit ledger | **RED** | `DbIoArtifactId::try_from_text` reserves/observes capacity (`storage:788-801`), and `LeaseInfo` is `DbIoLeaseResult` (`storage:1685-1739`). But the claimed aggregate witness is falsified when an owner is abandoned at saturation: the relevant page/control credit remains live forever rather than being handed back or represented by an exact public rejection. |
| B3 bounded terminal close | **RED** | `db_io_transfer_list` copies every list item and then executes `while source.close_step()` (`storage:849-865`). `db_io_close_platform` likewise executes `while owner.close_step()?` (`storage:1032-1041`). Both inner `poll_fn` calls return `Poll::Ready`, so they do not yield to a later worker turn; one outer async-driver poll can exhaust the whole owner. This directly contradicts one close opportunity per grant. |
| B4 loss/drop/cancel/ABA handback | **RED** | Page-loss saturation marks a permanent flag and sets `returned = true` without retaining the handle or returning its credit (`storage:434-450`). Platform-loss saturation does the equivalent (`storage:1083-1100`). General lost-owner saturation calls `std::mem::forget(owner)` (`storage:2947-2951`). These are irrecoverable orphan/leak paths, not exact lossless handback. |
| B5 all backend close routes reach terminal-empty | **RED** | Normal PG close first uses a typed `BackendClose`, but a lost `PostgresStorage` only invokes `retire_db_io_backend` (`postgres:875-880`). Its later actual `pool.close()` poll is the off-lane maintenance path above. The close test proves `is_closed` after repeatedly calling generic maintenance; it does not prove one lane-owned close poll per opportunity. |
| B6 hostile laws and mutation coverage | **RED** | The verifier looks mainly for names/tokens. Its B predicate accepts `DB_IO_PERMANENT_RETAINED_FAULT` and does not reject `std::mem::forget` (`📜️script.ts:7840-7855`, `7879-7892`). The new CLI law only performs a normal record/batch close, an ordinary dropped record, a batch capacity refusal, and a fabricated fault witness (`cli:1541-1563`); it does not drive a real cancel, stale, faulting close, replay, snapshot, or migration exit. The artifact law retires one constructed rejection and merely asserts saturation was not reached (`artifact:3622-3637`); the compaction law likewise retires one rejection then asserts its saturation flag is false (`compact:709-724`). Neither proves its title's saturation/cancel/stale/fault claims. |

## R4 Retained Streaming and Terminal Audit

### Preserved improvements

- Pack exports `PackIdentityChunkCursor`; snapshot `read_page` uses the fragment reader.
- WAL's `next_step` yields when `close_segment_step` consumes an opportunity.
- Index exposes one-step retained close for entries/blob lists.
- Artifact has `ArtifactStateRetirementCursor`; compaction has a fixed `[Option<Page>; 64]` owner
  collection and calls `publish_retained` with that fixed collection.
- CLI has mounted record, batch, replay, and snapshot close futures; the previous direct
  `while`/`loop` terminal drains in its named command paths are absent.

### Blocking R4 counterexamples

1. **Platform work is not actually resumable.** `db_io_write_observed_bytes` iterates all driver
   bytes (`storage:815-837`), `db_io_prepare_platform_slices` iterates all fragments
   (`storage:1043-1080`), and both use a `poll_fn` that immediately returns `Ready`. In an
   async-native `drive_task` future, this is one synchronous outer `poll`, not a persisted cursor
   with one page/copy opportunity. `db_io_transfer_list` and `db_io_close_platform` make the same
   mistake for list and close retirement. PostgreSQL and Neo4j call `db_io_transfer_list` on WAL,
   snapshot, and index list results (`postgres:706-757`; `neo4j:883-934`).

2. **Artifact rejection retirement loses the exact owner at capacity.**
   `retire_artifact_state_owner` sets `ARTIFACT_STATE_RETIREMENT_SATURATED` and then
   `std::mem::forget(owner)` (`artifact:475-485`). The staged entries/rejection have no remaining
   close cursor or handback route.

3. **Compaction page retirement loses retained pages at capacity.**
   `retire_compaction_pages` has the identical `std::mem::forget(owner)` branch
   (`compact:310-320`). A Boolean saturation marker is neither the rejected typed owner nor a
   credit-return witness. The new fixed array therefore does not meet its max/+1/lossless gate.

4. **Query and engine close paths have the same non-lossless saturation branch.**
   `retire_query_rows` forgets `QueryRows` (`query:800-810`) and
   `retire_engine_query_stream` forgets `QueryStream` (`engine:2407-2414`). Thus ordinary and
   interrupted query close cannot be claimed lossless under a full retirement registry.

5. **Compaction/snapshot close still falls through to ordinary field Drop.**
   `collect_chain_pages` does one `cursor.close_step()` and returns (`compact:393-419`), while
   `build_cold_archive` does the same (`compact:486-493`). On any remaining cursor owner or error
   exit, Rust's default field destruction is relied on rather than a mounted snapshot close cursor
   that records its terminal witness.

## Verifier, Formatting, and Diff Evidence

- `bun ./📜️script.ts verify interactivity p1q-b1-b6` exited **0** and printed
  `live-source and hostile mutations clean.` This is a false green relative to the evidence above:
  its R4 predicate only scans selected module snippets (`📜️script.ts:8004-8086`), omits shared
  storage helpers, and does not reject `std::mem::forget` saturation paths.
- `rustfmt --edition 2021 --check` on the scoped P1q storage/backends/R4 modules exited **0**.
- Scoped uncached and cached `git diff --check` exited **0**.
- Scoped uncached/cached `git diff --name-status` showed modifications only and **no `D` entries**;
  this audit found no silently deleted scoped source/test file. That does not cure the incomplete
  hostile-law bodies above.

## Bounded Repair Packets

1. **P1q-B3/B4 lossless retirement:** replace every saturation `forget`/permanent-flag branch in
   storage, artifact, compact, query, and engine with preflighted capacity that returns the exact
   owner before mutation, or a durable, typed, recoverable terminal owner. A saturation marker is
   not an owner. Add tests that fill each ring, assert the exact max+1 owner identity, then close
   all accepted owners and recover the exact prior ledger witness.

2. **P1q-R4 real one-poll cursors:** replace `db_io_write_observed_bytes`,
   `db_io_prepare_platform_slices`, `db_io_transfer_list`, and `db_io_close_platform` with
   explicit stateful futures/cursors. Each `poll` must advance at most one page/list/close owner and
   return `Pending` after arming a governed next `Lane::Io` turn; it must preserve cancellation,
   stale generation, and exact source/output ownership.

3. **P1q-B1/B5 backend-close lane:** retain a generation-qualified backend-close work item and
   submit/poll PostgreSQL's real close future only from `Lane::Io`, including the lost-facade route.
   Remove the `Waker::noop()` maintenance poll; test a lost facade through that actual worker path.

4. **P1q-B6/R4 laws and verifier:** make each named artifact/compaction/CLI hostile law execute
   success, refusal, cancellation, stale generation, injected close fault, interruption, recovery,
   and ring max+1. Extend the source predicate/mutations to kill `mem::forget`, Ready-only
   pseudo-yields, and storage-level terminal drain loops.

No runtime claim is made: the permitted source-only checks above do not establish runtime behavior.
