# Writer Refusal Reentrancy And Current API Caller Packet

Status: read-only source audit on 2026-09-05. No product source, process, or build was changed.

## Correction to the earlier refusal finding

The newly released permit does **not** have a same-key parked waiter. `WalWriterPermit::release` marks the request before it moves out the first `WalWriterRelease` ([writer/🦀️.rs:30-36](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:30>)). `retry` clears the former failure's `waiter` before it asks the controller again ([release/🦀️.rs:114-120](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:114>)). `Drop` has no returned future at all ([writer/🦀️.rs:40-42](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:40>)).

The real refusal P0 is cross-key: `request_controller` faults **every** active requested cell for the backend ([release/🦀️.rs:243-252](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:243>)). A different release B may already have installed `cell.waiter`; its next poll would return its retained error ([release/🦀️.rs:124-143](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:124>)), but refusal creates no follow-up controller turn and takes/wakes no B waiter. The caller that caused refusal—A's first `release`, A's `retry`, or a permit drop—does not need waking for itself; B does.

## Direct wake is not a safe caller variant

There is no sound rule of the form “drop the known DB mutexes, then call `notify_faults`” for public release requests. `WalWriterPermit` and `WalWriterRelease::retry` are public, and either may be invoked while an application-owned mutex is held. A synchronous custom `Waker::wake` can immediately try that mutex. The same is true for public `DbIoAsyncTaskLease::complete` and its destructor: both flow through `db_io_return_async_executor` ([storage/🦀️.rs:2908-2920](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2908>), [4064-4099](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4064>), [4120-4127](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4120>)). Its existing direct `notify_faults` call is consequently the same reentrancy class, despite releasing the backend registry first.

These are all current `request_controller` origins:

| Origin | Caller-controlled lock possible? | Required behaviour |
| --- | --- | --- |
| `WalWriterPermit::request_release` / public `release` / `Drop` ([writer/🦀️.rs:30-41](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:30>)) | Yes | Signal/latch only; never run an external waker. |
| public `WalWriterRelease::retry` ([release/🦀️.rs:114-120](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:114>)) | Yes | Signal/latch only; its old waiter was correctly discarded. |
| `WalWriterControllerWake::{wake,wake_by_ref}` ([release/🦀️.rs:219-224](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:219>)) | Yes: arbitrary executor waker context | Signal/latch only. |
| `WalWriterTable::finish_operation` ([writer/🦀️.rs:144-150](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:144>)) | Yes: called under the backend registry by `db_io_executor_close_operation` | Signal/latch only. |
| `db_io_backend_return_operation` ([storage/🦀️.rs:2878-2890](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2878>)) | Yes: page-admission rollback still owns the task arena at its call ([3409-3417](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3409>)); task close also owns its task slot/turn before its later explicit drop ([4518-4525](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4518>)). | Signal/latch only. |
| `db_io_return_async_executor` ([storage/🦀️.rs:2908-2920](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2908>)) | Yes: public completion/Drop above and async take rollback ([4239-4245](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4239>)). | Remove its direct writer-fault wake; signal/latch only. |

`controller_step` is the one currently valid direct-wake boundary: it has copied and released the controller row, returned from the backend-registry release call, and then calls `notify_terminal`/`notify_faults` ([release/🦀️.rs:255-270](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:255>)). It is an independent worker-pool callback, not the public caller that changed the signal. But it cannot service a `Shutdown`, `Stale`, or `Closed` request, since no controller invocation is then admitted.

## Small coherent repair: bounded pool-owned deferred-wake work

Keep `request_controller` non-waking. First copy `(Arc<WorkerPool>, WorkerMaintenanceTicket)` out of its controller row and drop that mutex before requesting the maintenance hook. On refusal, atomically:

1. Store the immutable refusal error in every exact active/requested/nonfaulted cell for this full backend control.
2. Move each already-present **other** `cell.waiter` into a pool-owned deferred-waker ring. Do not clone it, wake it, or clear `requested`/the guard.
3. Mark the ring runnable using a pool-internal work kind. The request path must only lock fixed state and issue the native pool idle signal; it must perform no user callback, `DbIoTask` submission, lost-owner registration, allocation, or I/O.

This should be a WorkerPool built-in `PoolWork::DeferredWake` class, not a `Job` and not another public maintenance hook. It is selected through the existing lane DRR with its own fixed ring, takes one waker, releases all pool/ring locks, and invokes `wake` inside `catch_unwind`. Native enqueue calls the existing internal idle condition signal; cooperative `has_pending_work` and `pump` must include it. This gives a later owner that cannot inherit the public caller's locks and needs no thread, task-arena slot, or closure allocation.

