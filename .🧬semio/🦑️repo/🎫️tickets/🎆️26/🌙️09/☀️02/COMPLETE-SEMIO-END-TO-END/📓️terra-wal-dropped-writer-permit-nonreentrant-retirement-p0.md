# Dropped WAL Writer Permit: Non-Reentrant Retirement Packet

Status: read-only audit on 2026-09-05. Writer-core native five-law run is reported GREEN by its owner (`a1c4093f…b516f`); this packet makes no claim for a storage integration or a new native run.

## The release-task-in-lost-owner design is unsafe

Do not put `DbIoTaskOperation` for a writer release inside `DbIoLostOwner` and submit/poll it from `db_io_lost_owner_close_opportunity`.

`db_io_lost_owner_close_step` holds the primary/overflow/quarantine lost-owner mutex while invoking that opportunity ([storage:3822](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3822)). Both proposed operations recurse into maintenance: `submit_db_io_task` begins with `db_io_maintenance_step` ([storage:4091](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4091)), and `DbIoTaskOperation::poll` does the same ([storage:4271](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4271)). The nested call immediately attempts the same lost-owner mutex first ([storage:4318](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4318)). This is a same-thread mutex deadlock, not merely an ordering concern.

Skipping the recursive lock alone is insufficient. The current maintenance pipeline returns immediately after any lost-owner turn, before retry, rejected-backend, backend, and task-close work ([storage:4318](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4318)). A pending writer release therefore continuously wins that turn while the release task's own cancellation/fault close and backend cleanup are later stages. That is a self-starving liveness loop even without a deadlock.

The normal result handback cannot repair this. A ready release result adds result-lease credit and enqueues its task close ([storage:3945](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3945)); its task close runs only in the last maintenance class ([storage:4365](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4365)). A fault dropped while still in the lost-owner turn would itself try to park another lost owner ([storage:3163](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3163)). No `DbIoTaskOperation::finish`, `poll`, or release-result `Drop` is safe while that mutex is held.

## Recommended fixed signal-cell design

The proposed fixed signal cell is the correct smaller direction, with these exact constraints.

### 1. The writer table remains the sole durable resource owner

Use a fixed cell for each physical backend-control slot × `WAL_WRITER_CAPACITY`: `64 × 32`, indexed only after validating the full key. The cell owns no `File`, pages, task operation, pool, `Arc`, or heap allocation. It stores only inline state equivalent to:

```text
active: Option<WalWriterKey>          // includes full backend kind/slot/generation + writer slot/generation
release_requested: bool
completed: Option<WalWriterKey>       // exact acknowledgement for an explicit closer
scheduled / lane_turn / wake_requested: bool   // per-backend dispatch control, or a separate fixed controller row
```

The cell must compare the **entire** `WalWriterKey`, not just the two array indices. The current key already carries the full `DbIoBackendControl`, writer slot, and writer generation ([writer:14](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:14)). No document copy is needed in the cell if a new table-internal `request_release_by_key` verifies the stored entry and obtains its canonical document itself; copying caller text would add an unnecessary second authority.

Activate the cell after table acquire but **before** returning the permit to any caller. Its active key must be reset only after the table entry for that exact key is removed, while the table still prevents the physical writer slot from being recycled. Clear/ack the exact cell before unlocking the table for a new acquire. This ordering prevents an old drop that races terminal cleanup from flagging a newly recycled writer slot.

`WalWriterPermit::Drop` is then a signal-only, no-I/O action: `request_if_active_exact(key)` sets `release_requested` only if the active full key matches. It performs no `submit_db_io_task`, no `DbIoTaskOperation::poll`, no result handback, no allocation, and no backend/lost-owner registry lock. A stale key must never set a different active key. An explicit close may transfer the permit to the signal state; the permit is only inert after the signal accepts its exact key. A cell acknowledgement is **not** an unlock acknowledgement: it becomes complete only after the table removes the guard following a successful terminal close witness.

The signal is the non-droppable intent; the existing table remains the non-droppable guard/file owner. Thus full `DbIoLostOwner` rings or all 64 task slots cannot cause `File::Drop` release or release a new writer. If a signal cannot be dispatched, it stays requested and the table remains `releasing`, which is fail-closed rather than fake success.

### 2. Split table release into signal and Lane::Io drive phases

The current `WalWriterTable::release_step` intentionally both marks `releasing` and invokes the guard close ([writer:101](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:101)). Split it internally into:

