# Render Pre-Submit Input Stall Audit

**Scope.** Read-only audit of the WGPU checkpoint-19 stall reported as `phase=Render engine=0 upload=2 gpu-cursor=None upload-progress=(0,0,0)`. [The retained receipt](🗑️generated/astra-runtime/checkpoint19-iab/wgpu-render-stall.json) confirms the worker fault at `2026-09-21T16:49:39.467Z`; its accessibility snapshot says the worker was not terminated and input was no longer accepted. It has no internal presenter trace, so the exact branch remains source-derived. No build or test was run.

## What the observed phase proves

The cursor reaches `Render` only after the engine and upload phases and after `Stage` has retained a presenter witness. In [the renderer](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs), the `Render` branch begins GPU submission only after it has made the staged input candidate ready. Until then it has no `gpu_cursor`; the reported shape is therefore pre-submit, not an upload or GPU-progress stall.

The three pre-submit waits are distinct:

1. `RuntimeMailbox::try_lock` can refuse the runtime mutex.
2. The mutex can be available while `AppRuntime.interaction` is `None`, because an async dispatch or deferred action owns the interaction state.
3. `ShellState::progress_presented_input_candidate` can report a matching scene intent pending.

The input candidate is not acknowledged in any of those paths. The only promotion is Shell acknowledgement, which swaps the staged hit/scene/widget/geometry registries and calls `InputState::publish_hits`. A pre-submit timeout must therefore abort and discard the exact candidate; it must never publish it.

## Confirmed reachable deadlock

This is the concrete liveness cycle:

1. [Winit `OsHost::build_and_publish_snapshot`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs#L240) deliberately does **not** call `RuntimeMailbox::pump_pending_applies` while the presenter holds a staged input candidate (`Render`, `CloseGpu`, or `Acknowledge`; renderer lines 14895–14897).
2. A normal `DispatchEvents` or `FrameDeferred` apply checks out `AppInteractionState` and sends it to an async task. Its terminal completion is a `ResumeDispatch` or `ResumeFrameDeferred` that is the only owner able to return that state (renderer lines 10459–10476 and 10529–10600).
3. `RuntimeMailboxInner::finish` enqueues that completion and invokes the host waker (renderer lines 11301–11308), but the resulting redraw still skips the normal apply pump because the candidate is held.
4. `Render` then sees `interaction=None` and returns pending without a GPU cursor (renderer lines 15326–15338). The queued completion that could return the interaction remains blocked by step 1.

This is sufficient to produce the reported stable `Render/engine=0/upload=2/gpu-cursor=None` signature. The available runtime shape does not identify whether the checkpoint hit this cycle, a direct mutex wait, or a scene intent; it does establish that all three need different liveness treatment.

## Bounded repair

Keep the existing prohibition on arbitrary mailbox mutation while a candidate is sealed. Add one candidate-safe mailbox operation with this contract:

* It takes at most one **ready completion whose `RuntimeApply::restores_interaction()` is true**.
* It calls `RuntimeApply::restore_interaction_only` while holding the runtime mutex, returning the exact `AppInteractionState`.
* If a dispatch/deferred cursor remains, it restores the now interaction-less completion to its original mailbox position. It does not call `apply_step`, `start_dispatch`, or `start_frame_deferred` until the candidate has either acknowledged or been discarded.
* `RestoreInteraction` has no remaining cursor and may be removed after the return.

The new `RuntimeApply::restore_interaction_only` helper at renderer lines 10448–10466 is the correct narrow primitive. It needs this mailbox caller; defining the helper alone does not break the cycle.

Use a distinct presenter result such as `AwaitingInteractionReturn` only when an interaction checkout has a returning completion or reservation. The Winit loop may return without scheduling a rAF in that case because `RuntimeMailboxInner::finish` owns the wake.

Do **not** use that no-retry result for `try_lock` failure. A standard mutex guard drop has no host-waker callback in the current source. Direct mutex contention must remain a normal `Pending` result with the existing `RESOURCE_READY` retry and a `RuntimeLock` watchdog reason. Treating both waits as `AwaitingRuntime` would introduce a lost-wake park.

The current concurrent implementation already adds `AppPresentInputWait`, `input_progress`, and an `AwaitingRuntime` enum member. At audit time, the Winit match has no `AwaitingRuntime` arm and the Render/Acknowledge branches still match the earlier three-state input result, so these edits must be integrated coherently before compiling.

## Scene-intent and watchdog law

`SceneIntentQueue::blocks_presented_candidate` blocks only an intent whose window is sealed for the exact candidate and whose tree and surface generations still match ([Interpreter lines 534–539](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs#L534)). It is not a global scene-queue barrier. `progress_presented_input_candidate` drives one exact terminal or interaction step before returning pending (lines 2104–2118).

The present watchdog must include both the wait reason and a monotonic input-progress witness. Increment it only when that single interpreter drive reports actual advancement; a pending answer with no advancement remains stall-detectable. Reset `input_wait` to `None` when the candidate becomes ready or is aborted. Apply this in both `Render` and `Acknowledge`: the latter runs after GPU submission and must keep its normal continuation, while the former is the only place a missing interaction may park on the return completion.

## Close interaction and safe invalidation

Window close does not justify clearing every staged candidate:

* `request_ui_document_close` records the exact surface token and generation, then marks that window closing ([Interpreter lines 322–335](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs#L322)).
* The close lane retires one focused/scene/input owner at a time before it releases the UI surface (lines 346 onward). A scene intent for the closing generation is retired instead of dispatched.
* `candidate_is_sealed_for` excludes closing windows ([UI engine lines 1136–1141](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs#L1136)), so that window’s intent no longer holds the candidate barrier.
* Shell advances close during `ShellChromeFramePhase::CloseDocument` (Shell lines 22735–22740), which belongs to a later frame build; a pending presenter prevents admitting that build.

Consequently, the safe sequencing is: first return the checked-out interaction or finish the exact candidate barrier; then the next frame build drives the exact close owner. If the candidate becomes stale before GPU submission, move it to `Aborted`, abort the gate, and use `discard_presented_input_candidate`. Do not acknowledge, publish hits, or synthesize an action for the closing surface.

## Fail-first laws

Extend the existing native helper and candidate-retirement coverage in [`wgpu-frame-job-unit`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs#L115):

1. Seal a real candidate, check out the interaction through a `DispatchEvents` or deferred completion, and finish that completion while the presenter is at pre-submit Render.
2. Drive the new return-only operation once. Assert the interaction is back, the candidate still matches, no staged hit registry was published, and the dispatch/deferred cursor is retained but has not run another event.
3. Drive Render to the first GPU cursor, then acknowledge. Only after acknowledgement may normal mailbox pumping start the retained cursor’s next event.
4. Cover direct runtime mutex contention separately: it must return `Pending` with a `RuntimeLock` signature and retry; it must not rely on a nonexistent mutex-unlock wake.
5. Cover a matching Ink/scene intent that advances once per opportunity and one that returns pending without advancement. The first must move the watchdog witness; the second must remain faultable. Add the same row to the neutral `presented-input-authority` fixture if its schema accepts the progress witness.

**Confidence:** high for the mailbox/candidate cycle and no-unlock-wake distinction; medium for the checkpoint’s exact immediate trigger because the receipt contains the terminal phase shape but no presenter branch trace.
