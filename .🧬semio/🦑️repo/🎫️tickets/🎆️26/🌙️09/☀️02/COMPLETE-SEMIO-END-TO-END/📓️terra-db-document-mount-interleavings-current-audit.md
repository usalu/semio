# DB Document-Mount Interleavings Current Audit

## Scope

Read-only review of the current retained document-mount owner after the `try_lock` drive and unlocked reply-fanout changes. No build was run. The parent reported the current ten neutral traces and nine selected native laws GREEN (`15440/WFy5o0`); that receipt qualifies those exact traces only.

The direct caller-side wait is gone: `drive` uses `try_lock` before scheduling ([engine:7613](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7613)). Completion takes the fixed waiter array and changes the generation-checked registry entry while locked, then calls the reply senders after the lock is released ([engine:7716](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7716)). Those are correct improvements.

## P0 — `scheduled` is not a single-poller permit

`schedule` admits a closure holding a strong owner reference ([engine:7604](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7604)). That closure clears `scheduled` *before* acquiring `work`, then blocks on the ordinary mutex ([engine:7657](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7657)). Thus the active mount poll can hold `work`, a first join schedules a second worker, that worker clears `scheduled` and blocks on `work`, and each later join schedules another blocking worker. The shared test pool has four workers, so three joiners can consume every remaining worker while the first owner is in a synchronous user hook. A release task placed on the same pool then cannot run.

This is not repaired by making only `drive` nonblocking: the contention has moved from the request thread to the executor. It also invalidates the claimed one-owner/one-poller property; the mutex serializes polls but does not bound parked worker threads.

Use a small owner-local driver state, not the `work` mutex, as the scheduling authority:

1. `request_drive(resume: bool)` records an atomic/coalesced `wake_requested` (and `resume_requested` when appropriate). It admits at most one `Queued` job only from `Idle`; it never takes `work`.
2. The job atomically changes `Queued -> Polling`. A job that finds `Polling` or a different state returns without taking `work`; only the holder of `Polling` may lock and poll it.
3. The poller releases `Polling` only after it rechecks the coalesced request bit. A request racing `Pending` is therefore either consumed by the active poller or admits the next job, never lost.
4. `Complete` makes the driver terminal before/with the existing generation-checked fanout. The job closure should carry a `Weak` owner; registry ownership is enough while `Opening` exists and prevents a rejected queue job from forming a self-cycle.

Add a native law with the existing four-worker `test_worker_pool`: hold the published-mount test hook inside the first poll, issue enough joins to exercise every available worker, then submit the hook-release through that same pool. It must run and lead to one `Ready` authority. The current release-from-test-thread law cannot expose executor starvation.

## P0 — a failed `try_lock` can lose the only cleanup-resume request

`drive` resumes `Parked` only when its initial `try_lock` succeeds ([engine:7613](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7613)). Its scheduled closure never calls `resume_parked`: a `Parked` item is put back unchanged and returns `Pending` ([engine:7698](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7698)).

The race is concrete:

1. The sole poller is polling `Cleanup` under `work`.
2. An uncancelled `shutdown_step` (or a joining request) calls `drive`; `try_lock` returns `WouldBlock`, and it only queues a poller.
3. The held cleanup poll returns `Err(rejected)`, so it writes `Parked` ([engine:7693](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7693)).
4. The queued poller sees `Parked`, restores it unchanged, and returns. No later wake is required, so the exact retained writer remains permanently parked despite the already-issued controlled resume.

Make `resume_requested` a coalesced owner field and consume it under `work` in the sole poller immediately before selecting the current work variant. `drive` must set the request before attempting/asking for the poller; it must not use direct mutation as the only delivery mechanism. This belongs in the same driver-state correction above.

Add a test-only controlled `retry_close` future: hold its terminal rejected result while another thread/request invokes `shutdown_step`, then release the failure. The already-issued request must produce exactly one second `retry_close` attempt, preserve the same retained rejection until that attempt succeeds, and finally remove the original generation. The current parked-cleanup law waits until it has observed `Parked` before calling shutdown, so it does not cover this interleaving.

## P0 — terminal pool admission is retried forever and can retain an Opening cycle

The mount owner treats `Shutdown` and `Poisoned` exactly like transient saturation: it stores the returned job and arms a timer ([engine:7633](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7633)). This differs from `WorkerPool`'s own retained-timer policy, which drops a job on those two terminal errors ([async:1844](../../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs:1844)). During `WorkerPool::shutdown`, shutdown is set before `fire_due`, then workers exit once deferred wake work is drained ([async:1685](../../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs:1685), [async:1910](../../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs:1910)). A retry callback that observes shutdown stores a new `Job` and re-arms a later timer that has no worker left to fire it. The `Job` captures an `Arc<Owner>`, so `Owner -> retry_job -> Job -> Owner` also survives after registry teardown.

The database accepts a shared `Arc<WorkerPool>` without enforcing that its worker lifetime outlives retained mount work. `shutdown_step` merely calls `owner.drive` and reports `Progress` ([engine:8249](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:8249)); after pool shutdown this cannot make progress or expose a truthful recovery owner.

Do not turn a hard scheduler loss into an ordinary retry or drop a `Cleanup`/`Parked` owner. The coherent fix is an executor-lifetime capability: `Database` retains a pool-use guard for every mounted owner, and worker-pool shutdown must reject/block until those guards have reached terminal retained cleanup. Separately, the mount scheduler must distinguish `Contended|Saturated` (coalesced retry) from `Shutdown|Poisoned` (no timer, no strong closure cycle; explicit non-runnable retained state). `Database::shutdown_step` must report that latter state as blocked/interrupted rather than a false `Progress` loop.

Add a native law that starts a controlled mount with its only waiter dropped, makes its next scheduler admission observe terminal pool shutdown, and proves: no timer/strong-job cycle remains, the `Opening` owner and any exact cleanup owner are still inspectable, and shutdown returns a bounded non-success status rather than spinning. A distinct construction-time pool lifetime law should show that a pool cannot terminally shut down before its `Database` releases the guard.

## No additional P0 found in the completion handoff

The current fixed 32-slot fanout is assembled under the registry lock and dispatched after it; the reply waker is not called under the registry mutex. The generation comparison prevents an obsolete completion from replacing a newer `Opening`. The remaining P0s are confined to owner drive/scheduling and terminal executor lifetime.
