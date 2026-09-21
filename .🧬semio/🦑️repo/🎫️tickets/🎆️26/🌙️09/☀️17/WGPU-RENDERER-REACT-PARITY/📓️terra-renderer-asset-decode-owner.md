# Renderer Asset Decode Owner

## Verified fault

The live frame transaction owns two incompatible responsibilities. It first
runs `pump_renderer_asset_decode_step`, consumes the sole fuel credit, then
loops until half of the interactive wall deadline
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12845-12859`). The loop is real
work: the probe advances one retained response/probe unit per call. It uses a
deadline loop, `saturating_add`, and `expect` on the active probe
(`…/🧊renderer/🦀️.rs:11223-11389,12735-12740`), which violates the current
bounded-frame policy even though its wall deadline limits the loop.

One-unit-per-rAF is not a repair. The existing comment records that the concrete
forest needs 19,884 probe steps and a catalogued GLB needs 26,174
(`…/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:1412-1458`). The old single-unit
variant was correctly removed because it left visible progress at roughly 55 units
per second (`…/🧊renderer/🦀️.rs:12851-12856`).

## Current continuation protocol

`BrowserRendererWorker::tick` includes `has_pending_asset_decode()` in
`continue_frame`, and then makes `request_frame` true for every continuation
(`🌐️browser-worker/🦀️.rs:371-390`). `runFrameTurn` calls `tick`, posts a public
`frame` reply for every such turn, and returns `continueFrame` to the
`FrameTurnScheduler` (`🎞️frame-worker/🟦️.ts:376-392`).

The page transport processes every public frame reply by requesting rAF when
`requestFrame` is true (`🚚️browser-frame-transport/🟦️.ts:970-974`). Its next
`flush` advances `inputSequence` and posts a batch even when no input rows exist
(`…/browser-frame-transport/🟦️.ts:589-611`). This does not itself mint an input
generation: only replaceable, lossless, or hub publication increments
`generation` (`…/browser-frame-transport/🟦️.ts:501-573`). It is still not inert:
`enqueue_batch` unconditionally invalidates the host scheduler as
`INPUT_STATE` (`🌐️browser-worker/🦀️.rs:325-337`).

Therefore a decode continuation must not use `BrowserTickOutput` or a normal
`frame` reply. That would emit page work for each decode unit and repeatedly
insert empty batches. A later publication needs one normal present wake, but an
intermediate parser/materializer unit does not.

## Smallest coherent owner

Retain the existing single `asset_probe` and its one-at-a-time World/shared
selection in `RuntimeMailbox` (`🧊renderer/🦀️.rs:11155-11228`). Move both
`retire_cancelled_renderer_asset_step` and exactly one probe transition into a
new typed mailbox turn, for example `RendererAssetDecodeTurn`:

| Result | Meaning | Next owner action |
| --- | --- | --- |
| `Idle` | no completed response or live probe | disarm decode work |
| `Pending` | one acquire, parse, semantic, materialize, close, or cancellation unit advanced | arm one Worker MessageChannel decode turn |
| `Published` | a GLB, reference, tile, or UI image changed live render state | arm one Worker MessageChannel decode turn if close remains, and post one ordinary `wake` for presentation |
| `Blocked` | the retained runtime/probe lock could not be acquired | do not spin; wait for the next real frame or asset-state wake |

The enum matters. The present Boolean returns `false` for both an empty lane and
a failed `try_lock`, and returns `true` for both intermediate decoding and state
publication. That ambiguity cannot decide either fair rearming or when the page
must present. Replace `expect` on the `Some` probe and terminal owner with
exhaustive `Option` branches so the typed result remains recoverable.

The owner must retain the exact cancellation and close paths already attached to
the asset authority: cancelled global responses retire through
`retire_cancelled_renderer_asset_step` (`🧊renderer/🦀️.rs:280-287`), World
responses use the surface token in `take_completed_renderer_asset_step`
(`…/🧊renderer/🦀️.rs:11155-11170`), and terminal owner handback is
`finish_renderer_asset_owner` (`…/🧊renderer/🦀️.rs:11373-11380`). It must not
copy bytes or bypass those ownership witnesses.

Add a narrow worker-only ABI such as `assetDecodeStep(): { pending,
presentationChanged }`. It must call that one mailbox turn only; it must not
call `redraw_offscreen_worker`, `FrameTransaction::step`, or `enqueueBatch`.
After a sealed response, `pumpAsset` arms this owner instead of immediately
posting `wake` (`🎞️frame-worker/🟦️.ts:767-774`). A `presentationChanged` result
posts the existing `wake` exactly once, which starts an ordinary frame to render
the newly published state.

## Shared MessageChannel arbitration

A second `FrameTurnScheduler` is not valid. `WorkerTurnTaskQueue` holds exactly
one callback and rejects another scheduled task (`🧵️frame-turn-scheduler/🟦️.ts:7-31`);
two schedulers sharing it would fail with `frame turn task credits exceeded`.

Extend the current scheduler into one fixed two-kind arbiter, backed by that same
one task owner:

1. Keep independent retained pending bits for `frame` and `assetDecode`, plus the
last-dispatched kind.
2. Each MessageChannel callback selects one pending kind, alternating when both
are runnable. A real ingress frame is never queued behind an unbounded asset
slice; a decode lane cannot wait for rAF.
3. A frame turn preserves current `tick`/public-frame behavior. A decode turn
calls only the new ABI and sends no public frame result.
4. Re-arm exactly one callback only if that selected owner returned `Pending`.
On close, stop admitting both kinds and let the existing Rust `closeStep` retire
the probe and response owners.

This is one owned unit per Worker callback, no caller drive, no deadline loop,
and no synthetic input generation. It also leaves the existing bootstrap wake
callback at `frame-worker` line 595 as a normal-frame wake; only asset decode
uses the new kind.

## Fail-first laws and existing test seams

1. Replace the source-string deadline test at
`🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:38-55` with a behavior law:
a real retained GLB can take many decode turns while an independently pending
frame takes its next stage between them. The law rejects `while`, deadline
arithmetic, and `expect` in the frame-transaction decode boundary; it accepts
one typed asset turn only.
2. Extend the actual catalogued-GLB lane at
`…/wgpu-renderer-async-boundary/🦀️.rs:1424-1463`. Drive one exact asset turn,
then one ordinary frame fixture turn, repeatedly; assert the exact mesh schema
arrives, one terminal owner is returned, and no frame transaction is held by
the decoder.
3. Extend the neutral `🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json` and its real
consumer `🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts:43-157` with alternating
`frame`/`assetDecode` rows. Assert a single MessageChannel task is armed, one
callback owns one listed kind, ingress is admitted between callbacks, and
close prevents another decode turn.
4. In that TypeScript test, assert the decode branch contains neither
`runtime.tick` nor `post({ kind: "frame" ... })`; assert only a
`presentationChanged` turn uses `post({ kind: "wake" ... })`. The existing
`runFrameTurn` and browser transport seams at
`🎞️frame-worker/🟦️.ts:376-392` and
`🚚️browser-frame-transport/🟦️.ts:589-611,970-974` are the appropriate
first-party oracle for the no-empty-batch boundary.
5. Add a cancellation row to the same fixture: retire a token after `Pending`
and before its next decode callback. The next turn may close the old owner but
must never publish into the successor. This exercises the existing surface-token
check rather than inventing a second generation authority.

## Scope and confidence

High confidence: the deadline loop lives in the frame transaction, every
continuation presently emits a public frame result, and the task queue cannot
host two independent schedulers. Medium confidence: the exact ABI spelling;
the typed result distinction is required, but the implementation may expose an
equivalent private wire shape. This review did not run build or tests.

## Cross-Platform Owner And Wake Correction

This section supersedes the earlier browser-only `Blocked` proposal. A failed
`RuntimeMailbox::try_lock` is **not** a parkable state: no generic wake is
registered for a `Mutex` release. The current native reference decoder proves
the gap. When its phase-two handback cannot acquire the mailbox, it restores
the output and returns `false`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11580-11670`),
without installing a release notification. Future input or an unrelated wake
can happen to retry it, but source establishes no guarantee. The same method
does explicitly invoke the configured waker for a failed worker submission and
for a raster-pool `Waiting` re-arm; those are different, known transitions.