```text
request_release_by_key(key)  // validate exact table entry; set releasing; no I/O
drive_requested_release_step(key) -> WaitingForPinned | Progress | Removed | Fault
```

`request_release_by_key` is idempotent. `drive_requested_release_step` runs only in the mounted backend's `Lane::Io` turn. It must keep the current behavior that a releasing entry permits only its already-pinned exact operation to resume ([writer:83](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:83)); it must not spin while `active_operation` is set. `finish_operation` of that exact operation requests another release wake after it clears the pin. A guard error retains both table entry and requested cell; a retry is an explicit later Lane::Io opportunity, never `Drop`-unlock.

The backend's shutdown `close_step` remains a separate all-entry drain. It must emit the same exact completion acknowledgement while removing every guard, including entries whose permit was not dropped, and `backend_terminal_is_empty` must require no active table entry/signal for its full backend generation. This preserves safe shutdown if a dispatcher is unavailable.

### 3. Use an internal backend release runner, not the task arena

Add one private `writer_release_step` capability to `DbIoTaskExecutor` and an independent fixed scheduler/controller row per backend control. It follows the existing rejected-backend pattern—`scheduled`, `lane_turn`, and `wake_requested` are useful precedents at [storage:2474](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2474)—but it neither allocates a `DbIoTask` nor owns a `DbIoTaskOperation`.

`db_io_writer_release_maintenance_step` scans requested signal cells with a round-robin backend cursor, validates the backend's full generation, and best-effort schedules one `Lane::Io` runner. The runner invokes `writer_release_step` outside the signal mutex. It must not run against an occupied blocking executor turn or an async lease; leave the request retained and let the completion path set `wake_requested`. Submission failure clears only `scheduled`, sets `wake_requested` and retirement pressure, and preserves `active + release_requested`. A later maintenance opportunity retries. Do not clear the cell or report release success on `WorkerPool` saturation, backend `Closed`, stale generation, or an unlock I/O error.

This is compatible with current backend serialization: normal blocking execution is guarded by `admitted_operation` ([storage:2773](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2773)); the release runner must observe that state and rely on the writer table's pin/releasing fence rather than inventing a task operation id. It must not use `db_io_backend_close_lane_step`, whose contract is only shutdown after no pending/admitted/leased work ([storage:2877](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2877)).

### 4. Make maintenance fair across classes

Replace the current fixed early-return order with an atomic round-robin cursor over:

```text
lost owner → page → platform → retry → rejected backend → backend close
→ writer release → task close
```

Each call performs at most one bounded opportunity and advances the cursor after any attempted class, including a blocked requested writer release. This gives every class a bounded next turn and prevents pending loss owners from indefinitely blocking the release's own backend/task cleanup. Keep the existing per-backend and per-writer cursors; a global fair cursor alone is not sufficient to prevent document A from starving B.

`Drop` may only mark the fixed cell. Eventual scheduling must come from the mounted maintenance dispatcher or a preallocated internal wake path; it must not manufacture a task/result lease in `Drop`. If the application has no independent maintenance pulse, add the same fixed `scheduled/lane_turn/wake_requested` Lane::Io callback discipline as rejected backend closure. A bare Boolean request without either a mounted pulse or a nonallocating wake leaves a guard locked forever after an otherwise idle drop.

### 5. Exact credit and lifecycle accounting

The `64 × 32` signal matrix is process-static, but it is still admitted memory/control state. Add a named `DB_IO_WRITER_RELEASE_SIGNAL_BYTES = size_of::<SignalCell>() * DB_IO_BACKEND_CONTROLS * WAL_WRITER_CAPACITY`, include it in the process/static capacity accounting and assert that `SignalCell` is fully inline and has no heap owner. It must not be charged once per permit or task: the table and signal cells are backend-owned fixed backing. Update each executor's `owner_backing_bytes` only for its inline table/controller fields; do not double count the global matrix. The current accounting has separate process and owner bounds ([storage:71](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:71), [storage:2648](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2648)); make the chosen accounting location explicit in native capacity vectors.

On backend recycle, reset only cells whose full active/completed backend key equals the retired generation, after the old backend has terminally drained its writer table. A stale permit drop then mismatches the new full backend generation and cannot request its cell. Never reset by physical backend-slot index alone.

## Native law packet

