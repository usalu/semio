# WAL Writer Controller and Signal-Table Landing Audit

## Scope and current status

Read-only review of the newly landed fixed signal cells, controller rows, worker-maintenance bridge, and DB registration/close integration. This is not a Memory/FS mounting or runtime qualification: `supports_writer_authority()` is not yet enabled for those production executors.

The intended non-reentrant Drop path is present: [`WalWriterPermit::drop`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:41) only marks a fixed signal then requests the controller; it neither allocates nor enters the DB lost-owner/task registries. Signal preparation precedes guard construction, and the reservation cancels on construction failure. Registration binds the executor and installs the hook before placing the executor in the backend registry. `db_io_backend_close_lane_step` runs `notify_terminal` after taking the executor out of the backend registry, so deferred waiter wakes are outside that mutex.

The callback's normal lock order is sound on this revision: controller row is copied then dropped before it enters the backend registry; the release cell is only entered under the executor/table path; signal request drops its cell lock before taking the controller lock. `close_controller` may retain the backend while `remove_maintenance_hook` reports running, which is the right non-stale close contract.

## P0 — controller faults are backend-wide, erasable, and make a consumed release future non-recoverable

[`request_controller`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:168) writes one `WalWriterController::fault` for the whole backend. Any successful later request clears it. `db_io_backend_return_operation` and `db_io_return_async_executor` unconditionally issue such a request after **every** ordinary operation, not only a release ([storage](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2851), [storage](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2879)). Meanwhile [`WalWriterRelease::poll`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:95) converts that backend fault into a terminal `Err`, even though its exact signal cell remains active and its guard/table entry is still retained.

Consequences:

1. A failure closing writer A can make writer B's unrelated waiter fail.
2. An unrelated operation can erase A's observed failure before A next polls, producing nondeterministic `Pending` versus `Err` for the identical state.
3. Once A has returned `Err`, its `WalWriterRelease` is consumed as a Future result; there is no exact retry/ack capability to re-request or observe eventual completion. The still-retained entry then depends on unrelated DB traffic or backend shutdown.

Minimal correction: make controller failure an immutable, exact-`WalWriterKey` completion state in the corresponding signal cell (or return a retained release object with an explicit `retry_step`). Never clear it because another key submits a maintenance request. On a hook `Fault`, either (a) complete that exact release with the error and provide a retained recovery/close capability, or (b) retain it pending and schedule a defined retry opportunity. It must not be a backend-global transient flag.

Add a native law with two writers on one registered test executor: writer A's `close_step` fails once; B polls while A is faulted; an unrelated DB operation returns; assert B remains pending, A's error is stable, and the documented A recovery path retires exactly its guard and waiter without traffic from B.

## P0 — admission credit lacks a demonstrated fixed backing accounting boundary

The dynamic controller credit charged at backend admission is only `WalWriterControllerWake + 2 * usize` ([release](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:154)); it plausibly represents the `Arc` payload/header allocated for the preallocated `Waker`. However, the two fixed global matrices are separately declared as `WAL_WRITER_SIGNAL_BACKING_BYTES` and `WAL_WRITER_CONTROLLER_BACKING_BYTES` ([release](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:61), [release](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:152)) but are not referenced by the DB process-budget declarations or a source witness. The reservation identity check at [storage:2674](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2674) only proves the caller supplied the same small dynamic credit.

The fix need not charge fixed static arrays 64 times. Establish one explicit process-owned static backing budget/witness for both matrices, and separately define the exact per-live-backend allocation as the `ArcInner<WalWriterControllerWake>` payload/header plus its items/controls. Add compile-time/source assertions that the matrices are bounded by `DB_IO_BACKEND_CONTROLS × WAL_WRITER_CAPACITY`, and a native capacity law filling all writer-capable backend slots: the ledger must return to its precise prior witness after all controllers/hooks/Arc wakes retire.

## P1 — notification wake is physical-slot scoped rather than full-control scoped

[`notify_terminal`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:128) iterates every cell of the physical slot and removes any pending notification without checking `active`/completed key's full backend generation. Current normal close ordering should drain old notifications before slot reuse, and `prepare` prevents reuse of the same cell while a notification remains. Still, the type does not encode that crucial ownership invariant: a malformed/partially retired executor could wake an old-generation waiter from a later generation's controller pass.

Store a `notification: Option<(WalWriterKey, Waker)>` and wake only values whose `key.backend == backend`. This makes the full-control fence local to the signal protocol, rather than relying on every current and future executor's close sequence.

## P1 — completion and close laws needed before Memory/FS mount

