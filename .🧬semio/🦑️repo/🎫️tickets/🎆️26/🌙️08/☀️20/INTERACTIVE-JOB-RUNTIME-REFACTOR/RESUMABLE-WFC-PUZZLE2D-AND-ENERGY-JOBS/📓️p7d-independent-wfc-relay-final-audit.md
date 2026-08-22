# P7d Independent WFC Relay Final Audit

Date: 2026-08-22  
Scope: Current Phase 7 Assembly/WFC production route, guest cold relay, and directly authored source tests.  
Method: Read-only static audit. No Cargo, Bun, build, test, runtime, Wasm, cache, target, ticket-status, or production-source mutation was performed.

## Verdict: REJECT

The repaired tree statically reconnects the exact `semio.infer` / `s.assembly.solve` path and removes the prior synchronous `block_on` relay. It also has a plausible retained-waker design for ordinary pending, wake, cancellation, and terminal cases. However, the retained-future panic/poison path can permanently lose the sole `GuestInstance`. This violates the required future-panic / semaphore-instance-loss recovery contract and makes later cold jobs deterministically fail on that handle. The existing channel-close branch reports a fault but does not recover the instance or schedule cleanup after terminalization.

The independent decision remains REJECT regardless of the otherwise-positive static evidence. All executable and Wasm gates remain unrun by instruction and cannot turn this source-only conclusion into acceptance.

## P0 — Caught Relay Panic Can Lose the Guest Instance Permanently

`run_guest_relay_request` acquires the semaphore, removes the instance from the shared slot, then awaits `start_job` or `step_job` with that instance only in its local stack frame. Restoration to the shared slot happens solely after the request match completes.

- [host component:2659-2677](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2659) acquires the permit and executes `slot.take()`.
- [host component:2679-2693](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2679) awaits the guest start/step future while the local `guest` owns the instance.
- [host component:2695-2700](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2695) is the only normal-path restoration and completion publication.

`GuestRelayPoolFuture::poll_once` catches an unwind, marks itself complete, and drops its retained future. It does not convert the panic to a completion, restore the instance, or schedule the one-shot cancellation path.

- [host component:2594-2609](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2594) catches `Future::poll` unwind; the `Err(_)` arm only marks complete and takes/drops the future.

If that future panics after `slot.take()`, unwind drops its local `GuestInstance`, the semaphore permit releases, and the shared slot remains `None`. A subsequent relay does acquire the permit but returns the explicit missing-instance fault at [host component:2666-2675](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2666). There is no re-instantiation or authoritative poison/recovery transition.

The sender is dropped when the retained future is dropped. `GuestColdRelayJob::step` recognizes the resulting closed receiver and terminalizes the relay as a fault at [host component:2754-2761](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2754). Its `Drop` only schedules cancellation when `!terminal_delivered` ([host component:2789-2795](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2789)); that channel-close fault sets `terminal_delivered`, so it prevents the only fallback cleanup. This is a concrete failure path, not merely an absent test.

## Static Findings That Pass Source Inspection