1. **No re-entry / no task-arena consumption.** Saturate the DB I/O task arena and all three `DbIoLostOwner` tiers using existing ring-pressure fixtures, drop an exact active writer permit, and assert its exact cell is requested while task-slot and lost-owner occupancy are unchanged. Drive the internal release runner after capacity returns; prove terminal release and exact ledger restoration. This fails any implementation that routes `Drop` through `submit_db_io_task`.
2. **Cell ABA.** Acquire A, request/drop it, drive terminal release, then acquire B in the same physical writer slot with a new writer generation. A's old signal/drop must not request B; B remains writable until B itself requests release. Repeat after backend generation recycle to prove full-control—not array-index—matching.
3. **Pinned-operation fairness.** Pin A, request A release, request B release, and admit one ordinary task close. The runner leaves A requested without a spin, advances B's release when eligible, and the task-close witness progresses within one global maintenance rotation. Finish A and prove its exact wake completes later; no new A/B mutation is admitted after its own request.
4. **Unlock fault and dispatcher saturation.** A first guard close returns `Io`; the active requested cell and table guard remain, no second process acquires, and a later mounted retry reaches exact completion. Separately make `Lane::Io` submission reject; assert `scheduled` clears but `wake_requested`, request, and guard remain, then retry without a new permit.
5. **Shutdown/completion acknowledgement.** Request a release and also begin backend retirement; backend shutdown drains all guards, writes exact completed acknowledgement before slot recycle, and only then becomes terminal. An old permit's final drop after recycle cannot change a new cell. An explicit close future may report terminal only on this exact acknowledgement, not merely `release_requested`.

## Decision

Adopt the fixed signal-cell + internal mounted release runner. It is smaller and safer than a `DbIoLostOwner::WriterPermit { DbIoTaskOperation }` state: it has no recursive maintenance, no saturated task-arena dependency, no lost-owner capacity dependency, and no path where failure releases the OS guard by ordinary `Drop`. It still requires the fair maintenance cursor, exact full-key cell lifecycle, and explicit terminal acknowledgement above; without those, it is only a leak-prone request bit rather than a retained release protocol.

## Addendum: Mounted Wake And Supersession Witness (2026-09-05)

### Current wake gap

There is no independent DB maintenance pulse. `db_io_maintenance_step` is called by
task submission ([storage:4091](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4091)), task polling ([storage:4275](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4275)), retained task cleanup ([storage:4306](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4306)), and explicit test drains. Its current fixed order is also ingress-only ([storage:4318](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4318)). A dropped permit in an otherwise idle process would therefore remain requested forever if it only sets a cell bit.

The existing rejected-backend and backend-close wake paths cannot be reused by `Drop`: each constructs a `Box<dyn FnOnce()>` for `WorkerPool::try_submit` ([storage:2623](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2623), [storage:3027](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3027)). `WorkerPool::Job` is exactly a boxed, one-shot closure ([async:1447](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:1447)); it has no reusable, allocation-free wake handle. Timer callbacks are also boxed and held in a dynamically keyed timer map ([async:755](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:755), [async:805](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:805)). A periodic `callback_at` dispatcher would avoid task-arena recursion, but it still allocates/reschedules indefinitely and its callback bypasses `Lane::Io` DRR; it is a fallback, not the correct retained owner primitive.

Native workers do wake at least every four milliseconds ([async:1500](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:1500)), but their loop merely fires timers and pops queued jobs ([async:1658](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:1658)). Without a mounted work item, waking their condition variable cannot consume a DB release signal. wasm similarly only advances when its host calls `pump`.

### Smallest clean primitive: preallocated pool maintenance hooks

Add one fixed, generic WorkerPool primitive rather than attempting to retain a `Job`:

```text
WorkerMaintenanceHook { slot, generation, lane: Lane, step: fn() -> More | Idle | Fault }
WorkerMaintenanceTicket { pool hook slot/generation }
pool.install_maintenance_hook(Lane::Io, step) -> ticket
pool.request_maintenance(ticket) -> Requested | AlreadyRequested | Stale | Shutdown
pool.remove_maintenance_hook(ticket) -> only after no running turn
```

The pool owns a fixed hook table (capacity must be a public finite constant; `64` covers the current maximum DB backend controls) and a requested bit per hook. `request_maintenance` performs only: exact ticket/generation comparison, `requested.store(true)`, and the existing native idle `Condvar::notify_all`; it creates no `Job`, timer entry, task, result lease, or registry owner. The native worker loop must select one requested hook as a preallocated `Lane::Io` work item through the same DRR/worker-permit path as ordinary jobs. It clears the bit before invoking `step`; a concurrent request or `More` result re-arms it. Thus a release performs one bounded physical close opportunity and cannot monopolize either the worker or the Io lane. Implement the same fixed hook table and lane selection in the wasm pool; `request` only sets the bit there and the next host `pump` consumes it.

