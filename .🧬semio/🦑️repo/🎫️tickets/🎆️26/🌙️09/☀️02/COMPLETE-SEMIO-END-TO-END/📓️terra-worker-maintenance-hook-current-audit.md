# Terra Worker Maintenance Hook Current Audit

Read-only audit, 2026-09-05. No build or native execution was run. This starts from the fixture and source gate visible before the WorkerPool implementation lands; an implementation-specific addendum follows once the source is present.

## Current source gate status

`🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📜️script.ts:10-35` evaluates an independent JavaScript `Map` model and then requires `../../🔔️maintenance/🦀️.rs`. Resolved from the package root, that is the new module path `🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs`, not the existing async root source. At first inspection that module was absent, so the source gate intentionally RED after its independent AJV/lifecycle oracle. It now exists; this audit makes no fresh source-gate or native-pass claim.

When the module lands, strengthen the gate beyond existence with the actual public names and selection seams: fixed capacity, full ticket fields (`pool`, slot, generation), fixed callback signature, install/request/begin-remove/finish APIs, native and wasm `PoolWork::{Job,Maintenance}` selection, and the four exact Rust law names. The native target is a necessary second proof, but it is not a substitute for a source gate that rejects a missing integration before an expensive compile.

## Required implementation invariants

### Ticket and slot lifecycle

The registry needs one lock-protected transition for every exact ticket:

```text
Vacant --install(full pool identity, generation, lane, callback, args)--> Idle
Idle --request(exact ticket)--> Requested
Requested --take--> Running
Running --finish(Idle)--> Idle or Requested (if concurrently requested)
Running --finish(More)--> Requested
{Idle,Requested} --begin_remove--> Removed
Running --begin_remove--> ClosingRunning --finish(*)--> ClosingIdle --next remove--> Removed
```

`begin_remove` must first set the closing bit and clear the queued request while holding the registry lock. A later request of that exact ticket returns `Closed`; a stale or different-pool ticket returns `Stale`; neither may touch the replacement slot. It may recycle the slot only after a retained caller's later exact remove observes that the running callback completed. The unique pool identity must be a checked, never-wrapping process allocation and be present in the ticket comparison—not merely an `Arc::ptr_eq` convenience used by callers.

The callback must run after its `Running` state is committed and **without** the registry mutex. Completion must reacquire the same registry mutex and preserve a request made during invocation: `Idle` clears only the callback's own running state; `More` requests another turn; `Fault` clears running but must leave a concurrent request pending unless the documented fault policy explicitly closes the hook. `ClosingRunning` wins over either `More` or a concurrent request and becomes `ClosingIdle`; only the caller's later exact remove releases the slot.

### Panic and shutdown

The existing native loop catches a selected `Job` only outside the job closure (`🧰️framework/🔨️modules/⏳️async/🦀️.rs:1658-1673`). A maintenance callback cannot rely on that outer catch: a panic must still execute the running-to-terminal completion transition, otherwise its slot remains `Running` forever and `begin_remove` waits forever. The `PoolWork::Maintenance` branch must catch its callback internally and finish it as `Fault`, or own an RAII turn guard whose `Drop` marks the exact running ticket fault/idle. That guard must not hold the registry mutex while user callback code runs.

Request must linearize with shutdown. After shutdown wins, it must return a non-success status and leave no requested bit that has no worker/pump to consume it. Shutdown must also make outstanding requested hooks non-runnable; later `begin_remove` must be able to retire an idle requested hook synchronously. A running native callback is allowed to finish before `shutdown` joins, but its finish may not re-arm itself.

### Scheduler and lock order

The native scheduler currently takes a per-worker lane-queue mutex in `select_and_pop` (`:1600-1627`) and the wasm scheduler holds `SchedulerState` while choosing a `Job` (`:1906-1927`). Do not select a hook while holding either job-queue/state mutex if request/remove can take the hook registry and notify/pump; choose/release the job lock first, then take one hook-registry turn. The callback runs under neither lock.