The present unit law exercises a local signal cell, not the actual backend/controller bridge. The next registered custom-executor native gate should cover:

1. Drop a permit while an operation pin is live: the hook observes the signal but does not close; `finish_operation` re-requests; the exact waiter wakes only after `release_step` removes the guard.
2. `prepare` after `finish` but before deferred `notify_terminal` fails; after `notify_terminal` it succeeds with a newer generation and the old future observes its original terminal epoch.
3. Backend close racing a queued hook: first `remove_maintenance_hook` returns `false`, so slot/owner credit remain retained; the next close opportunity returns `true`, wakes deferred waiters, removes the exact hook, then and only then retires owner credit.
4. Hook panic/error: validate the chosen exact-key recovery semantics above and prove no lost guard/table cell/credit after the test executor is closed.
5. Full controller-slot reuse: old `Wake::wake_by_ref`, old permit drop, and old terminal notification cannot request/wake the newly generated backend.

## No blocking lock inversion found

I found no direct controller/signal/backend mutex cycle in the current source. The important existing safety properties are that [`controller_step`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:180) drops the controller row before entering `db_io_writer_release_lane_step`, and [`db_io_backend_close_lane_step`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2905) runs executor close plus notification with the registry released. Preserve those two properties when mounting Memory/FS.

## 2026-09-05 addendum — per-key failures and mounted controller

**Scope.** Read-only inspection after the per-key `fault`, retained `WalWriterReleaseFailure::into_parts`, exact `retry`, and controller native-law additions.  The earlier backend-global fault is gone: a failed table close records only the selected active key, and retry clears only that exact active cell ([`release/🦀️.rs:107-122`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:107>), [`writer/🦀️.rs:181-195`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:181>)).  I did not run the reported controller native laws.

### P0 — one failed key strands an already-requested healthy key

Two releases may coalesce into the same maintenance invocation: each permit marks its own cell requested, while `WorkerMaintenanceRegistry::request` records one hook request ([`release/🦀️.rs:216-225`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:216>), [`maintenance/🦀️.rs:87-95`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:87>)).  The table processes one key per turn.  If A faults, it marks A's cell faulted and returns `Err`; the controller wakes A but returns `WorkerMaintenanceStep::Fault` ([`writer/🦀️.rs:181-195`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:181>), [`release/🦀️.rs:235-249`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:235>)).  A maintenance `Fault` deliberately leaves the hook unrequested; only `More` re-requests it ([`maintenance/🦀️.rs:136-144`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:136>)).

Therefore B, which was requested before A's callback started, remains a healthy requested signal but gets neither another table turn nor a wake.  It is released only if unrelated later traffic happens to request this controller.  That violates the new per-key isolation guarantee.

**Small correction.** The per-key cell already retains the error and A's exact retry authority, so the controller must not convert that condition to a pool-wide `Fault`.  After waking faulted waiters, return `WorkerMaintenanceStep::More`.  The following bounded callback skips faulted A via the existing selector, retires B if present, and returns `Idle` when A was the only request.  This schedules at most one harmless empty turn and preserves worker-pool DRR fairness; it neither clears A's fault nor invokes release recursively.

Add a real controller law with A (`close_step` fails once) and B (successful) dropped **before the first hook turn**.  Poll both futures to install distinct wakers.  Require A to yield its retained failure, require B to complete without any B retry or new controller request, then retry A and prove exact pre-test ledger restoration.  The present two-writer law creates B only after A's fault path and thus cannot expose this interleaving ([`storage/🦀️.rs:7817-7819`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7817>)).

### P0 — maintenance-admission refusal changes a future to error without waking it

