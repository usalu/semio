# WAL Writer Release Controller Binding: Current Audit

Status: read-only source audit, 2026-09-05. No build, process, or product source was changed. The owner reports the nine-law primitive group pending under pressure; this report makes no result claim.

## Current boundary

The signal and writer-table primitives exist, but there is no mounted backend binding yet. The registered source gate itself calls this an **independent** capability check at [OS script:11-12](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:11), and the current native group contains only the table/signal and generic DB-I/O laws at [OS script:64-70](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:64). A repository-wide source read finds no caller of `WalWriterTable`, `WalWriterPermit`, or `WalFileWriterGuard` outside their defining module/tests.

That is the correct current claim: no `WalStorage` mutable operation, `ArtifactWal`, filesystem executor, or backend close path has writer ownership yet. The `WalStorage` surface is deliberately raw segment storage ([storage:10-16](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:10)); the controller is the necessary bridge, not a completed exclusion feature.

## P0 conditions for the proposed controller

### 1. Fence mutations at signal time, not hook time

`release::request` only sets the cell bit ([release:26-32](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:26)). The table currently checks only `entry.releasing` in `validate` ([writer:81-85](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:81)) and `pin_operation` ([writer:87-95](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:87)). A dropped permit can therefore be signalled, but a mutation can enter before the later Lane::Io hook sets `releasing` through `release_step` ([writer:105-113](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:105)).

Make every actual mutable WAL admission call one table-internal `pin_mutation(key, operation)` which, while it owns the table, reads `release::requested(key)`. A matching request returns `Closed` before assigning a new pin. Only the operation already pinned before the request may resume. This is safe with the intended lock order `backend registry/table -> signal cell`: `Drop` holds only the signal cell and releases it before scheduling; it never waits for the registry.

Do not treat a preflight `validate` as mutation authority. Every one of `create`, `append`, `sync`, `seal`, `truncate`, and `delete` must carry the exact permit key through its final `pin_mutation`, and final task cleanup must call `finish_operation` once. The fixture already names the six mutation classes ([writer fixture:10](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🧪️fixtures/🔣️.json:10)).

### 2. Do not wake a caller under the backend-registry mutex

`release::finish` extracts a waker and invokes `wake` ([release:114-117](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:114)). The proposed controller would call it while it has the backend registry, as normal blocking execution does ([storage:2773-2795](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2773)). A caller waker may synchronously poll a DB future, take the registry, or request backend close. That is a real re-entrancy/deadlock surface.

Replace the public-to-controller terminal operation with a private `finish_take_waker(key) -> Result<Option<Waker>, DbError>`: it advances the epoch, clears the exact active cell, and returns the waker without invoking it. The callback removes the exact table entry, calls this while still holding its ownership locks, then drops the backend registry and only then invokes the returned waker. The current direct `finish` must not be used by the controller.

Likewise, `finish` and reservation cancellation currently assert exact activity ([release:34-46](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:34)). A delayed controller callback after a close/recycle must be a typed fenced/no-op outcome, not an assertion that the WorkerPool catches as `Fault` ([maintenance:54-63](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:54)). An assertion there can poison the signal mutex and leave an exact guard retained forever.

### 3. The drop wake needs a separate fixed scheduler witness

The cell's single `waiter` is the external close future ([release:8-13](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:8)); it cannot also be reused as the dispatcher wake. `request` alone cannot make an idle WorkerPool execute a callback. The new WorkerPool hook is suitable: requests coalesce and wake an idle native worker without a `Job` allocation ([async:1736-1748](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:1736)), and selected maintenance runs through the ordinary lane/permit path ([async:1606-1630](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:1606)).

Use a second, preallocated controller wake—not the completion waiter. One fixed controller row per backend slot contains:

```text
exact DbIoBackendControl
WorkerMaintenanceTicket
preallocated scheduler Waker / pool request authority
release cursor (0..32)
closing, callback_running, release_fault state
```

After it releases the signal-cell mutex, `WalWriterPermit::drop` invokes only that preallocated scheduler wake. Its `Wake` implementation validates the full controller generation and does `pool.request_maintenance(ticket)`; it does not look up the backend registry, submit a task, allocate a job, touch a lost-owner ring, or perform I/O. A shutdown/stale/closed request is ignored **with the signal still set**; this is fail-closed, not release success. Keeping the scheduler wake external to the signal cell also leaves the one completion waiter semantically unambiguous.

The controller row must be fixed backing, never an `Arc` held by the permit. Letting a permit keep an `Arc` to the controller would let an external forgotten permit determine controller lifetime and obstruct backend removal. The full cell key already prevents a late old permit from scheduling a reused controller.

### 4. Linearize mount before publication and remove after terminal drain

`register_db_io_backend_reserved` installs a live backend row and returns its control ([storage:2660-2706](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2660)); backend credit returns only late in close ([storage:2921-2948](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2921)). The installation sequence needs an unpublished `Mounting` slot state:

1. Reserve the existing backend owner credit and registry physical slot/generation; retain executor and set `writers_ready = false`.
2. Drop the backend-registry mutex; install the exact `Lane::Io` hook with context `[slot, generation]`.
3. Reacquire the registry, verify the same unpublished generation, store the ticket/controller, then set `writers_ready = true` and return `DbIoBackendControl`.
4. If hook install or this recheck fails, keep the executor and owner credit together in the existing rejected-backend retention path; do not silently drop it. A capacity or shutdown error is an `Unavailable` backend-open failure, not a backend that accepts unguarded WAL writes.