The correct portable design is one retained `RendererAssetDecodeOwner` whose
acquisition, cancellation, exact probe identity and terminal handoff remain
host-owned. Its decoder performs one `RendererAssetProbe::step` per owned
turn; it never holds the `AppRuntime` mailbox mutex. It reports only
`Pending`, `Published`, `Cancelled`, or `Fault` at the handoff boundary. There
is no general `Blocked` result. A state may wait only when its own producer
has registered an explicit waker.

| Target | One-unit executor | Explicit wake and host handoff |
| --- | --- | --- |
| Browser | The existing single `WorkerTurnTaskQueue`, extended to arbitrate `frame` and `assetDecode`; the decode callback calls the private one-step ABI directly. | A published decode invokes the existing runtime waker, installed by browser bootstrap at `🌐️browser-worker/🦀️.rs:840-844`; the TypeScript callback schedules the existing turn. No decode callback emits an empty public frame/input batch. |
| Native | A `WorkerJobSession`-backed `NativeRendererAssetDecodeSession`, following `RendererIoNode::pump_one` at `🧊️renderer/🦀️.rs:3008-3138`. Submit one probe step to `renderer_worker_pool()` on `Lane::Maintenance`, retain/resume the exact session on `StepOutcome::Yield`, and close one retained owner on cancellation or fault. | The job session registers/takes its real wake; terminal or yielded handoff calls `RuntimeMailbox`'s configured waker. Winit installs it as `HostUserEvent::Wake` at `🪟️winit-app/🦀️.rs:872-882`; that event invalidates `RESOURCE_READY` (`:899-905`), and the next `OsHost::redraw_core` already takes bounded pending applies before frame admission (`:244-249`). |

