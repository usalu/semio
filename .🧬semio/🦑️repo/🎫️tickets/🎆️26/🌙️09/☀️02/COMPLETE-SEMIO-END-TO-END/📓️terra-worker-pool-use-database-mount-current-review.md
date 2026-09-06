# WorkerPool Use and Database Mount Current Review

## Outcome

The newly added `WorkerPoolUse` is correctly clone-shared: one database admission increments the pool lifecycle counter once, and the same `Arc<WorkerPoolUse>` is retained by the database, mount future, authority, and authority handoff without incrementing it again.  The mount driver is also now a genuine single-poller state machine: its `drive` path does not take `work`, `poll_once` alone changes `Queued -> Polling`, and completion materializes replies before it unlocks and wakes waiters.

There is one concrete P0 lifecycle escape.  A database that has reached terminal shutdown releases `pool_use`, but several public database operations can still submit work directly through `self.pool`.  If any unrelated retained use keeps that WorkerPool runnable, the terminal database can mutate storage or run a sync handshake.  This needs a shared open-state admission fence before further runtime qualification.

This was a source-only audit.  I did not run Cargo or modify product source.  The earlier mount receipt `WFy5o0/00` proves the preceding nine-law boundary only; it does not qualify the current WorkerPoolUse/hard-refusal additions.

## Confirmed Current Ownership

| Boundary | Current behavior | Assessment |
| --- | --- | --- |
| Pool lifecycle | Native and cooperative pools hold `Open { retained_uses }`, reject `acquire_use` after `Closing`, and only set `shutdown` after the count is zero ([async `🦀️.rs`](../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs) lines 1743–1754, 1905–1915, 1959–1979; mirrored at 2354–2365, 2489–2500, 2533–2550). `Drop` now uses checked subtraction, not a masking saturation. | Sound linearization for a normally retained cell. |
| Database opening | `Database::open_with` acquires its use before capability/catalog probes and only installs it in the returned `Database` after those probes have succeeded ([engine](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs) lines 7972–8050). Early rejection drops the local `Arc`. | Correct pre-probe fence. |
| Mount / authority | `mount_document` clones the database's cell before inserting the `Opening` slot (lines 8216–8285). `spawn_with_pool_use` places that exact `Arc` in both authority and runner handoff (artifact lines 4248–4272, 4310–4313); no second `acquire_use` occurs. | Correctly avoids both an early shutdown window and a count-per-handle leak. |
| Request cancellation | `DatabaseDocumentMountWait::drop` removes only its matching waiter generation/slot (engine lines 7464–7491). The owner stays in `Opening`; a later join or shutdown can drive it. | Correct ownership separation. |
| Sole poller and fanout | `request_drive` only marks atomics/queues a weak job; it never locks `work`. `poll_once` is the sole `Queued -> Polling` transition and checks a coalesced wake before returning idle (lines 7628–7809). `complete` swaps waiters and installs/removes the registry entry under the registry lock, then sends outside it (lines 7812–7849). | The RNAs9H work-mutex join deadlock is addressed in current source. |
| Hard refusal | `Shutdown`/`Poisoned` retains the exact queued closure, records its kind, and moves to `NonRunnable`; it does not create a retry timer (lines 7678–7700). `shutdown_step` observes that state and reports `DatabaseShutdownBlock::Executor` rather than claiming progress (lines 8368–8382). | Correct fail-closed behavior. In supported production, a live database's retained use prevents `Shutdown`; `Poisoned` requires an internal scheduler invariant breach. |

## P0 — Terminal Database APIs Bypass Its Lifecycle Fence

At the last successful shutdown phase, `Database::shutdown_step` does `self.pool_use.take()` and sets `shutdown_complete` ([engine](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs) lines 8423–8427). `mount_document` correctly starts with `self.pool_use.as_ref().ok_or(DbError::Closed)?` (line 8217), so `create_document`, `ensure_document`, `document`, and compaction through `document` fence after terminal acknowledgement.

Three still-live paths do not use that gate:

1. `create_document_catalog_retained` directly calls `DatabaseCreateCatalogFuture::try_submit(self.pool.clone(), self.catalog.clone(), self.storage.clone(), document)` (line 8307). It can durably change the catalog after the database claims terminal shutdown.
2. `hello_retained` directly calls `DatabaseSyncHelloFuture::try_submit(self.pool.clone(), self.storage.clone(), ...)` (lines 8493–8503), and `hello` exposes it at lines 8506–8523. It can read/serve a sync session after terminal shutdown.
3. `checkpoint_document` calls the version graph without an open-state check (lines 8525–8528). It must at least reject before attempting a post-terminal graph operation.

