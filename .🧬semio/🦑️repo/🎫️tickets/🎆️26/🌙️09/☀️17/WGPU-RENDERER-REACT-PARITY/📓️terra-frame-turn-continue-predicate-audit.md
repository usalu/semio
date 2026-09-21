# Frame-Turn Continue Predicate Audit

**Scope.** Read-only audit of the current browser WGPU Worker continuation path. This supersedes an initial hypothesis that every live frame-build session is externally blocked; the source disproves that hypothesis.

## Exact scheduling contract

[`🌐️browser-worker/🦀️.rs`](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs) builds `continue_frame` after each renderer tick from cursor work, text work, World3d work, asset decode, settle, ready applies, a live build, or presenter work. It sets `request_frame = continue_frame || next_deadline.is_some()`; the deadline is deliberately excluded from `continue_frame`.

[`🧵️frame-turn-scheduler/🟦️.ts`](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-turn-scheduler/🟦️.ts) owns one MessagePort callback. It rearms only when `runFrameTurn` returns true. [`🎞️frame-worker/🟦️.ts`](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts) returns the Rust `continueFrame`, never `requestFrame`. This separation is correct: a scheduler deadline must remain page-rAF paced.

## Predicate classification

| Term | Current owner and evidence | Direct Worker turn |
| --- | --- | --- |
| cursor wake | `OsHost::take_cursor_wake_directive` consumes one retained token | Yes; consumption is local and finite. |
| World3d work | `world3d_cursor_work_pending` contains retained build, bridge, snapshot, draw rebuild, retirement, and camera-fit cursors | Yes; the frame transaction owns their bounded steps. |
| asset decode | `has_pending_asset_decode` includes only an active decode probe, a completed response, or World3d completed decode/announcement work | Yes. It deliberately excludes an in-flight browser fetch. |
| settle | `settle_pump_pending` names local deferred work, refresh, or a producer crossing | Yes for the admitting step. File-open and extension calls leave the interaction in a detached completion future rather than retaining a live frame build. |
| ready applies | `has_pending_applies` checks only the ready completion queue | Yes. |
| frame build | wasm `FrameBuildHandle::poll_runtime_and_resubmit` executes one synchronous owner turn using `try_step_on_worker`; `try_step_inline` returns `Outcome` or `Terminal` | Yes. A live session is not evidence of an external wait. |
| presenter | `AppPresenter::present_step` advances retained phase/retirement/gate state and has its own stall witness | Yes on this path; no browser Promise or callback is awaited in the reviewed presenter cursor. |
| text work | `AppInteractionState::has_pending_text_work` considers reservation bytes or any open stream runnable | **No, not in every state.** See next section. |

The external async boundary is correct for a file picker or async extension result: `RuntimeApply::start_frame_deferred` transfers the interaction into `spawn_frame_deferred_reserved`; when the future resolves, `RuntimeMailboxInner::finish` invokes the runtime waker. Browser bootstrap installs that waker as the JS callback that calls `frameTurns.request()`. Thus neither `has_live_session` nor parked settle work should be weakened.

## Confirmed hot-spin: incomplete text ingress

The first `text-chunk` creates a text operation with its **whole declared byte count**, pushes only that page, and only the final chunk commits it. The page transport emits further chunks in later UI batches. Until final ingress:

- `TextEditAuthority::begin` retains `reserved_bytes += declared_bytes`.
- The operation is not in the committed queue, so `TextEditAuthority::step` returns `Idle` when it has no queued operation.
- `has_pending_text_work` nevertheless returns true because `reserved_bytes != 0` (and also because its stream slot remains occupied).
- Browser `continue_frame` consequently rearms a MessagePort task that reaches the same idle text step before the next page batch arrives.

This is an actual producer/consumer boundary, not a delayed deadline. Source anchors:

- browser ingress and final-only commit: `🌐️browser-worker/🦀️.rs:509-576`;
- page-side segmented source: `🚚️browser-frame-transport/🟦️.ts:1078-1089`;
- retained reservation and idle-on-uncommitted operation: `🖱️ui/🧬️contract/🪢️text-edit/🦀️.rs:475-496,564-612`;
- faulty continuation predicate: `🧊️renderer/🦀️.rs:15348-15362`;
- direct scheduler decision: `🌐️browser-worker/🦀️.rs:371-389`.

## Minimal repair law

Keep a direct turn only when a text operation can consume already-retained content or retire it. An open ingress stream waiting for its next page must be suspended until the next `TextChunk` batch invokes ordinary `frameTurns.request()`.

The predicate must distinguish **ready buffered text work** from **reserved ingress capacity**. It must retain direct continuation for a committed queue/active edit, cancellation retirement, or other local text disposer work; it must not use `reserved_bytes != 0` or a live stream slot as a readiness proxy.

Do not change the page transport or coalesce a text stream. The next source batch is the wake authority, preserving original order and the 4 KiB chunk cap.

## Fail-first boundary test

Extend the existing actual scheduler suite [`🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts`](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts) and its `🧫️fixtures/🧵️frame-turn-scheduling/🔣️json` fixture with a three-phase text-stream row:

1. Admit a non-final first page of a multi-page text event; let the retained text step consume that page.
2. With no second ingress batch, observe exactly the finishing frame callback and `continueFrame: false`; a still-reserved declared byte count must not enqueue a second direct callback.
3. Admit the next page through the real `text-chunk` ingress. It must request a new Worker frame; final commit then drains the text operation, releases exact reservation, and ends continuation.

The law needs a Rust behavioral companion beside the existing text ingress tests: uncommitted partial content must report *waiting*, a committed buffered operation must report *runnable*, and cancellation must remain runnable until its bounded retirement is terminal.

## Terminal edge

When a tick discovers a new present/text/frame fault it stores `quarantined` before calculating the predicate. If a pending local term is still true, one redundant MessagePort callback can be armed; that next tick exits through the already-quarantined branch with `continue_frame: false`. This is bounded to one callback, not a spin. Gate `continue_frame` with `present_fault.is_none()` only if eliminating that no-op callback is desired; it is not the liveness defect above.

The separate close ladder is deliberately outside `FrameTurnScheduler`: `beginClose` starts its close callback and `closeRuntime` advances runtime/jobs until both terminate before closing the MessagePort. This audit found no source evidence that it incorrectly treats a page Promise as a runnable normal-frame predicate.

## Validation status

No build, server, browser run, or test was executed. Findings are from current source only; concurrent source changes after this audit require re-reading the listed anchors before implementation.