Treat a hook as a preallocated `Lane` work class, not a queue entry. It must check out the same worker permit and low-priority admission as a `Lane::Io` job, but it must consume no `Job`, `Box`, timer, or lane-queue capacity. Per-lane job/hook alternation and a rotating hook-slot cursor are both needed: a continuously ready Io job cannot starve a release hook, and a permanently `More` hook cannot starve normal Io jobs or another requested hook. If the selected turn points to a now-removed or already-running hook, retry boundedly or select another class; do not spin one worker while retaining a permit.

Native idle wake may use the existing condvar notification (`:1574-1576`) and its bounded idle recheck (`:1662-1680`) only after the requested bit is committed. wasm has no independent thread: `request` only establishes pending work, and `pump` must include requested hooks in both selection and its boolean “more work” result (`:2081-2118`). Native and wasm must share the same ticket/registry state machine; they may differ only in the external wake mechanism.

## Fixture/test gaps to close

The current corpus at `🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🧪️fixtures/🔣️.json` usefully covers duplicate request, remove-while-running, a request concurrent with a running callback, and stale logical owner A after B. It does **not** prove physical-slot reuse/generation, foreign pool identity, capacity exhaustion, shutdown, callback panic, a `More` callback racing removal, or lane/job fairness. Add at least these native/cooperative laws alongside the registered four:

1. `worker_maintenance_panic_finishes_exact_running_ticket_and_unblocks_remove` — a panic becomes `Fault`; it leaves no forever-running slot and no registry lock held.
2. `worker_maintenance_shutdown_rejects_request_and_retires_requested_idle_hook` — no callback is scheduled after shutdown, and removal remains terminal.
3. `worker_maintenance_slot_reuse_rejects_old_pool_slot_generation_ticket` — old ticket request/remove/finish cannot affect a newly installed hook in the exact same physical slot, including a second pool.
4. `worker_maintenance_io_job_hook_and_slot_rotation_are_boundedly_fair` — continuous Io jobs, two `More` hooks, and a `Fault` hook all advance through bounded alternating turns; each callback runs without a queued job or boxed closure.

The requested writer signal cell can safely use this primitive only after those invariants hold. Otherwise its nonallocating `Drop` request risks turning into either an idle-process permanent guard retention or a stale/recycled-slot release.

## Historical source review: fixed registry, before WorkerPool wiring

The new module is now at `🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:1-189`. It has a fixed 64-slot `[Option<Hook>; 64]`, full pool/slot/generation ticket comparison (`:6-12,80-83`), a checked never-reused registry generation (`:71-77`), and runs the callback outside the registry mutex while converting a panic to `Fault` before `finish` (`:53-63`). These are the right basic ownership properties.

`finish` also handles the important concurrent-request case correctly: `select` clears `requested` and marks `running` under the lock (`:108-123`); a concurrent `request` re-sets it; `Idle` does not clear that later request, while `More` re-sets it (`:125-132`). `closing` suppresses either outcome.

### Historical P0 (resolved below): shutdown was not represented by the registry

`WorkerMaintenanceError::Shutdown` exists (`:20`), but `WorkerMaintenanceRegistry` has neither a shutdown flag nor a shutdown transition (`:47-133`). Its direct `request` therefore cannot return it. The WorkerPool wrapper must make shutdown and request linearizable: after pool shutdown wins, reject the request before it changes registry state; shutdown must clear/cancel every queued hook request; and `begin_remove` must still retire an idle hook after shutdown. Do not merely rely on the native loop condition (`async/🦀️.rs:1662`) or wasm `pump` early return (`:2081-2085`), because either leaves a post-shutdown requested bit with no executor.

### Historical concern (resolved as a retained caller contract below): remove-while-running completion

`remove` marks `closing`, clears the queued request and returns `false` while an invocation is running (`maintenance/🦀️.rs:94-101`). `finish` clears `running` but deliberately leaves the closing entry allocated (`:125-132`); only a *second* `remove` frees the slot. The neutral trace models that extra call, but an actual backend-close or shutdown owner has no completion signal in this API telling it to make that call. In an otherwise idle pool, it can wait indefinitely even though the callback finished.

Choose one explicit retained contract before integrating it: either (a) `finish` removes an exact closing entry and provides a monotonic completion witness to the remover, or (b) rename the current method `begin_remove`, return a retained wait token/`Pending`, and make callback completion signal the owner/pool wake which drives a second bounded remove turn. Do not let a caller infer completion from `Closed`, and do not permit slot recycle before that exact running ticket has finished.