This is observable, not merely theoretical: retain a separate `pool.acquire_use()` before database shutdown, so the executor remains open after the database releases its own cell. Open and terminally shut down a database. Then call either `create_document_catalog_retained` or `hello_retained`. The current source can admit its retained worker; the pool's stopped state only hid this when the database was its sole use.

### Smallest coherent correction

Make a private `Database::require_open_use(&self) -> Result<Arc<WorkerPoolUse>, DbError>` the sole lifecycle admission. It must test `pool_use`/`shutdown_complete` together and return a clone only while live. Use it in every database-owned activity:

- `mount_document` (replace the inline check),
- `create_document_catalog_retained`,
- `hello_retained`/`hello`, and
- `checkpoint_document`.

The retained public APIs need a rejection type that can express `DbError::Closed` without fabricating a future.  Because this is greenfield, add a `Closed(DbError)` retained-rejection variant to `DatabaseCreateCatalogRejected` and `DatabaseSyncHelloRejected` (or make both public constructors return `Result<_, DbError>` and update their sole callers) rather than starting a worker and reporting a later failure.  Keep the normal, successful call holding the `Arc<WorkerPoolUse>` in its owner/future until exact terminal handback; a pre-admission closed error owns no storage/job to clean up.

`storage()` is intentionally a raw `Arc<DbBackend>` escape (engine lines 8457–8465). A previously issued `Arc` cannot be revoked by a database shutdown, so it cannot be made part of this fence without changing that API's domain boundary. Document it as an external storage capability and do not use it to justify post-terminal `Database` method calls. Its lack of lifecycle revocation is a separate capability-design decision, not a reason to leave the three direct operations unfenced.

### Required executable laws

1. Hold an unrelated `WorkerPoolUse`; terminally shut down `Database`; both retained constructors reject `Closed` before their storage/sync/catalog probes, and the held use still permits an independent known-safe job.
2. Under the same arrangement, `checkpoint_document` returns `Closed` without invoking the version graph.
3. Drop the unrelated use and verify `pool.shutdown()` succeeds; no rejected operation leaves a job, database catalog pending owner, sync owner, or writer owner behind.

## P1 — Qualification Gap, Not a New Source Bug

The registered source check lists the three new pool-use laws and the 14 mount laws ([async script](../../../../../../🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📜️script.ts) lines 168–186; [OS script](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts) lines 645–675). Current sources contain the right test names. The available report, however, explicitly says the current WorkerPoolUse/coalesced-driver/hard-nonrunnable set is native-pending. Do not infer a current native pass from the prior nine-law mount receipt.

After the P0 fence, the only remaining hard-refusal state is intentionally fail-closed: its exact weak-captured job and retained mount work remain private in `Opening`, and shutdown returns `DatabaseShutdownBlock::Executor`. That prevents a false cleanup/drop after a scheduler invariant breach. There is no evidence in current supported execution that this creates a second poller, an owner overwrite, or a retry timer; the `Shutdown` branch cannot normally occur while the database's retained use is live. Do not add an automatic retry or a raw job-extraction API merely to make that synthetic state terminate.

## Current Post-Terminal Fence Delta

