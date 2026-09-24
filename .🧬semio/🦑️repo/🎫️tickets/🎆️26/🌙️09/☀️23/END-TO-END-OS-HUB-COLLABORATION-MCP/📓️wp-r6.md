# WP-R6 — Dropped DB Backends Retire Through Their Own Maintenance Hook; Retirement-Slot Law Budget

Slice: R6 (session 10). Native only. Private cargo target: `.tmp-ticket/wp-r6/target`. Captures: `wp-r6/generated/`.

## Status

| Item | Status |
|------|--------|
| 1. Census: what stays alive after drop-without-close | DONE (see Findings) |
| 2. Design decision | DONE: owned, hook-driven async retirement (not a must-close type) |
| 3. Implementation in `🗄️storage` (+ writer release controller) | LANDED, `[DEBUG]` lines removed |
| 4. Retirement-slot law budget | DONE: the 10 s wall budget is a liveness watchdog, not a product bound (now 60 s, named constant) |
| 5a. db `--lib --all-features` nextest, final tree | runs 13/14/15: 765, **766**, **766** of 766. The single failure was `wal_recovery_abort_faults_retry_without_duplicate_abort` ("WAL writer signal occupied" on an immediate reopen under parallel load; 30/30 in isolation). See Gaps |
| 5b. db `--lib` plain in-process `cargo test` | **NOT MET**. See Gaps |
| 5c. kernel `--lib --features sync,ureq` nextest | 1192/1193. The one failure is a peer's `os_store::sync` law. The kernel crate does not depend on the db crate |
| 5d. hub `test-quick` (hub mutex) | **327/327**, 10 skipped, on the final tree (`hub-quick-2.txt`) |

## Findings (census, `generated/serial-census-*.txt`)

Every Memory backend holds 14.3 MB of process credit (`MemoryDbIoExecutor` backing). The process budget
(`DB_IO_PROCESS_BYTES` ≈ 160 MB) therefore admits about 11 live memory backends. At the first
`backend process credit exhausted`, the census showed 12 live backends in two families:

1. **Dropped but never retired (`close_requested=true`)**, the majority. `MemoryStorage`/`FsStorage::drop` did
   request retirement, but the request was a `pool.try_submit(Lane::Io, job)`. On `Contended` (a transient
   `try_lock` miss on the lane queue, which is common) or `Saturated`, the job was dropped. The backend kept
   `close_wake_requested=true, close_scheduled=false` and waited for a later `db_io_maintenance_step`, which only
   runs when *someone else* submits DB I/O. In a quiet hub that never happens, so the backend, its 14 MB credit
   and its `WorkerPoolUse` leaked. The rejected-backend registry had the same lossy pattern.
2. **Never dropped (`close_requested=false`)**. The storage `Arc` is still owned by a retained product owner that
   the test abandoned: compaction/catalog-bootstrap/create-catalog registries, hello sessions, engine
   authorities (`generated/serial-all-1.txt`, "outlived … close_req=false"). These are not backend-retirement
   bugs. They are callers that drop engines without `close_step`.

Two more lossy paths showed up as soon as retirement became prompt:
- Completed and abandoned tasks sit in the task close ring and hold `pending_operations` on their backend.
  That ring was also drained only opportunistically.
- `request_controller` read the hook ticket, released the row lock and then requested it. This races a
  concurrent `Retire` and faulted every requested writer spuriously (`Stale`).

## Design