### P1: `Fault` is currently indistinguishable from idle to the registry

`WorkerMaintenanceStep::Fault` only avoids the `More` re-arm (`:125-132`); no fault is retained, surfaced, or wakes an owner. That is safe only if the static callback's `[u64; 2]` context has already stored an exact fault/retry condition in its own owner and will issue a later explicit request. Document/enforce that at the first writer-release integration. Otherwise a transient I/O fault becomes an idle hook with a still-owned file guard and no retry source.

At the time of this interim snapshot, the module was not yet included by native/wasm `WorkerPool`; the current wiring is reviewed in the following section.

## Wiring review: current native/wasm integration

The root now includes the registry, reexports the ticket API, and selects `PoolWork::{Job,Maintenance}` through both schedulers (`🧰️framework/🔨️modules/⏳️async/🦀️.rs:1449-1452,1606-1640,1673-1677,1933-1948,2117-2125`). This does not use a queue entry, `Box`, timer callback, or task arena for a request. Native `request_maintenance` also commits the registry request before `Condvar::notify_all` (`:1743-1748`), so its existing idle loop has a real wake source. The per-lane `hook_first` bit and slot cursor in `maintenance/🦀️.rs:40-45,108-123` give job/hook alternation and hook rotation. The callback is selected/running under the registry lock but executes after that lock is released; `PoolWork::run` catches its panic and calls `finish` (`:53-63`).

The observed lock ordering has no direct reverse acquisition in the current code: native scheduler holds a lane queue then observes/selects the maintenance registry (`async/🦀️.rs:1611-1628`); request/remove take only the registry and native request then only the idle condvar (`:1743-1753`). wasm holds scheduler state then observes/selects the registry (`:1933-1948`); its public maintenance method is still missing, so there is currently no registry-then-scheduler path. Keep it that way—callbacks must never call pool selection while retaining the registry mutex.

### Historical P0 (resolved below): native shutdown check and wasm API parity

Native `install_maintenance_hook` and `request_maintenance` do `is_shutdown()` before entering the registry (`async/🦀️.rs:1737-1748`). Shutdown can win immediately afterward; the method returns `Installed`/`Requested`, but the worker loop has exited and nothing drains the hook. `WorkerPool::shutdown` only stops workers/timers (`:1826-1837`) and does not close registry requests. This is the concrete post-shutdown stranded-request race identified above.

Put a `shutdown` bit in the registry state and an atomic `begin_shutdown` transition under its mutex. The pool shutdown path must close that registry before/with publishing process shutdown and clear requested hooks; install/request check that same bit under the same lock. Existing running turns can finish while native `shutdown` joins, but completion may not re-arm a closed registry. Removal remains allowed after shutdown so it can reclaim the exact idle slot.

The wasm `WorkerPool` has selected-hook support and includes hooks in `has_pending_work` (`:1933-1948,2117-2147`), but exposes none of native's `install_maintenance_hook`, `request_maintenance`, or `remove_maintenance_hook` methods. This is a public native/wasm contract mismatch; add the identical methods and shutdown semantics to the wasm impl. Its request must not fabricate a native wake: the next host `pump` is the admitted wake, and its return value must stay true while a requested hook remains.

### Historical concern (resolved as a retained caller contract below): running-hook removal

The integration does not change the module behavior: `remove_maintenance_hook` returns `false` for a running callback and never schedules its required second removal. This remains an observable liveness bug for an idle backend owner. Implement the explicit completion/wake contract described in the preceding section before the writer permit uses hook removal as its terminal witness.

### Historical P1 (resolved below): registered native laws were not yet present

The package script registers `worker_maintenance_native_idle_wake_uses_no_queued_job` and `worker_maintenance_cooperative_wake_obeys_pump_and_drr` (`async/📦️packages/🦀️rust/📜️script.ts:34`), but the new module presently defines only the first two selected laws (`maintenance/🦀️.rs:143-189`). The first native list pass should therefore remain RED until those actual native/cooperative tests are added. Also add the promised marker assertions to the non-native source gate; it currently only confirms that the new module file exists.

## Re-review after closed-registry and parity repair