No writer permit can be acquired before `writers_ready`. Conversely, acquisition needs an all-or-nothing preflight: reserve table key/slot and signal epoch first, then construct the backend guard on Lane::Io, then commit the table entry with no remaining fallible step. The current table's `acquire` immediately publishes an entry after receiving a guard ([writer:62-70](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:62)); calling `release::prepare` after that can leave an orphaned entry if signal preparation fails. Do not open a filesystem lock and then dispose it because signal/table admission failed.

At backend close, mark writer admission closed first. The existing close gate only considers ordinary `pending_operations`, `admitted_operation`, and `leased_operation` ([storage:2877-2893](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2877)); extend it to retain the executor/owner credit while any writer table entry, signal, or release callback remains. Close drains its table itself, including permits never dropped, advancing the exact signal epoch before physical table-slot reuse. Then call `remove_maintenance_hook`.

`remove_maintenance_hook` intentionally returns `false` while its callback is running ([maintenance:97-105](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:97)). Therefore a controller cannot remove its own hook and return the backend credit in the same callback. It must retain `closing + ticket` through the callback return, have the next backend-close opportunity observe `remove == true`, and only then release `owner_operation/owner_credit` and recycle the slot. A concurrent post-close `Drop` requests a cell but receives a closed scheduler; the shutdown table drain—not the hook—provides its terminal acknowledgement.

### 5. Blocking and async lease boundaries both need a re-request

The controller callback may run only when the backend has no current `admitted_operation` or `leased_operation`; the registry exposes both fields ([storage:2412-2428](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2412)). If either is set, leave the signal retained and return `Idle`, not `More`, to avoid a one-core busy loop. When the already-pinned operation finally calls `finish_operation`, it requests that controller hook again.

The async path must do this after it returns the executor at [storage:3672-3703](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3672) and after the manual async lease path returns it at [storage:4001-4036](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4001). Both functions must drop the registry first, then request the fixed hook. Otherwise a release arriving during an async lease can remain `Idle` forever after the lease returns.

For a guard I/O failure, retain the table entry, matching active signal, and a keyed fault in the controller; do not clear `requested` as completion and do not repeatedly return `More`. The generic hook's `Fault` result otherwise merely leaves its slot unrequested ([maintenance:136-144](/Users/ueli/Documents/semio/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:136)) and carries no DB error. Make a later explicit close/retry or backend drain request another turn; expose the stored exact error to an explicit close owner rather than making its future falsely succeed or pend without diagnosis. A dropped owner may remain fail-closed after an unlock fault.

## Minimal bounded implementation

1. Add one private `DbIoWriterReleaseController` to each existing `DbIoBackendRegistrySlot` plus `writers_ready`; no `DbIoTask`, task slot, lost owner, or timer is added.
2. Add a private executor capability that drives one exact writer-table release and returns `WaitingPinned | Progress | Removed(WalWriterKey) | Fault(DbError)`. It must not return a borrowed guard/table. Implement the first real path for Memory and filesystem; storage kinds without an actual cross-process guard fail writer acquisition closed until their provider guard exists.
3. Add an exact controller callback `fn([u64; 2]) -> WorkerMaintenanceStep`. It validates the backend generation, skips admitted/leased work, scans one requested writer cell from its cursor, drives one bounded table step, and captures a completion waker. It releases the backend registry before waking. `Progress`/another eligible requested cell yields `More`; a pin yields `Idle`; a storage error latches a keyed failure and yields `Idle`.
4. Give `WalWriterPermit` a `Drop` that only sets `request_if_exact` and invokes the preallocated scheduler wake after the signal mutex is dropped. Add the explicit close future separately; it captures `terminal_epoch + 1` before request and succeeds only after exact removal.
5. Extend backend close with a writer-drain phase and a hook-removal phase before owner-credit return. Hook removal `false` is a retained close state, never an implied terminal result.

### Accounting

The 64 × 32 signal matrix and 64 controller rows are fixed process backing. Account their `size_of` in a named static-capacity witness; do not charge a permit, a signal request, or a hook invocation to the 64 task-operation ledger. If the `WalWriterTable` lives inside a concrete executor, include its inline size in that executor's `owner_backing_bytes`, which is exactly what the backend owner reserves today ([storage:2397-2399](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2397), [storage:2648-2657](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2648)). Do not reserve a second owner operation: the backend already has one static control credit. The controller must reuse the slot's existing pool authority rather than hold an additional counted `Arc`.

## First native acceptance laws

1. **Signal-time fence.** Pin A; drop/request it; a new operation B with A's valid permit is denied before the hook starts. The original pinned A can finish once, and only that finish requests a later release turn.
2. **Deferred wake.** Install a completion waker that probes the backend registry. Terminal release invokes it only after the controller has dropped the registry lock; assert the exact epoch and table removal, with no deadlock or poisoned signal cell.
3. **Async lease wake.** Request a writer while an async-native executor is leased. The hook does one `Idle` turn. Return the executor, finish the pin, and prove one later hook release without a new DB task or a busy `More` loop.
4. **Mount/remove failure ownership.** Exhaust WorkerPool maintenance-hook capacity, attempt backend registration, and prove no control is published and the executor/credit remains in retained backend cleanup. Separately close while the hook is running: first remove is `false`, owner credit and physical slot remain retained; the next close step removes the hook and only then recycles the control.
5. **Drop under pressure.** Saturate task and all lost-owner tiers, drop an active filesystem writer, and prove no task/lost-owner occupancy changes, the separate process remains denied, and the mounted hook later performs terminal release. Repeat after backend and writer-slot reuse to reject both stale drop and stale callback.

## Decision

The fixed controller/hook direction is coherent only with the two-phase admission, signal-time fence, deferred completion wake, and hook-removal-before-credit-return above. The current primitives are a valid foundation, but directly calling the present `release::finish` under the backend registry or letting `Drop` merely set the current bit would respectively introduce re-entrant deadlock and a real post-drop mutation window.