This is a narrow extension of the existing pool, not a new dependency or a second thread. It preserves the current rule that pool jobs are finite `FnOnce`s, while adding the missing preallocated coalescing wake class. It is preferable to a permanently parked dispatcher job (which consumes a worker, deadlocking a one-core pool) and to a millisecond timer loop (which continuously allocates and bypasses Io scheduling).

Mount one hook per registered backend control, not per permit. Store its ticket in `DbIoBackendRegistrySlot` beside the existing `pool`/close scheduling fields ([storage:2405](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2405)). The hook's static DB callback receives the full backend control through its fixed hook slot, scans only that control's 32 signal cells round-robin, validates the complete backend+writer key, and drives at most one requested `WalWriterTable` release step. A permit drop does the following after taking only its cell mutex:

```text
if cell.active == permit.full_key:
    cell.release_requested = true
    obtain current backend hook by full backend control
    pool.request_maintenance(hook_ticket)
```

The backend lookup must validate the complete control generation and clone only the already-owned `Arc<WorkerPool>`; it must not enter the lost-owner, task, or result registries. If the control is already stale, table-drain-before-backend-recycle is the required terminal witness; the old drop must not mutate a replacement cell. Failure to schedule or a latched guard I/O fault leaves the signal/table entry retained and reports no completion. Backend close uses the same hook or its existing all-entry close path, and removes the hook only after table terminality plus no running/requested hook turn. A shutdown pool similarly leaves the guard retained/fail-closed; it does not turn a signal into unlock success.

### Completion must be monotonic, not `Option<key>`

`completed: Option<WalWriterKey>` is insufficient. A can terminally close, B can acquire and close the same physical writer cell, and an explicit A close future can poll only afterward. Replacing `completed` with B loses A's acknowledgement and makes an already-complete close indistinguishable from a stale request.

Keep a cell-lifetime, checked monotonic `terminal_epoch: u64` that is never reset on writer-slot or backend-slot reuse. An explicit `close(self)` first records a `WalWriterReleaseWait { key, required_terminal_epoch: cell.terminal_epoch + 1 }` under the exact active-key lock, marks `release_requested`, and consumes the permit. The driver writes `terminal_epoch += 1` only after the matching table entry's guard has returned a valid terminal witness and the entry has been removed. It may retain `last_terminal_key` only for diagnostics; correctness must not depend on that overwriteable field.

The waiter decides exactly:

```text
terminal_epoch >= required_terminal_epoch  => success for its own captured key
active/requested still matches its key     => pending (or exact retained I/O fault)
otherwise                                  => invariant failure; a replacement may not activate
                                             before the prior terminal epoch advances
```

This is a monotonic supersession witness. B's later activation/completion cannot erase A's proof: B can occupy the same cell only after A advanced the epoch. Backend recycle has the same ordering requirement—do not reset the cell epoch; full old-key drops mismatch the new active key, while an old explicit closer still observes its already-recorded threshold. Refuse new acquire/release if this epoch would overflow; wrapping would reintroduce ABA.

### Required native laws

1. **Idle drop.** With no task submit/poll/close calls after drop, prove a native pool worker consumes exactly one requested writer hook, releases the guard, and the hook has no queued `Job`, DB task-slot, or lost-owner occupancy. The wasm companion proves the next explicit `pump` is sufficient.
2. **Io fairness.** Hold one normal Io job, request two backend writer releases, and verify each hook turn consumes one bounded close step through the same DRR permit accounting; an interactive job and task-close opportunity still progress. A pinned writer returns `More` only after its pin clears—no spinning work item.
3. **No stale hook.** Recycle a backend control and writer cell, then invoke the old hook ticket and old permit drop. Neither may request, drive, or acknowledge the new control/key. Hook removal must wait for an in-flight turn.
4. **Completion supersession.** A explicit close obtains epoch 1; drive A terminal, acquire and terminally close B in the same cell before A polls; A reports success from its captured epoch. Repeat across backend recycle and assert an old drop cannot set B while old close still has its proof.
5. **Fault/shutdown.** An unlock fault and a stopped pool retain the table guard and leave the waiter non-successful; a later mounted request/retry can advance the exact epoch once. No branch completes on `release_requested` alone.