**Decision.** Drop-without-close stays legal. Every backend already owns a pre-admitted pool maintenance hook
(the WAL writer controller, installed at registration and counted in the backend's owner credit). That hook
is the backend's retirement slot. Retirement is owned, bounded and asynchronous: no Drop blocks, no thread
is spawned, and no request can be lost. A must-close handle was rejected: `Arc<DbBackend>` is shared by
engines, catalogs and hub state, so a statically must-close type would force a single owner that the
architecture does not have.

- `db_io_request_backend_close` marks `close_requested` and calls `writer::release::request_retirement`. That
  is a coalescing `request_maintenance` on the backend's own ticket, made while the controller row lock is held
  (no TOCTOU against `Retire`). It never allocates and never queues a closure. While the backend exists the pool
  cannot shut down, because the backend holds a `WorkerPoolUse`, so the request cannot be refused.
- `controller_step` (the hook), per turn: writer release, then the lost-owner batch, then
  `db_io_task_retirement_batch` (a bounded task-close ring batch), then `db_io_backend_retirement_turn` (one
  `db_io_backend_close_step`). It returns `More` while work is runnable, `Idle` while it waits for an event, and
  **`Retire`** once the backend reaches terminal.
- Terminal tail (`db_io_backend_close_step`, in one turn): `retire_controller` removes the deferred-wake
  partition and clears the controller row, then the owner credit is returned, the registry slot is freed and
  the `WorkerPoolUse` is dropped. The hook returns `Retire`, so the pool frees its slot without an external
  `remove_maintenance_hook` racing the running invocation.
- Events that resume an `Idle` hook: operation return and async executor handback (`request_controller`,
  already there), every `db_io_enqueue_close`, including re-enqueues (new), and the hook waker. The hook waker is
  the `close_backend_step` context (Postgres and Neo4j pool close), and an abandoned `DbIoTaskOperation` now installs it
  as the task's waker, so the terminal publication of an abandoned task wakes its backend's retirement.
- Task close ring: `db_io_task_closable` is the single predicate. A task is closed only once it is abandoned or its
  owner has taken the terminal. Previously a cancelled task whose owner still awaited its terminal could be
  closed under that owner, and the owner's waker was dropped (a hang once retirement became autonomous).
  `DbIoTaskOperation::poll` waits, instead of failing, while a cancelled result is still being retired.
- `db_io_enqueue_close` on an already-retired slot is a no-op (`db_io_result_handback` no longer races its own
  liveness check). The rotation of the close ring now happens before the close turn lock is released.
- Rejected backends (no hook): a refused `try_submit` is retained through `pool.submit_at(now)`, which re-admits
  the exact closure through the timer wheel. A refusal by shutdown or poison is recorded as a loud fault.
- Removed: `DbIoBackendCloseWake`, `db_io_poll_backend_close_on_lane_io`, the `close_scheduled` /
  `close_lane_turn` / `close_wake_requested` fields, `close_controller`, and the opportunistic
  `db_io_backend_maintenance_step` (maintenance class `backend-close`). Fixture and schema
  `🔐️writer/🧫️fixtures|🧬️schema` now list 6 classes.

### Retirement-slot law (`artifact_authority_drop_reuses_registered_retirement_slot_beyond_capacity`)
The law's purpose is slot reuse beyond capacity (65 authorities through 64 slots, with the hook pool capacity
restored afterwards). Those are exact count assertions and are unchanged. The per-iteration 10 s wall-clock
deadline only guards liveness: wall time under load average 30–47 is not a product bound. It is now the named
`ARTIFACT_RETIREMENT_LIVENESS_WATCHDOG` (60 s). The runner retirement hook still re-requests itself while the
runner is `ClosingReady`. That spin is what makes wall time grow with load; see Gaps.

### Artifact runner: a faulted close waits for explicit admission
The law `artifact_engine_close_fault_retains_exact_runner_until_explicit_maintenance_retry` failed about 50 % of the time
in isolation once retirement became prompt. Three separate paths turned a stray wake or request into a re-poll of a
faulted close: `schedule()` from `ClosingParked`, `ArtifactRunnerClosePoll::drop` on `ClosingPollingWake`, and
`ArtifactRunnerPoll::drop` for a cancelled runner. A faulted close (`close_error` set) now parks in all three. The
retirement hook re-polls it only after `ArtifactRunnerHandoff::admit_close_retry` (new field
`close_retry_admitted`). Result: 10/10 in isolation. No product owner calls `admit_close_retry` yet, so it is
`#[cfg(test)]`; see Gaps.

### Law timing fixes (all were flaky on the pre-change binary too)
- `database_create_catalog_observed_vec_and_string_overallocation…`: waits (10 s deadline) for the callback-close admission release. Before: 8/20 FAIL; after: 20/20.
- `database_create_catalog_resolved_drop_retains_use_until_terminal_drain`: a 10 s deadline replaces a 1024-yield budget. Before: 2/10 FAIL; after: 10/10.
- `db_io_retained_fixtures`: blocker and callback admissions retry on `Contended` (`admit_fixture_job`). The abandon-lifecycle wait uses a deadline instead of 1M yields. `drain_control_tasks` also offers an owner maintenance opportunity, because two laws block the pool's only Io worker.

## Evidence

