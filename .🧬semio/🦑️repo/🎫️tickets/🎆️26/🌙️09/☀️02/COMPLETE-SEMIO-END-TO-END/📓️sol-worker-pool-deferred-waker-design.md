# Worker Pool Deferred Waker Design

## Status

Schema-first design and independent source oracle only. The WorkerPool and writer controller runtime are not changed or qualified by this slice. No Cargo process was launched.

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

## Required native laws after runtime landing

1. Cross-key refusal under an application mutex performs zero inline wakes; a later pool turn wakes B once, then B returns its retained failure while A and B guards remain mounted.
2. `A.retry()` discards only A's obsolete waiter. A second refusal moves B's existing waiter once; both failures preserve distinct retry owners and neither guard unlocks.
3. Filling all 64 maintenance hooks does not consume the separate wake partition. Closed/stale refusal still drains the parked waiter with zero jobs, DB tasks, lost owners, or I/O.
4. Filling all 2,048 reserved waiter cells drains exactly once during native and cooperative shutdown. After the terminal shutdown fence, a rejected enqueue restores its exact waiter and backend close remains required.

## Registered evidence

`@semio-tech/framework-async-rs:worker-deferred-wake-design-check` validates the strict JSON Schema with AJV, independently models all five refusal/shutdown cases and the 2,048-owner capacity equation, and checks the current async/DB bounds. It reports how many future runtime markers are present but deliberately does not treat their absence as runtime qualification.

Source receipt on 2026-09-05: GREEN via the registered Nx target with `AJV=1 cases=5 fixed-waiters=2048 inline-wakes=0 runtime-markers=0/8`. The zero runtime-marker count is the expected pre-implementation boundary, not a runtime pass.
