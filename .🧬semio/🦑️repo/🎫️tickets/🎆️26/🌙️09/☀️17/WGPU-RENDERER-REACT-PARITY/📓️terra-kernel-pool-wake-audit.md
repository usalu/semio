# Kernel Pool Wake Audit

Read-only source audit on 2026-09-20. No build, test, activation, or production edit was run.

## KernelPoolFuture repair

**Verified: the changed extraction fixes the reported deadlock.** `KernelPoolFuture::run_turn` now moves the future into a local before the `if let` at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:8173-8181`. The mutex guard therefore ends at the extraction statement, before `Future::poll`, and the Pending branch can re-lock to restore the future.

This precisely removes the old self-deadlock: the previous `if let Some(mut future) = self.future.lock(...).take()` kept its temporary guard through the branch, then a Pending result tried to lock `self.future` again to restore it. The reported ProgramBridge path reaches `KernelPoolFuture` through `KernelClient::get` at `renderer/🦀️.rs:5374-5380` and parks while the pool future waits for kernel work, so a first Pending turn was sufficient to freeze the request service.

No remaining lost wake was found in the `KernelPoolFuture` handoff itself. A wake during polling or while the future is absent sets `notified=true` at `8164-8170`; the current turn restores the future, clears `scheduled`, then observes `notified` and submits the successor at `8179-8185`. A wake after `scheduled` becomes false submits the successor itself. These two cases cover both sides of the re-arm boundary.

Confidence: high.

## New proven response-slot lost wake

**P0 — `KernelFuture` can sleep forever after its kernel outcome was delivered.** `KernelFuture::poll` first checks and drops `slot.result` at `renderer/🦀️.rs:5344-5348`, then registers `slot.waker` at `5349`. `ResponseSlot::deliver` writes the outcome and takes/wakes the waker at `5316-5321`.

The following legal interleaving loses the only notification:

1. Consumer locks `result`, finds `None`, and drops that lock (`5344-5348`).
2. Kernel worker calls `deliver`, stores `Some(outcome)`, finds no registered waker, and returns (`5316-5321`).
3. Consumer stores its waker (`5349`) and returns `Poll::Pending` (`5350`).

The response is now ready but there is no future producer event to wake the parked caller. This applies directly to the `foreign.controller` ProgramBridge case: the caller uses `semio_framework_async::block_on` in `🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:90-111`, whose native path parks until its waker fires (`🧰️framework/🔨️modules/⏳️async/🦀️.rs:510-533`).

**Required repair:** make response and waker registration one atomic ownership protocol. A single `Mutex<ResponseSlotState { result, waker }>` is the clearest form. If the two mutexes remain, consumer registration must re-check `result` after storing its waker and return Ready when the producer won the original check/register gap; producer must continue to take the registered waker after publishing. Do not rely on an unrelated later kernel request to rescue the parked caller.

**Required test:** add a deterministic test hook/barrier immediately after the consumer observes empty `result` and before it stores its waker. Deliver an outcome in that gap, release the consumer, then require either Ready from that poll or a recorded wake followed by Ready. A timeout-only integration test cannot force this interleaving.

Confidence: high.

## Fixture assessment

The new neutral fixture at `🧪️fixtures/🧵️kernel-pool-future/🔣️.json` has an immediate-ready case and two self-woken Pending polls. The semantic-document test at `🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs:1116-1147` first verifies the exact trace with a current-thread Tokio runtime and then requires `KernelPoolFuture` to emit the same trace within two seconds.

This does test the actual lock-scope defect: its first self-woken Pending poll reaches the old branch's restore/re-lock, which deadlocks before the sender produces the trace. It also verifies a successor turn is admitted after a synchronous wake. It does **not** exercise the separate producer-delivery/consumer-registration interleaving above, because its future wakes itself while being polled and has no `ResponseSlot` producer.

The schema file is not executed by this test: `🧬️schema/🧵️kernel-pool-future/🔣️.json` is not referenced outside its own file, while the test parses the fixture directly as `serde_json::Value` (`1132-1136`). This is a fixture-validation gap, not the source of the native deadlock.

Post-audit correction: `🗑️generated/astra-runtime/kernel-pool-fixture-schema-2.log` now records a separate strict AJV pass accepting both neutral traces. The semantic-document test itself still parses direct JSON rather than invoking the schema; the separate fixture gate closes the validation gap. This does not affect the ResponseSlot lost-wake finding.

## Nearby ownership audit

- `ShellPoolFuture` already uses the safe local extraction pattern before polling at `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1453-1465`; no equivalent self-relock is present there.
- `KernelRequestQueue::poll` registers `consumer_waker` while holding the same queue mutex that `try_push` uses to admit work and take the consumer waker (`renderer/🦀️.rs:7979-8004`, `8007-8028`). Its check/register transition has no corresponding gap.
- `ResponseSlot::deliver` still calls arbitrary `Waker::wake` in the lexical scope of its `waker` mutex (`5318-5320`). The current ProgramBridge caller's thread waker only unparks, so this audit does not claim a reproducing reentrant deadlock there. Dropping that guard before invoking the external wake remains the correct lock-ownership cleanup when the response-slot repair is made.

## ResponseSlot repair review

**Verified in source: the single-state-mutex repair closes the reported check/register lost wake.** `ResponseSlot` now owns `result` and `waker` under one `Mutex<ResponseState>` at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:5309-5318`. `deliver` stores the result and takes the waker while holding that one state mutex, then wakes after the guard is gone (`5321-5329`). `KernelFuture::poll` clones the caller waker before the state lock, then either consumes a delivered result or replaces the registered waker while still holding that same lock (`5342-5365`).

The former lost-wake interleaving is no longer possible. Delivery before the lock is acquired leaves a result that the poll consumes as Ready; delivery after the poll chooses Pending takes the installed waker and wakes it. External `Waker` clone, drop, and wake work are all outside `ResponseState`'s lock. This also resolves the earlier reentrant-wake cleanup note at line 47.

`kernel_response_delivery_never_parks_without_a_wake` directly targets the race at `🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs:1150-1236`. Its custom `RawWaker::clone` performs the one scheduled delivery exactly while `KernelFuture::poll` clones the waker (`1160-1167`), which is the old check/register window. The `register-waker` fixture case (`🧫️fixtures/🧵️kernel-pool-future/🔣️.json:20-35`) would leave the old implementation Pending with zero wakes; the test requires either Ready or an observed wake followed by Ready (`1197-1210`). The same three timing traces are compared to a Tokio oneshot oracle (`1213-1234`), and the schema enumerates the three allowed timings (`🧬️schema/🧵️kernel-pool-future/🔣️.json:46-68`).

The test is a precise synchronization proof for the shared state protocol. It samples `KernelOutcome::Created`; all outcome variants use this same `ResponseSlot` ownership path, so no distinct response registration mechanism remains to audit. This audit did not run the test. Parent separately reported a pre-fix red reproduction and a later fixture-schema gate; neither is presented here as an independent execution result.

Confidence: high.