The ring must reserve enough entries for the real bounded worst case: `DB_IO_BACKEND_CONTROLS × WAL_WRITER_CAPACITY` = `64 × 32 = 2048` waiter owners if all writer controllers share one WorkerPool. One release signal cell owns at most one `waiter`, so a dedicated 2048-entry transfer ring cannot overflow under the current hard limits. If a future generic pool wake facility shares this ring, it needs a separate reserved WAL partition; falling back to synchronous wake when full would reintroduce this P0. Shutdown must drain/latch the same queue before worker exit, or explicitly retain an already-transferred waker in the per-cell fault state until the bounded owner can run—never silently discard it.

Existing terminal wake can remain in `controller_step`/backend-lane safe paths, but writer **fault** notification from public `db_io_return_async_executor` and every public request path should use this deferred work. This is a stricter rule than merely moving `notify_faults` after known internal mutex drops.

### Minimum executable laws

1. **Refusal wakes only later.** B is released and polled with a Waker that tries to lock an application mutex. While holding that mutex, release A after the controller hook is made stale/closed. Assert no B wake while A's call stack holds the mutex; run the pool's deferred-wake turn after unlocking; assert one B wake and B then returns its retained failure. A's first future observes its fault without needing a pre-existing wake.
2. **Retry supersedes only itself.** Start with an A failure and retained A release, then have B already pending. `A.retry()` clears A's old waiter before a second refusal. B's old waiter wakes once through the deferred ring; A's old waiter never wakes. Both table entries/guards remain retained and both future errors keep distinct retry owners.
3. **Pool shutdown and capacity are fail-closed.** Fill all 64 ordinary maintenance hooks, then exercise a writer controller hook's `Closed`/`Stale` request. No new `Job`, DB task, or lost-owner item appears; the dedicated deferred waker work still drains exactly the parked B waker. A pool shutdown drains the deferred ring before the native worker terminates, while the cells/table guards remain fail-closed for explicit backend close.

## Current static API migration audit

This is source-only, not a compile claim. The first-party mutable WAL trait is consistently writer-stamped: all six mutation methods take `&WalWriterPermit` at [storage/🦀️.rs:4661-4702](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4661>). `WalRef`, Memory, FS, FaultStorage, SQLite, Postgres, and Neo4j currently implement that shape; the external adapters' headers are at [SQLite:761-806](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:761>), [Postgres:929-972](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs:929>), and [Neo4j:1086-1129](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs:1086>). Production mutation callers found by static search pass a permit: `ArtifactWal` recovery/rotation ([wal/🦀️.rs:2118-2471](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2118>)), compaction retention ([compact/🦀️.rs:177-186](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:177>), CLI fixtures, and cluster fixtures. I found no remaining first-party call of the old unpermitted mutation form.

One current compile-style issue is concrete in the new controller test helper:

```rust
fn writer_controller_law_table<T>(
    control: DbIoBackendControl,
    action: impl FnOnce(&mut WalWriterTable<WriterControllerLawGuard>) -> T,
) -> T {
    // ...
    let result = action(lock(&executor.table).as_mut().unwrap());
    result
}
```

at [storage/🦀️.rs:7794-7800](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7794>). `T` can borrow the table argument, so the temporary `MutexGuard` cannot be proven dead before the generic `result` is returned (the reported E0597 shape). The narrow test-only correction is to require `T: 'static`, bind a named table guard and result, and explicitly drop the table guard before returning:

```rust
fn writer_controller_law_table<T: 'static>(
    control: DbIoBackendControl,
    action: impl for<'a> FnOnce(&'a mut WalWriterTable<WriterControllerLawGuard>) -> T,
) -> T {
    let mut registry = lock(db_io_backend_registry());
    let owner = &mut registry.slots[usize::from(db_io_backend_parts(control).0)];
    let executor = owner.executor.as_mut().unwrap().as_any_mut()
        .downcast_mut::<WriterControllerLawExecutor>().unwrap();
    let mut table = lock(&executor.table);
    let result = action(table.as_mut().unwrap());
    drop(table);
    result
}
```

Every current helper use returns owned permits, releases, tuples, or `()`, so the static-output restriction matches its intended test-only contract. It also preserves the deliberate short registry hold; no async operation occurs inside `action`.

No build result is asserted.
