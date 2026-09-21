# Checkpoint 18 Frame Liveness

## Finding

Checkpoint 18 did not expose a settings command or footer geometry failure. The settings pointer down and up were admitted at the exact mounted `framework.settings` toggle. The resulting `noteShellCommand` dispatched. Publication then waited behind frame generation 10 while its retained transaction remained in `Uploads` without another Worker turn for about 43.64 seconds.

The dedicated frame Worker schedules every retained continuation with `setTimeout(callback, 0)`. This repository's mounted UI intake already records why that transport is unsuitable for repeated Worker turns: nested timers clamp after five calls and hidden panes clamp Worker timers to at least one second, while `MessagePort` tasks remain ordinary unthrottled tasks. Checkpoint 18 is the physical receipt for the same defect at the frame owner.

## Runtime chain

The WGPU console at `🗑️generated/astra-runtime/checkpoint-18-main/wgpu/console.txt` records:

- `194313`: frame generation 10 admitted.
- `205259` and `206539`: the left GLB is present in both World3d surfaces, with one draw and one instance. This is progressive asset publication rather than an asset crash.
- `215875`: host input generation 25 drains a move and two discrete events.
- `215876`: pointer down at `(1299.6617, 985.6)` hits `Some((Toggle, Some("framework.settings"), None))` against all 43 mounted/staged targets.
- `216138`: `noteShellCommand` dispatches; pointer up follows and hits the same toggle at `216139`.
- No frame-step receipt follows until `259782`, a gap of about 43.64 seconds.
- `259787`: generation 10 still reports `blocked=true pending=true phase=Some(Uploads)`, with no retained fault and no stall steps.
- `260038`: generation 10 becomes unblocked and terminal.
- `260087`: generation 11 is admitted.
- `263245`: `puzzle3d.panel.settings` finally ingresses with nine nodes.
- `269905`: the engine sync includes `puzzle3d.panel.settings`.

The final step receipt later contains the panel tab and four numeric controls in the retained control registry. The screenshot still paints the World3d surface over that area. That later paint-order/stale-presentation mismatch is distinct from the 43.64-second publication delay and must be reassessed after the Worker wake is fixed.

## Source boundary

`BrowserRendererWorker::tick` already includes `host.frame_build.has_live_session()` in `request_frame`. `runFrameTurn` returns that exact `requestFrame` flag to `FrameTurnScheduler`, which correctly re-arms while the step returns true. The broken boundary is the concrete scheduler supplied by `frame-worker/🟦️.ts`: `(callback) => setTimeout(callback, 0)`.

The bounded repair is one capacity-one `MessageChannel` task owner for the frame scheduler. It retains at most one callback, publishes one message per requested turn, runs one callback per message, and closes both ports after the frame scheduler reaches terminal-empty. The existing `FrameTurnScheduler` remains responsible for deduplicating requests and executing one frame unit per task.

The tick result also needs to separate two meanings the old `requestFrame` Boolean combined:

- `requestFrame` remains the page-facing redraw request and includes a future scheduler deadline.
- `continueFrame` is the private Worker continuation and includes only runnable work: a cursor wake, pending text/world/decode/settle/apply work, a live frame owner, or pending presentation.

The Worker scheduler re-arms only for `continueFrame`. A future deadline can therefore request the next page animation frame without creating an unpaced MessageChannel loop. Hub status is also excluded: status may remain pending only because the Runtime interaction is externally checked out, and the retained Runtime waker requests a new Worker turn when that owner returns. An external asset fetch is not a live decode probe; its existing response/pump callback requests work when bytes arrive. No page ingress, timer duration, loop, frame credit, or runtime dependency changes.

## Test-first receipt

The neutral fixture now declares a retained runtime transaction with `Uploads → Finish → Terminal`. Its page requests are `true, true, true`, while its private Worker continuations are `true, true, false`; the final future/page request proves the private owner becomes idle instead of spinning. Separate future-deadline and external-Hub cases each execute one owned task and return `continueFrame=false`, proving neither page timing nor externally blocked status creates a self-wake loop. The third-party React Scheduler oracle consumes the same exact sequence independently:

```text
frame-scheduler-oracle: input=41 generation=7 callbacks=2 runtime=uploads→finish→terminal close=1 exhaustion=fault
```