The registry now has an in-lock `closed` bit. `install` and `request` check it while holding the exact state mutex (`🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:40-46,72-95`); `shutdown` closes the registry and clears every queued request (`:112-116`); `finish` suppresses `More` after closure (`:136-144`). Both pool shutdown paths close the registry before publishing pool shutdown (`🧰️framework/🔨️modules/⏳️async/🦀️.rs:1856-1864,2146-2152`), and wasm now exposes the same three hook methods (`:2038-2053`). This repairs the earlier stranded-request race: a request linearized before shutdown can report success but is synchronously cleared by the later close; a request linearized after it is rejected. No post-shutdown requested hook remains runnable.

The native idle and cooperative host-pump laws are now present at `async/🦀️.rs:1872-1900,2203-2231`. They prove a real native idle wake without queue ingress and two cooperative `pump` turns with no queued job. No fresh run was performed by this audit.

### Retained remove is sound, with a caller-owned next opportunity

The `remove(false)` behavior should remain. It atomically fences requests by setting `closing`, but retains the exact ticket/slot while its selected invocation runs (`maintenance/🦀️.rs:97-105`). Callback completion then clears `running` and cannot re-arm (`:136-144`); the owner’s next bounded close turn calls `remove` again and receives `true`, after which the ticket is stale. Auto-removal would make a close owner lose its exact terminal witness and permit premature slot reuse.

This is a coherent **retained caller** contract, not a generic awaitable primitive: `false` means `Pending`; the caller must retain the ticket, return to its own mounted close scheduler, and call `remove` again later. It must not busy-spin and must not treat `Closed` as completion. That matches the planned DB backend-close state machine, which already owns repeated bounded close turns.

### Remaining qualification gaps (P1)

1. The native idle law waits until both turns have already completed before removal (`async/🦀️.rs:1881-1893`), so it does not actually exercise `remove(false)`. Add a blocking callback law: remove returns false, a request returns `Closed`, callback finishes, the retained same ticket removes true exactly once, then request is stale.
2. Add shutdown-during-running-`More`: close the pool while the callback is held, release it, then assert it cannot re-arm and removal with the original ticket is terminal. This exercises the repaired closed-state edge rather than only install-after-shutdown.
3. The cooperative law has no queued Io job and the native law has no job/hook competition. Add two rotating `More` Io hooks plus an actual Io `Job`, and assert alternation/rotation under the same DRR permit accounting. This is the remaining evidence for the stated job/hook fairness, not just hook progress.
4. The non-native script still checks only the maintenance module's existence (`async/📦️packages/🦀️rust/📜️script.ts:31-33`). Add marker assertions for closed-state checks, pool identity/generation, `PoolWork`, both native/wasm hook methods, and the exact law selectors; otherwise a future source regression can make the lightweight gate misleading.

## Re-review: Callback-Owned `Retire` (2026-09-06)

### What is now sound

The callback-only retirement repair is correctly placed in the registry rather than recursively invoking `remove` under a running callback:

- `WorkerMaintenanceStep::Retire` is a distinct result at [maintenance lines 17-24](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:17).
- `PoolWork::run` invokes user callback code outside the registry mutex and hands its disposition to `finish` at [lines 84-95](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:84).
- `finish(Retire)` deletes only the exact ticket's slot after that invocation returns at [lines 164-177](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:164). This deliberately discards a concurrent `requested` bit, so a request issued while the callback runs cannot strand the former generation.
- The direct registry law requests the selected ticket while it is running, finishes `Retire`, then requires `Stale` and a new generation at [lines 271-284](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:271). The source-registered native 64-cycle capacity law is at [async lines 2152-2177](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:2152).
- Artifact authority uses `Retire` only in its committed maintenance callback after it has released the strong terminal job, the handoff ticket, and the fixed cursor/pool-use at [artifact lines 4065-4089](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4065). Its reservation `Drop` is the only external `remove`, and that happens before `commit`, when the callback cannot yet run ([lines 4114-4128](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4114)). Thus the original same-callback self-remove leak is repaired for the mounted authority owner.

### P0: panic between taking the cursor and restoring it loses the retained owner