`request_controller` converts a refused/stale/closed maintenance ticket into an exact-cell fault, but it leaves every stored `cell.waiter` in place and returns without waking any of them ([`release/🦀️.rs:216-225`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:216>)).  A `WalWriterRelease` that was polled first has registered its waker ([`release/🦀️.rs:127-143`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:127>).  If the pool has shut down or the hook was retired, its state changes from Pending to `Err(WalWriterReleaseFailure)` but no executor is told to poll it again.  This is a real future-progress deadlock, not merely a missing diagnostic.

**Small correction.** On request failure, collect `cell.waiter.take()` for every *exact active requested* key whose fault is newly stored; release the controller and cell mutexes; then wake those wakers.  Do not wake under either mutex.  The next poll observes the immutable per-key error and transfers the same retry owner.  A retry that is refused again must repeat this notification; it must not clear another cell's fault.

Add a dedicated-pool native law: install one test controller, poll its release to register a counter waker, shut down that pool (so `request_maintenance` deterministically rejects), then request/drop the permit.  Assert one wake, the next poll returns the retained failure, no terminal epoch/guard is fabricated, and backend close still performs the sole terminal retirement.  A two-key variant should prove both prior waiters wake exactly once and retain independent errors.

### Mount rollback — no current first-party data escape, but ordering should eliminate the partial mount

The current registration order binds writer state first and installs the maintenance hook second ([`storage/🦀️.rs:2701-2725`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2701>).  The existing filesystem implementation performs an empty-table assignment atomically after its `is_none` check ([`storage/🦀️.rs:6951-6963`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6951>), and the unpublished executor is parked for retained close when install fails.  Thus this revision has no observed first-party guard escape: no permit can exist before backend-registry publication, and rejected close drains the empty first-party table.

The public executor contract nevertheless has no rollback method or requirement that a failing `bind_writer_control` leave no state.  More importantly, maintenance-capacity failure happens after the bind.  Reverse only these two mount actions: install the controller/hook for the reserved yet unpublished control first; then call `bind_writer_control`.  If bind fails, remove that never-requested exact hook (it must return terminal `true`), then retain the un-published executor through the established rejected-owner path.  Specify `bind_writer_control` as failure-atomic; no implementation may acquire a document lock, emit a permit, or retain a partially bound table on `Err`.

Add a mount-capacity law using a dedicated pool with all 64 maintenance hooks prefilled.  Registering a writer-capable counting executor must fail before `bind_writer_control` runs, leave no controller row/signal reservation, and restore the exact DB ledger after the existing retained-owner cleanup.  This tests the failure branch without relying on a dropped guard or an ignored mount error.

### Lock and notification result

The normal terminal path remains correctly non-reentrant: `controller_step` has returned from `db_io_writer_release_lane_step` before it calls `notify_terminal`, and `notify_terminal` takes each deferred waker then wakes it after the cell guard expression completes ([`release/🦀️.rs:228-246`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:228>).  The mounted law's registry probe is an appropriate assertion for this terminal path.  The two P0 cases above are notification/scheduling omissions rather than a discovered mutex inversion.

## 2026-09-05 addendum — safe refusal-fault notification boundaries

**Scope.** This traces the current request-controller call graph only.  It does not claim a build or alter the proposed per-key `Err → More` repair.  The required property is narrower and concrete: a waker for a newly faulted release may run only after the backend-registry, task-slot, and lost-owner mutexes that were held by its originator have all been dropped.  A wake may synchronously repoll and reenter any of those paths.

### One notifier with taking semantics

Add private `notify_faults(control)` beside `notify_terminal` in [`release/🦀️.rs`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:177>).  It must, for each physical cell, lock only long enough to `take()` the waiter where all four predicates hold: the active key has the full `control`, `requested`, and `fault.is_some()`.  It then wakes the collected/taken values after every cell lock is released.  Taking is important: keeping a clone re-wakes an obsolete failure waiter on every `More` pass and lets it be overwritten by a later retry.  The next retained-future poll already observes the stable fault without another wake.

`controller_step` is the canonical automatic boundary.  It copies and drops the controller row, calls `db_io_writer_release_lane_step`, whose backend-registry guard ends on return, and only then handles notification ([`release/🦀️.rs:228-246`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:228>).  Call both `notify_terminal(control)` and `notify_faults(control)` there unconditionally before mapping `Err` to `More`.  This services (a) a guard error recorded by the table and (b) its healthy coalesced neighbour in the following hook turn, without calling back into the controller under a mutex.

### Exact remaining request sites

| Origin | Existing lock state at `request_controller` | Safe wake boundary / required change |
|---|---|---|
| [`WalWriterPermit::request_release`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:35>) including `Drop` | No DB backend/task/lost-owner guard is acquired by this function. | After `request_controller` returns, call `notify_faults(key.backend)`. The first permit itself has no waiter, but a distinct already-polled writer on the same controller can be faulted by this refusal, so omitting this call still strands that other waiter. |
| [`WalWriterRelease::retry`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:114>) | It drops its exact signal-cell lock before requesting. | Under the same cell lock, clear **both** the exact `fault` and obsolete `waiter`, then call `request_controller` and `notify_faults` after the lock. The consumed failure's waker must never be retained to wake a retry in a different executor. |
| [`WalWriterControllerWake`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:192>) | This is the saved `Context` waker, invoked by the executor after its pending poll; it holds no controller or backend guard here. | Call `notify_faults` after `request_controller` in both `Wake` methods. This covers an external driver refusing the follow-up schedule without requiring a thread or task submission. |
| [`db_io_return_async_executor`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2908>) | It drops the backend registry first. Its three production callers—`db_io_finish_async_driver` (3732), `DbIoAsyncTaskLease::complete` (4061), and lease `Drop` (4122)—all call it before acquiring a task slot. The `take_async_native` rollback at 4239 also calls it before reacquiring the slot. | It may call `notify_faults` directly after `request_controller`. Preserve that order. Do not move its caller-side invocation beneath a task-slot lock later. |
| [`db_io_backend_return_operation`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2878>) from allocation rollback | `db_io_allocate_task` calls it at 3410 while its task-arena guard is still live. | It must drop that arena guard before `notify_faults(task.backend())`. The mutation/request itself may remain under the arena guard because it is non-waking. |
| `db_io_backend_return_operation` from [`db_io_task_close_step`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4513>) | **Task-slot guard is live.** This is the sole non-test call site that is unsafe for direct notification; `db_io_task_close_step` can itself be entered from the lost-owner/maintenance route. | Keep `db_io_backend_return_operation` mutation/request-only. In the task-close branch, set `backend_admitted = false`, explicitly drop `owner`, then call `notify_faults(backend)` before returning. If that call ever returns `Err`, retain the same task-close owner and defer the flush; do not use `?` while its task guard is still live. |
| `WalWriterTable::finish_operation` reached through [`db_io_executor_close_operation`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2922>) | It invokes `request_controller` while the backend registry is deliberately still held, and its only caller is the task-close branch while the task-slot guard is also held (4454). | **Do not notify in either function.** Capture `backend` and the close `Result`, drop the task owner, run `notify_faults(backend)`, then apply `?`/rotate/relock. This one local restructuring covers terminal, nonterminal, and error paths without adding a task-slot field. |

The last row is the important edge: adding notification immediately after the registry guard in `db_io_executor_close_operation` is still wrong because its caller retains `DB_IO_TASK_SLOTS[handle.slot]`.  Likewise, invoking it from `db_io_backend_return_operation` universally is wrong for the task-close call even though its own registry guard has ended.

### Minimal owner protocol, no dispatcher thread

No worker, lost-owner entry, task-arena reservation, or allocation is needed for refusal notification.  Keep `request_controller` as a non-waking fixed-cell state mutation.  At each safe boundary above, run the fixed `notify_faults(control)` sweep.  For the one task-close path, make the wake a two-stage close transition:

1. While the task slot is held, capture its exact full `DbIoBackendControl` and evaluate the close/return `Result` without `?`.
2. Drop the task slot (and never hold a lost-owner mutex), call `notify_faults(control)`, then apply the captured result.
3. Re-enter/rotate only afterwards.  On a nonterminal close, rotation creates the next bounded opportunity; on error, the existing retained task remains in the close ring, so its next close opportunity repeats the non-waking request/flush sequence.

This is preferable to calling arbitrary release wakers while the global close turn is held.  It also makes the notification retriable without confusing it for proof that the guard released: notification merely exposes the retained failure, while `WalWriterTable` remains the sole owner of physical guard retirement.

### Retained-failure API must remain linear and explicit

The current [`WalWriterReleaseFailure::into_parts`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:100>) is the right semantic shape: an error is not a bare `DbError`; it carries the exact same epoch/key future that is still backed by the table guard.  Preserve this and do **not** add `From<WalWriterReleaseFailure> for DbError`, `Result<(), DbError>` convenience close APIs, or a Drop implementation that clears the fault/unlocks the guard.  Each would discard the only retry/ack authority or turn a failed release into an unauthorized unlock.

Add `#[must_use]` to `WalWriterRelease` and `WalWriterReleaseFailure`, with text that says callers must either retain it or use `into_parts` then `retry`.  In `retry`, clear the former cell `waiter` together with the exact fault; the returned new Future will install its own current waker.  Dropping the failure must remain fail-closed: the backend owns the guard, the cell remains occupied, and a later backend-close path—not silent Drop—performs retirement.  This is a meaningful enforced API boundary even though Rust cannot prevent an explicit ignored `Err` at runtime.

### First focused native law

Use a registered writer-capable test executor and a counter waker.  Poll A and B to install waiters; shut down/retire the controller pool so B's later permit-drop request is refused; assert both counter wakes occur only after cell/controller locks are released, each next poll yields a separate retained failure, and the table still contains both guards.  Consume A with `into_parts().1.retry()`, prove only A's old waiter was cleared, and show B remains faulted until its own retry.  Finally close/retry both and assert exact ledger/slot restoration.  This covers cross-key refusal, taking-not-cloning notifications, retry supersession, and the no-implicit-unlock rule in one native boundary test.