The first focused Worker law was run before the task owner existed:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-wgpu:test-browser-worker' --skip-nx-cache -- wgpu-worker-step-budget -t 'keeps an actual Worker task armed while the retained runtime frame remains pending'
```

It selected one law, skipped 111 outside the filter, and first failed at `new WorkerTurnTaskQueue()` because the constructor did not yet exist. Vitest completed in 2.56 seconds and Nx in 11.2 seconds.

The law was then strengthened before implementation to inspect the concrete frame-Worker continuation seam before constructing the task owner. It failed because `runFrameTurn` returned `result.requestFrame` instead of the private `result.continueFrame`. The second RED selected the same one law with 111 outside, took 2.93 seconds in Vitest and 10.1 seconds in Nx. This demonstrates the current scheduling behavior rather than only the new type's absence.

## Implemented receipts

`WorkerTurnTaskQueue` now owns one callback and one `MessageChannel`. A second callback is refused, each port message takes the exact retained callback before invoking it, and terminal close clears the slot and closes both ports. `BrowserTickOutput` now publishes the private `continueFrame` signal described above. The frame Worker uses it for its scheduler return while preserving `requestFrame` in the page message.

The initial focused law passed 1/1 with 111 laws outside the filter. Its actual `MessageChannel` tasks consumed all three declared runtime phases, its page-only terminal request did not re-arm, and its capacity and closed-state refusals passed. Vitest took 1.84 seconds and Nx 7.5 seconds. After adding the direct future-deadline and external-Hub fairness cases, the focused law passed again with 135 laws outside across the complete ten-file inventory; Vitest took 2.66 seconds and Nx 9.7 seconds.

The earlier full result of eight files and 112 laws exposed a target-inventory inconsistency: `test-browser-worker` omitted the wheel suite, while it named the socket-door suite that the Vitest config omitted. The command and config now both include those canonical suites. The corrected full target passed all ten files and 136/136 laws. Vitest took 3.10 seconds and Nx 9.1 seconds:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-wgpu:test-browser-worker' --skip-nx-cache
```

The third-party React Scheduler oracle passed again after the fairness cases were added. The generated frame Worker was refreshed through its Nx target in 29.1 seconds. `check-frame-worker` reports `🎞️frame-worker.js is fresh` and completed in 7.4 seconds. `rustfmt --emit stdout` parsed the changed Rust source and the packet's diff check is clean. Native compilation and the fresh WGPU physical checkpoint remain root-owned.

## Segmented text ingress fairness

The retained frame owner remains directly runnable while `FrameBuildHandle::has_live_session()` is true. Its Worker turn is synchronous: each call to `try_step_on_worker` returns an outcome or terminal state, while file and extension work is detached through `RuntimeApply` and its completion requests a new Worker turn. Removing that continuation would strand real frame work.

The actual unbounded self-wake was narrower. `AppInteractionState::has_pending_text_work` treated both a declared-byte reservation and any live segmented-text stream as runnable. `start_text_operation` reserves the full declared length before the browser transport supplies later chunks, so a stream awaiting its first or next external page kept returning `continueFrame=true` even though no local text unit could advance.

The neutral fixture now records four ingress phases. Start and partial ingress are externally blocked and return false; commit and cancellation expose runnable local work and return true. The third-party React Scheduler oracle passed this exact table through the scoped Nx command and still reports the retained frame sequence:

```text
frame-scheduler-oracle: input=41 generation=7 callbacks=2 runtime=uploads→finish→terminal close=1 exhaustion=fault
```

The actual native law is `async_boundary_tests::an_incomplete_text_stream_waits_for_external_ingress_without_a_worker_self_wake`. It uses the real `AppInteractionState` and retained `TextEditAuthority`, then checks empty, declared, partial, committed, drained, and cancelled phases. Native 94 provided the required RED: two selected laws ran, one passed and this law failed at the first externally blocked reservation assertion, with 1,291 outside the filter. Nextest took 34 ms, Nx took 44.7 seconds, and the run id was `a1b6e304-05f9-4063-862e-0128bacf604d`.

The browser source-boundary law independently failed before production because `has_pending_text_work` still read `reserved_bytes()` and `text_streams.iter()`. It selected one law with 135 outside, took 2.23 seconds in Vitest and 10.2 seconds in Nx.

`TextBuffer::runnable_work_pending` now reports only a queued or active unit, active cancellation, bounded retirement, a retained disposer, or retired roots. `has_pending_text_work` combines that predicate with the explicit text-cancel flag. Open stream metadata and declared reservations remain ownership and backpressure authorities but no longer advertise local progress while bytes are outside the Worker.

After the repair, the focused browser law passed with 135 outside in 1.43 seconds of Vitest and 6.9 seconds of Nx. The final full browser-worker census passed all ten files and 136 laws; Vitest took 1.93 seconds and Nx took 7.0 seconds. The third-party oracle passed again. Both changed Rust sources parse under `rustfmt --emit stdout`; the scoped diff check is clean. The root-owned native GREEN and fresh WGPU physical checkpoint remain pending.

## Scope

This packet does not change settings commands, footer hit geometry, probe waits, frame credits, or paint ordering. It does not claim the settings panel paint mismatch is repaired until a fresh physical checkpoint proves the panel is visible above the canvas.