| Command | Result | Capture |
|---|---|---|
| serial in-process, before | 531 pass / 171 fail (credit exhausted) | `serial-base.txt` |
| serial census, before | 12 live backends: 6 `close_req=true` with lost requests, 6 never dropped | `serial-census-1.txt`, `-2.txt` |
| serial in-process after the hook change (default features) | 598 / 104 | `serial-census-3.txt` |
| db `--lib --all-features` nextest run 3 (before the flake fixes) | 764/766 | `nextest-3.txt` |
| db nextest run 4 / 5 / 6 | **766/766**, 765/766 (`database_create_catalog_observed_vec…`), **766/766** | `nextest-4..6.txt` |
| that law, baseline binary ×20 | 8/20 FAIL (pre-existing): admission release is a callback close, and the law asserted it synchronously | console |
| same law after the wait-with-deadline fix ×20 | 20/20 | console |
| `database_create_catalog_resolved_drop…` baseline ×10 | 2/10 FAIL (1024-yield budget); after the deadline fix 10/10 | console |
| `db_io_real_queued_callback…aba` ×10 | 10/10 (drain helper keeps an owner maintenance opportunity: the law blocks the pool's only Io worker, so the hook cannot run) | console |
| kernel `--lib --features sync,ureq` nextest | 1192/1193. `os_store::sync::…fixtures_replay_matches_expected_events` fails deterministically (5 s seed deadline). The kernel does not depend on `semio-framework-os-kernel-db`, and the store sync files are under peer edit (MM) | `kernel-nextest-1.txt` |
| hub `nx run os-hub:test-quick -- --no-fail-fast` under `fleet-mutex.sh hub r6` | **327/327** | `hub-quick-1.txt` |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | the markers for the removed lane-poll functions were updated in `📜️script.ts` and its self-test fixture (`🔬️interactivity-db-io-b1-b6/🟦️.ts`). The gate was already red on HEAD for about 150 unrelated stale markers, so it is not a usable gate | `/tmp` console |

## Files changed (R6)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs`: hook-driven backend retirement, task close ring batch and closable predicate, abandoned-task hook waker, idempotent enqueue, rotation under the turn, rejected-backend timer retention, poll waits for cancelled-result retirement.
- `…/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs`: `request_retirement`, `retirement_waker`, `retire_controller`, `Retire` from `controller_step`, locked request in `request_controller`.
- `…/🛢️db/🗄️storage/🔐️writer/🧫️fixtures/🔣️.json`, `…/🧬️schema/🔣️.json`: maintenance classes.
- `…/🛢️db/🗄️storage/🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs`: `drain_control_tasks` asserts autonomous drain with a deadline; lifecycle inspections tolerate autonomous retirement; `db_io_backend_close_step` rename.
- `…/🛢️db/⚙️engine/🧪️tests/🔬️unit/🦀️.rs`: two create-catalog laws wait with deadlines for callback close.
- `…/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`: liveness watchdog constant.
- `…/🛢️db/🗄️storage/🧪️tests/🔬️interactivity-db-io-b1-b6/🟦️.ts`, `📜️script.ts` (`interactivityDbIoB1B6Failures` markers).
- R4's `⚙️engine/🦀️.rs` retry-deadlock fix was not touched.

## Gaps
- **Flaky under nextest parallel load**. Seen in 1 of each 3-run set, never reproduced in isolation:
  (a) `wal_recovery_abort_faults_retry_without_duplicate_abort`: `ArtifactWal::open` right after `close()`
  hits "WAL writer signal occupied". The release notification of the previous writer is still in its signal
  cell because the drain is asynchronous. The fix is for reacquire to wait on the terminal epoch instead of
  refusing.
  (b) `hello_sessions_retire_when_drained_and_when_close_races_the_returned_frame`: about 4 in 12 in isolation on the
  final tree. Census at the failure: 0 DB I/O tasks, and the hello state is `driver=Retry`, retry job retained,
  `cancelled`, `abandoned`, `close_requested`. The Retry→Queued resubmission is accepted, but the job never runs
  within 5 s. That is a `🔄️sync` driver or Io-lane issue (H5's area). I could not prove whether this slice
  changed its rate.
- No product owner admits a faulted artifact-close retry (`admit_close_retry` is `#[cfg(test)]`). Before this
  slice, stray wakes retried it by accident. A database/hub shutdown owner should admit it explicitly.
- **Plain in-process `cargo test -p semio-framework-os-kernel-db --lib` is still not green** (last run: 437 ok / 107 failed
  before the 580 s cutoff, several `db_io_retained_fixtures` laws spinning on `fixture_serial`,
  `cargo-test-1.txt`). This slice fixed backend retirement. Two families remain, and neither is in backend retirement:
  1. *Never-dropped storages*: tests abandon engines, compaction, catalog-bootstrap/create-catalog, hello sessions
     and authorities without `close_step`. Their retained registries keep the `Arc<DbBackend>` (and its 14 MB credit)
     alive. These are the "engine dropped without close" owners, and each registry needs the same treatment given
     to backends here: a drop that parks the owner and retires it through a pre-admitted hook.
  2. *Process-global witness laws*: about 100 laws assert `ledger_witness() == before` over the process-global
     DB I/O ledger. They also run on the in-process baseline (`db-base`), where 12 of 76 `db_storage::` tests fail
     under parallel threads. In one process they race every other test and every asynchronous retirement. They
     need scoped witnesses, meaning ledger credit tagged by pool or backend, rather than more serialization.
  Both are crate-wide work packages. They are not regressions: nextest, which runs one process per test, is green.
- `artifact_runner_retirement_step` re-requests itself while the runner is `ClosingReady` (a busy spin, and the
  reason the law's wall time scales with load). The clean fix is to have the runner close future register the
  hook waker instead of polling with a `noop` waker.
- `close_db_io_backend` (the explicit async close) still waits by `wake_by_ref` spin. It should park on a
  terminal waker that the hook fires at `Retire`.

## Pids
All detached test and build pids I started are finished or killed. None of my cargo or test processes is running.