The new terminal path has a separate owner-loss edge. `artifact_runner_retirement_step` takes the only `ArtifactRunnerRetirementCursor` from its fixed global row at [artifact lines 4072-4074](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4072), then invokes its close callback at [line 4075](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4075) without local unwind containment. `PoolWork::run` catches that panic outside the callback and records only `Fault` ([maintenance line 90](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:90). During stack unwinding, however, `cursor` is dropped:

1. the fixed global row remains `None`;
2. its `_pool_use` and strong runner/close owner are dropped;
3. `ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index]` is not cleared because the terminal branch was skipped;
4. the maintenance ticket becomes idle `Fault`, so it will not automatically call the now-empty row again.

That violates the retained-owner/fail-stop boundary: a callback panic can silently lose the only cleanup authority while permanently consuming the global generation slot.

The minimal correction is local to `artifact_runner_retirement_step`: wrap only `(owner.close)()` in `catch_unwind(AssertUnwindSafe(..))`; on `Err`, write the unchanged exact `cursor` back to the same `ARTIFACT_RUNNER_RETIREMENTS[index]` row before returning `WorkerMaintenanceStep::Fault`. Do not clear the generation, ticket, terminal job, or pool use on this branch, and do not turn it into `More` (that would spin a repeated panic). A later explicit recovery request can retry the retained cursor; absence of such recovery remains a visible fail-closed fault, not an owner loss.

Add an injected-close native law with this order: reserve/commit one cursor; cause its first `close` call to panic; wait for the callback turn; assert same fixed generation, cursor, strong pool use, and exact ticket remain; request a second turn after changing the injected close to terminal; require cursor/generation/ticket to disappear exactly once and `WorkerPool::shutdown()` to become terminal. The existing 65-drop capacity law does not cover this unwinding path.

### P1: public `Retire` versus external `remove(false)` needs one stated terminal rule

For the current ArtifactAuthority this race is excluded by the pre-commit-only external remove described above. For a future public callback, an external owner could call `remove(ticket)` while the callback is running (receiving `false`) and that callback could return `Retire`. `finish` will then erase the slot, and the external owner's later exact `remove(ticket)` returns `Stale`, not `true`. This is safe against ABA but not the old retained-remove acknowledgement shape.

Document `Retire` as **callback-owned terminal retirement**: a caller that delegates terminal cleanup to such a callback must treat later `Stale` as the callback's terminal witness and must not retain a second `remove` cursor. If a generic external close owner needs `true` acknowledgement, its callback must return `Idle`/`Fault` and preserve the existing `remove(false) -> later true` protocol. Add one direct registry law for this interleaving so the distinction cannot regress.

No native run was performed for this re-review. The source gate and registrations observed above are not a qualification receipt.

## Re-review: Artifact Retirement Panic Recovery (2026-09-06)

The prior P0 is fixed in the current source. `artifact_runner_retirement_step` now takes the exact fixed-row cursor, invokes only `owner.close()` inside a local `catch_unwind`, and restores the same `Option<ArtifactRunnerRetirementCursor>` to the same slot before returning `Fault` ([artifact](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4065)). Thus the outer WorkerPool catch never observes an ownerless row: the generation remains nonzero, the cursor retains the terminal job, strong runner, ticket, and `WorkerPoolUse`, and the registry leaves the hook faulted/idle until an explicit request.

The new law [artifact_runner_retirement_panic_retains_exact_cursor_until_explicit_retry](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:5011) exercises the required sequence: first call panics; same generation and row remain; pool shutdown sees its one retained use; an explicit exact-ticket request reaches a second terminal attempt; then shutdown succeeds. The 65-owner reuse law at [5058](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:5058) further checks that callback-owned `Retire` actually returns both the registry and fixed retirement capacities.

No new P0 is visible in this callback boundary. One P1 hardening case remains: the panic injection occurs before the close closure makes a resource-side effect. Add a second injected closure that records an intentionally idempotent pre-terminal transition and then panics, then verify the retained retry reaches terminal without duplicate close/release. The wrapper can restore its cursor but cannot itself roll back arbitrary side effects inside a future `close_one`; that contract must be idempotent/fail-closed in the close owner.

No build was run by this audit. WGPU reports source gates green and registers the panic law as mount law 20 and the reuse law as 21; those are source/owner reports, not an independent qualification claim.
