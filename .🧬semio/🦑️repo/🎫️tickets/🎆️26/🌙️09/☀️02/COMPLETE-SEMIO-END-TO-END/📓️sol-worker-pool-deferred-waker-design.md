# Worker Pool Deferred Waker Design

## Status

Runtime implementation, strict neutral fixture, mounted DB refusal law, registered source gate, and registered exact-native selectors are present. The source gate and all three pure async native laws are green. The mounted DB refusal law remains native-pending.

## Current refusal defect

`request_controller` retains the writer-controller row while it asks the maintenance registry. A refused request faults every exact active/requested/nonfaulted writer signal for that backend, but leaves previously installed cross-key waiters in their cells. Public permit release, retry, wake callbacks, async-executor return, allocation rollback, and task close can all originate the request while caller or registry locks exist. The current direct `notify_faults` calls therefore cannot form a general reentrancy-safe boundary.

## Fixed owner topology

Each WorkerPool owns a dedicated deferred-wake registry separate from the 64 ordinary maintenance hooks and job queues:

- 64 generation-qualified partitions, matching the maximum live DB backend controls that can share the pool.
- 32 waiter slots per partition, matching the fixed writer capacity of one backend.
- 2,048 total `Option<Waker>` owners, one for every possible writer signal waiter.
- One partition ticket retained in each writer controller beside its maintenance ticket.

Installing a writer controller reserves its deferred partition before publication. Enqueue moves, never clones, one exact waiter into that controller's partition. If a ticket is stale/closed, the partition is terminal, or its mathematically reserved capacity is unexpectedly unavailable, enqueue returns the same waker owner; the caller restores it to the exact faulted signal cell. There is no synchronous fallback wake.

## Refusal transition

`request_controller` copies `(pool, maintenance_ticket, deferred_wake_ticket)` and releases the controller row before calling `request_maintenance`. On refusal it visits only the full-backend active/requested/nonfaulted cells. For each cell it stores the immutable refusal fault, takes an existing waiter, and attempts a move into the fixed partition. A rejected move restores the exact waiter to that same cell. This path performs no waker callback, queued `Job`, closure allocation, DB task, lost-owner registration, I/O, or terminal epoch change.

The existing safe controller callback may still publish terminal notifications. Public request origins stop calling user wakers directly; they only signal the fixed deferred queue.

## Scheduling and locks

`PoolWork` gains a `DeferredWake(Waker)` kind on `Lane::Io`. Native and cooperative selectors include the deferred registry in their existing lane DRR and rotate within the lane across job, maintenance, and deferred-wake sources. A turn moves exactly one waker out, releases the maintenance, scheduler-lane, and deferred-ring locks, and invokes `wake` inside `catch_unwind`.

The enqueueing public call can still be inside an application-owned lock, but it never invokes the waker on that call stack. Native dispatch is an independent worker turn; cooperative dispatch occurs only on a later host `pump`.

## Shutdown

Shutdown first closes ordinary ingress and deferred-wake admission, then drains wakers accepted before that fence:

- Native workers exit only after the deferred ring is empty; shutdown signals the existing idle condition and joins after that drain.
- Cooperative `has_pending_work` remains true while the ring is nonempty, and each post-shutdown `pump` drains at most one entry until terminal empty.
- Enqueue after terminal shutdown returns the exact waker owner for restoration to its writer signal cell. Backend close remains the only later authority that can retire the guard and expose the retained fault/terminal state.

No queued waiter is silently discarded and shutdown does not fabricate writer completion.

## Runtime laws

1. Cross-key refusal under an application mutex performs zero inline wakes; a later pool turn wakes B once, then B returns its retained failure while A and B guards remain mounted.
2. `A.retry()` discards only A's obsolete waiter. A second refusal moves B's existing waiter once; both failures preserve distinct retry owners and neither guard unlocks.
3. Filling all 64 maintenance hooks does not consume the separate wake partition. Closed/stale refusal still drains the parked waiter with zero jobs, DB tasks, lost owners, or I/O.
4. Filling all 2,048 reserved waiter cells drains exactly once during native and cooperative shutdown. After the terminal shutdown fence, a rejected enqueue restores its exact waiter and backend close remains required.

The pure async laws cover the full 2,048-owner registry, generation fencing, shutdown drain, native no-inline dispatch, and cooperative later-pump drain. The mounted DB law uses a real registered backend on a dedicated one-worker pool: B is released and parked first; its maintenance ticket is retired to produce a generation-qualified stale refusal; A is publicly released while an application mutex is held. The law requires zero inline wakes, a still-Pending B while its exact slot is queued, one later wake after worker release, two distinct retained failures, both guards fenced, and explicit retries to terminal.

The retry-epoch admission is stronger than the raw 32-slot capacity equation. A faulted release remains `Pending` while `deferred_fault_waiter` is true and the exact pool slot is occupied. It can expose `Ready(Err(retained_owner))` only after selection has removed the old Waker from that slot. Therefore a caller cannot obtain and retry that owner before the prior epoch releases its exact slot; the maximum queued owners per signal remains one.

## Registered evidence

`@semio-tech/framework-async-rs:worker-deferred-wake-check` validates the strict JSON Schema with AJV, independently executes the hostile retry epoch, models all five refusal/shutdown cases and the 2,048-owner capacity equation, and requires all nine runtime markers. Missing runtime code now fails the target. `worker-deferred-wake-native-check` selects the three exact async runtime laws. The OS writer native group also selects `wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch`.

Source receipt on 2026-09-05: GREEN via the registered Nx target with `AJV=1 cases=5 fixed-waiters=2048 retry-epochs=1 max-queued-per-signal=1 inline-wakes=0 runtime-markers=9/9`. The registered OS writer source oracle also passed and requires that no public DB handback path names the direct `notify_faults` boundary.

Root's exact native run is GREEN3 at `worker-deferred-wake-exact/exact-cargo-laws-Pj2Y8N/00`, executable SHA-256 `cd3a8448cf9099e37da12a70cc59ec032b6929607825aa55cc14163a162be552`. It passed the fixed 2,048-owner capacity/generation law, native no-inline wake plus shutdown drain, and cooperative post-shutdown pump drain. It does not qualify the separately registered mounted DB refusal law.