| Requirement | Evidence | Static result |
| --- | --- | --- |
| Exact public Assembly route and factory registration | Procedural `plugin()` registers the Assembly factory on the production ActionBus and freezes routed metadata at [procedural plugin:139-147](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🦀️component.rs:139). The factory fixes `semio.infer`, `s.assembly.solve`, and `s.assembly.inference.request.v1` at [Assembly inference:15-18](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:15), and uses ActionBus `register_once` at [Assembly inference:495-497](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:495). | PASS (static) |
| Factory-owned wire decode / restart checkpoint | The cold bridge resolves the exact key/schema and invokes `dispatch_wire` with separately carried restart bytes at [infer bridge:188-191](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:188). The Assembly factory decodes its owned payload and replaces only its checkpoint at [Assembly inference:486-491](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:486). | PASS (static) |
| One worker closure per `WorkerJobSession` call and no post-terminal re-entry | `WorkerJobSession::step` submits one closure, marks terminal once, and yields without re-entering after terminal at [job framework:766-786](../../../../../../../../../../../../🧰️framework/🔨️modules/🧵️job/🦀️component.rs:766). | PASS (static) |
| Retained-waker normal pending / lost-or-double-wake coalescing | A single scheduled flag gates pool submission, while a wake-request flag covers wakes during polling; `Pending` clears admission then reschedules only if necessary at [host component:2579-2604](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2579). `Wake` and `wake_by_ref` both enter that same gate at [host component:2614-2621](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2614). | PASS (static; runtime race unrun) |
| No worker held while a guest future is pending | The relay future is polled once per pool closure and returns on `Pending` at [host component:2590-2604](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2590). The authored one-worker competing-job source test is at [host component:2997-3031](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2997). | PASS (static; test unrun) |
| One guest admission per pending caller turn; ordinary cancellation and nonterminal drop | `GuestColdRelayJob::step` only `try_recv`s while pending and submits exactly one start or step otherwise at [host component:2746-2786](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2746). Start/step cancellation share an atomic cancel admission at [host component:2642-2645](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2642), and normal nonterminal drop schedules cleanup at [host component:2789-2795](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2789). Authored tests cover pending-step duplication, cancel race, and nonterminal drop at [host component:2997-3066](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2997). | PASS for normal source path; P0 exception above remains |
| Process-wide worker pool and timer wait | Host cold routes request the process singleton at [host component:239-242](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:239). `PeriodicPoolTimer` registers the deadline through `submit_at`, performs one tick, then registers another deadline at [host component:269-285](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:269). | PASS (static) |
| Live revision/generation validation immediately before exposure | Router admits a unique live identity, removes live authority after guest completion, and calls `validate_commit` before `Ok(result)` at [host component:4034-4056](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:4034); actor update handoff is [host component:4059-4067](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:4059). | PASS (static; integration test unrun) |
| Bounded bridge channels | Preview is latest-wins at 1 MiB [infer bridge:90-100](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:90); checkpoint/commit is lossless FIFO with two-item/2-MiB bound [infer bridge:103-120](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:103); diagnostics are a 32-item/64-KiB overwrite-oldest ring [infer bridge:122-143](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:122). | PASS (static) |
| Resumable deterministic WFC / checkpoint cap | WFC persists explicit stages [WFC job:23-50](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️component.rs:23), uses a one-word multiply-high range mapping [WFC job:129-135](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️component.rs:129), admits checkpoint/commit materialization to 1 MiB [WFC job:316-350](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️component.rs:316), and rejects restore bytes above 1 MiB [WFC job:1146-1150](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️component.rs:1146). | PASS (static; timing/determinism tests unrun) |

## Additional Static Risk: Commit Side Vector Is Not Pre-Reserved

`CommitBuild::new` reserves the serialized output but creates `assignment` as `Vec::new()` ([WFC job:341-350](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️component.rs:341)). `commit_one` pushes one assignment every step ([WFC job:1022-1041](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️component.rs:1022)), so capacity-growth reallocations remain possible during materialization. The byte envelope still caps node count indirectly, but the test only times construction/reservation ([WFC job:1861-1882](../../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️component.rs:1861)); it does not prove the worst capacity-growing `commit_one` step. This is a P1 boundedness/watchdog risk, not the basis for the P0 verdict.

## Required Repair and Acceptance Gates

1. Make panic, poisoned-future, and closed-channel handling preserve or deliberately retire/recreate the `GuestInstance`; do not mark terminal before the exact-once cancellation/recovery ownership is resolved. Add source/runtime regressions that panic after instance acquisition in both start and step, then prove a subsequent job does not observe a missing instance.
2. Exercise lost wake, duplicate wake, channel closure, semaphore wait, cancellation before/during start and step, terminal cancellation race, session abandonment, and retained-future panic on a one-worker process pool. Record completion, cancellation admission count, instance availability, and progress of a competing job.
3. Reserve or otherwise boundedly materialize the WFC commit-side assignment vector, then run worst-case commit materialization under allocation pressure with p99/max watchdog evidence.
4. Run focused relay/Assembly/WFC/bridge tests in debug and release; production native development, `-D warnings`, and release; exact public-factory replay with 1/2/4/default worker counts; mounted router freshness validation; and both `wasm32-unknown-unknown` and `wasm32-wasip2`.

## Explicitly Unrun Gates

No executable conclusion is claimed for compilation, authored tests, race behavior, watchdog timing, real pool replay, native strict-warning/release, mounted guest-host integration, `wasm32-unknown-unknown`, or `wasm32-wasip2`. The test sources cited above are reviewed only; they were not run in this audit lane.

## 2026-08-22 repair disposition

Status: **both cited source findings repaired; ready for independent source/static re-audit. The
historical REJECT above is preserved as the prior verdict until that re-audit.**

### P0 disposition — unwind-safe guest ownership and terminal cleanup