The existing `NativeReferenceDecodeJob` is useful only as evidence that native
uses `renderer_worker_pool().try_submit(Lane::Maintenance, …)` and calls the
mailbox waker on a completed worker job
(`🧊️renderer/🦀️.rs:10673-10755`). It is not the model to copy because it uses
the unsafe lock-failure return above. `RendererIoNode` is the relevant model:
it owns a `WorkerJobSession`, registers a waker, submits exactly one step,
resumes the same rejected/yielded owner, and closes it stepwise. The new native
session needs a checked `decode_steps` counter, an atomic cancellation flag,
and terminal payload `{token, admitted_surface_token, result}`; the host
validates both identities before publication. Do not assert that the existing
mesh lease is `Send`; compiler validation must decide whether the terminal
payload may use `RuntimeCompletion` directly. If it cannot, retain it in the
native session's host-side handoff slot and consume one exact handoff in the
already bounded host tick.

This preserves native throughput without rAF: a completed worker turn wakes
the Winit event loop immediately, while each submission remains one bounded
unit. It also makes cancellation progress independent of a future frame.
`FrameTransaction::step` should contain neither a decode pump nor a decode
deadline loop after this move; its normal renderer work remains unchanged.

### Required laws

1. Add a native law beside the real catalogued-GLB coverage in
`🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:1412-1463`: a multi-turn GLB
decode yields repeatedly through the session, one independently admitted host
frame is presented between decode turns, and the exact terminal token publishes
once. Cancellation after any yielded turn closes the old owner and publishes
nothing into a successor.
2. Add a Winit handoff law using the existing callback path: completion first
causes the configured `HostUserEvent::Wake`, then the bounded host-tick apply
occurs before the next frame admission. A lock-contention branch must be
unrepresentable in the worker step; the test rejects a retry that depends on
an arbitrary redraw.
3. Extend the browser arbiter fixture from the preceding section with the same
token/cancellation rows. It must prove that an asset turn performs no public
`tick` or `frame` post, while `Published` performs precisely the one explicit
runtime wake.

Confidence is high that an arbitrary mutex-release `Blocked` state is unsafe,
and high that Winit's configured wake gives native a concrete unthrottled
handoff path. The exact session/payload type is an implementation decision
that needs compiler validation; this audit did not build or run tests.