The earlier database P0 is fixed in current source.  `Database::require_open_use` at [engine `🦀️.rs:8023`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L8023) is now called before mount (`:8291`), retained catalog creation (`:8385`), `hello_retained` (`:8580`), and `checkpoint_document` (`:8608`).  The new source law `database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack` at `:12578` holds an unrelated use, completes database shutdown, then observes `Closed` from all four paths while an unrelated permitted job still runs.  The registered native selector is [`db_engine::tests::database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack`](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts#L667).

That is a correct **Database facade** fence, not yet a global pool-lifecycle guarantee.  The following remaining direct owners can outlive a caller's pool lifecycle.

| Owner | Current source fact | Missing lifetime authority |
| --- | --- | --- |
| Registered DB I/O backend | `register_db_io_backend` and private `register_db_io_backend_reserved` reserve backing and retain only `Arc<WorkerPool>` in `DbIoBackendRegistrySlot` ([storage `🦀️.rs:2448-2465,2684-2748`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2448)). | No `WorkerPoolUse` lives from successful backend registration through terminal backend close.  A live Memory/FS/SQLite/Postgres/Neo4j storage facade therefore does not itself make `WorkerPool::shutdown` busy. |
| Raw task submission | Public `submit_db_io_task(pool: &WorkerPool, task: DbIoTask)` stores its caller-supplied pool in the task slot after separately admitting `task.backend()` ([storage `🦀️.rs:3375-3445,4154-4159`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L3375)). Every task carries a backend control, but the function never proves that this `pool` is the backend's registered pool. | A caller can register the backend on pool A and queue its task on unrelated pool B.  A backend-owned use on A would not protect B, and B can be shut down between allocation and submission. |
| Storage facades | Memory and FS have `{ control, pool, closed }` ([storage `🦀️.rs:6448-6512,7729-7773`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L6448)); SQLite, Postgres, and Neo4j have the equivalent shape and call the raw submission API.  Their `Drop` merely asks the backend registry to retire. | After pool stop, a facade can call `close` or issue a storage trait method.  The eventual task is rejected by a stopped pool, but the backend close owner/credit remains parked rather than being guaranteed runnable to terminal completion. |
| Standalone compaction future | Public `DatabaseCompactionFuture::try_submit` creates its retained state and schedules IO without `pool.acquire_use()` ([compaction `🦀️.rs:2158-2213`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs#L2158)). | Unlike `DatabaseSyncHelloFuture`, whose state owns `_pool_use` ([sync `🦀️.rs:1487-1493,1813-1878](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs#L1813)), direct compaction can race shutdown or be retained after caller drops the only external pool use. |

`DatabaseSyncHelloFuture` is **not** a remaining raw-lifecycle hole: it acquires a use before input admission at sync `:1835`, retains it in `DatabaseSyncHelloState`, and releases only with state retirement.  `DatabaseCapabilityOpenFuture`, catalog-read/bootstrap, and create-catalog retained states already follow the same pattern.  `handle_frontier_advertise` intentionally accepts a raw storage capability and has no pool argument; it becomes protected once its `DbBackend` has a registered-backend use.  That raw storage capability is still not revocable by a terminal `Database`, so it must not be used to claim facade-level revocation.

### Smallest Coherent Guard Placement

Do not add five separate facade guards or a use per `DbIoTask`.  Make the backend registry the single owner:

1. Acquire `Arc<WorkerPoolUse>` at the beginning of both public registration and the reserved registration path, before owner credit/slot mutation.  Store it in `DbIoBackendRegistrySlot` beside the exact `pool`.  The ownership is `backend registered -> close requested -> executor terminal -> writer controller terminal -> backend owner credit returned -> control slot erased -> pool use dropped`.
2. Extend every pre-registration/rejected/lost backend owner that may asynchronously close an executor (`DbIoRejectedBackendSlot` at storage `:2524`, `DbIoLostOwner::Backend` at `:3851`) with the same cell.  On registration failure, either the local cell drops when no executor needs retirement or it transfers intact with the exact executor/credit.  Never begin close after dropping the only lifecycle cell.
3. Remove the public pool argument from `submit_db_io_task`: `pub fn submit_db_io_task(task: DbIoTask) -> Result<DbIoTaskOperation, (DbError, DbIoTask)>`.  Every `DbIoTask` has a backend, so extend `db_io_backend_admit_operation` to atomically validate the full `{kind,slot,generation}` control, reject `close_requested`, increment `pending_operations`, and return the backend's registered `Arc<WorkerPool>`.  Store that returned pool in `DbIoTaskSlot`.  This removes cross-pool routing rather than trying to compare two caller-provided handles after separate locks.
4. When backend close frees the registry slot in `db_io_backend_close_lane_step` (`:2936-3011`), move the final use out, install the empty slot/free index, unlock the backend registry, then drop the use.  The use must remain while `close_fault`, retry, writer-controller hook removal, or owner-credit return is unresolved.  Do not drop it under the registry lock.
5. Add `_pool_use: Arc<WorkerPoolUse>` to `DatabaseCompactionState`; acquire it as the first operation in `DatabaseCompactionFuture::try_submit`, before the compaction admission slot or retained owners.  A stopped/closing pool must return its existing `DatabaseCompactionRejected` with all input owners untouched.  The held cell releases only when the compaction state reaches its existing terminal cleanup path.

This also establishes the correct parent for dropped task/result cleanup: `DbIoTaskOperation`, `DbIoAsyncTaskLease`, result leases, and writer-release controller work do not need their own independent cell as long as the matching backend control cannot retire/drop its registry cell until every `pending_operations`, executor lease, result handback, and controller close witness is terminal.

There is a closely related input-validation correction for the same extraction: current `db_io_backend_admit_operation` tests slot generation but does not compare the enum variant against `owner.kind` ([storage `🦀️.rs:2859-2875`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2859)).  The new atomic admission must require `db_io_backend_control(owner.kind, slot, generation) == task.backend()`.  Otherwise a public forged same-slot/generation control with a different backend kind can use a real executor and defeat the claimed exact-backend routing.

### Independent Fixture and Native Laws

Add a small schema-first `db-io-backend-pool-lifecycle-v1` fixture under the existing storage native-check owner.  It should describe only ownership states (`registered`, `task-active`, `retiring`, `terminal`) and expected `shutdown: busy|complete`; it must not reuse the Database facade fixture as an oracle.

1. **`db_io_registered_backend_use_blocks_pool_shutdown_until_terminal_close`**: register the existing in-memory law executor on one local pool.  `shutdown` is `Busy { retained_uses: 1 }`; raw task executes; request exact backend close; drive terminal close; then `shutdown == Ok(())`.  Also assert the writer-controller hook and backend owner credit are gone before the use drops.
2. **`db_io_task_uses_registered_backend_pool_not_caller_pool`**: two pools A/B; register on A; submit a task through the public API after its pool argument is removed; prove only A receives it and B can independently stop.  This is the regression for the current cross-pool escape.
3. **`db_io_backend_drop_retains_pool_until_deferred_close_terminal`**: drop a MemoryStorage (or the current blocking fixture backend) without an explicit close, force one close opportunity to yield/fault, and show pool shutdown stays busy until the exact backend controller resumes and empties.  It proves Drop did not discard the final use.
4. **`database_compaction_future_acquires_pool_use_before_admission`**: a stopped pool rejects before compaction slot/input movement; a live direct future makes shutdown busy across cancellation/retirement and releases only after its existing terminal witness.
5. **`db_io_forged_backend_kind_is_rejected_before_task_page_admission`**: alter just the kind for a real `(slot,generation)`, assert `StaleGeneration`/`Closed` before a writer/page changes phase and before IO scheduling.  This must be native because it verifies the Rust-public enum boundary.

Register storage laws with the existing OS Rust gate and compaction law with its existing `semio-framework-os-kernel-db` target.  The current worker-pool source checker should add markers for the backend slot's `_pool_use`, atomic backend-pool admission, and compaction state's `_pool_use`, but must not count only strings: the cross-pool native law is the actual proof.

## Current Backend-Guard Milestone: One Remaining P0 Rollback Escape

WGPU reports the central backend guard is now source-coherent: the backend registry retains `Arc<WorkerPoolUse>`, task admission derives and validates the exact registered pool/control, and direct compaction retains a use.  This review confirms the new registration shape at [`storage/🦀️.rs:2448-2465`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2448) and current task admission route at [`:3401-3441`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L3401).  It does not supersede executable qualification; WGPU reports its source oracle green only.

There is one concrete owner-loss path which prevents a full lifecycle claim.  On every registration rollback, `register_db_io_backend` and `register_db_io_backend_reserved_with_use` do:

```rust
let _ = db_io_park_lost_owner(DbIoLostOwner::Backend {
    owner: Some(executor), operation: owner_operation, credit: owner_credit,
    pool: Some(pool), pool_use: Some(pool_use),
});
```

at [`storage/🦀️.rs:2696-2701`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2696) and [`:2719-2741`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2719).  `db_io_park_lost_owner` can itself return its owner after all primary, overflow, and quarantine slots are occupied ([`:3893-3906`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L3893)).  Ignoring that return directly drops a potentially nonterminal executor, its reserved owner operation/credit, and the sole `WorkerPoolUse`.  The latter lets shutdown cease being busy even though the original backend cannot have received its required terminal close opportunity.

This is not a normal resource-pressure rejection: rollback has already consumed an executor plus owner credit, so it needs a retained rejection result.  The smallest coherent repair is either:

1. reserve an unrejectable backend-retirement owner cell before accepting/reserving the executor; or
2. change registration's failure result to own a `DbIoBackendRegistrationRejected { error, executor, operation, credit, pool, pool_use }` whose explicit retained `close_step` tries the existing rejected-backend path until it has transferred all six owners.

Do not return a bare `DbError` until that object has transferred/retired; do not drop the `pool_use` separately.  A capacity-native law must fill the three lost-owner tiers, force a registration failure after owner credit is reserved, and show that the rejected value still holds the identical executor/pool/use/operation, `pool.shutdown()` remains `Busy`, and only its terminal close step releases credit/use.  WGPU has acknowledged this P0 and is deliberately not making a global backend-lifecycle claim before it is resolved.