`run_guest_relay_request` no longer owns a bare `GuestInstance` local after slot acquisition.
`GuestInstanceLease` owns the taken value across all `start-job`, `step-job`, and `cancel-job`
suspension points and restores it from `Drop` through poison-tolerant mutex recovery. Because the
semaphore permit was acquired before the lease and is dropped after it, unwind restores ownership
before another request can acquire the permit.

`GuestRelayPoolFuture` retains `catch_unwind`, but its panic branch now takes and drops the poisoned
future before invoking recovery. That ordering runs the lease and permit destructors first. The
recovery handler then schedules guest cleanup through an atomic one-shot scheduling gate and sends
one typed `GuestRelayCompletion::Fault`; a shared one-shot sender prevents panic and normal paths
from both publishing. A separate atomic admission gate keeps `cancel-job` at exactly one call even
when in-flight cancellation, panic recovery, channel-close fallback, and relay `Drop` race.

The slot is now an explicit `Available` / `Leased` / `Quarantined` ownership state. If foreground
or background-cleanup `cancel-job` itself panics, the unwind lease first restores the instance and
the panic handler then moves it to `Quarantined`. Because the one cancel admission has already been
consumed, recovery does not retry cancellation and does not expose uncertain guest state to another
job. Subsequent mounted routes return the retained quarantine fault without waiting on the
semaphore or reusing the guest.

`GuestColdRelayJob` now separates terminal delivery from its cleanup obligation. Caller
cancellation with no in-flight request submits a foreground, panic-recoverable cancel request and
yields until it resolves, so a `cancel-job` panic becomes the terminal fault rather than a premature
`Cancelled`. Fault and closed-channel paths request cleanup before terminalization, and `Drop`
continues requesting cleanup whenever the guest has not reported `Done`, `Failed`, or successful
cancellation. Thus terminal bookkeeping cannot suppress required panic cleanup.

Authored regressions inject panics in mounted `start-job` and `step-job` after lease acquisition,
then assert a nonmissing slot, one cancellation, surviving worker progress, and a second mounted
`semio.infer` result on the same handle. A one-worker regression injects `cancel-job` panic and
asserts one fault followed by `Yield`, one cancel admission, quarantined nonmissing ownership,
released permit, worker survival, and prompt rejection of the next mounted route. A separate
mounted regression panics the background cleanup cancel and proves it also quarantines before the
next route. Another regression deliberately poisons the instance mutex and asserts the mounted
route still completes through poison recovery. These are source evidence only and were not run.

### P1 disposition — fixed/reserved lossless materialization

`CommitBuild::new` now computes byte capacity with checked arithmetic, rejects
`MAX_COMMIT_ITEMS + 1`, reserves the serialized byte vector and the complete `Vec<u32>` assignment
side vector with typed allocation faults, and records immutable byte/item limits. `commit_one`
checks both limits before every push/append and faults on overflow or mismatch. `CheckpointBuild`
uses checked arithmetic, exact reservation, and a byte-limit check before every header/domain/trail/
decision/observation append. The source regression fills the exact-maximum assignment without a
capacity change and rejects item maximum plus one; checkpoint exact maximum plus one is likewise
rejected.

The adjacent inference bridge no longer uses a growing `VecDeque` for its lossless checkpoint/
commit channel. It uses a fixed `[Option<_>; 2]` FIFO, checked byte addition, the existing 2-MiB
policy, and exact-max/+1 tests. Preview coalescing and diagnostic overwrite-ring policies are
unchanged.

### Allowed static evidence and open executable gates

- `rustfmt --edition 2021 --check` on the relay, bridge, and WFC leaves: exit 0.
- `bun ./📜️script.ts verify interactivity tool-jobs --format json`: exit 0; 775/775 bounded,
  zero batch-only/forbidden/deleted rows, one production factory/registration/dispatch, zero
  failures.
- Scoped `git diff --check`: exit 0. The production relay has zero `block_on`, private pool/thread,
  batch-driver, `mem::forget`, or `ManuallyDrop` hits. The repaired relay/bridge/WFC regions have
  zero temporary debug-output hits; two pre-existing actor logger `eprintln!` calls lie outside the
  relay region.
- Cargo, build, test, runtime, and Wasm commands were not run by instruction. No compile, runtime,
  race, watchdog, native, strict-warning, release, mounted-integration, worker-count replay,
  `wasm32-unknown-unknown`, or `wasm32-wasip2` pass is claimed.
