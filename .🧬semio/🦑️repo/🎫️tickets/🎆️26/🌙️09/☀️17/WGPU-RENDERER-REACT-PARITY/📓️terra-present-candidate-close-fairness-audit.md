# Presented Candidate And Component-Close Fairness Audit

Read-only source audit on 2026-09-21. No build or test was run.

## Confirmed candidate ownership result

The current normal cancellation, fault, supersession, stale-completion, and host-close paths retain then return a staged input witness before its enclosing owner can be retired.

- `frame-job/🦀️.rs:230-240` returns the witness from a Build or Prepare phase before `retire_active_phase`.
- `frame-job/🦀️.rs:375-380` does the same on input supersession.
- `frame-job/🦀️.rs:402-406` turns preparation cancellation/fault into that same cancellation path.
- `frame-job/🦀️.rs:445-459` returns a completed-but-unpresented frame's witness before closing it.
- `renderer/🦀️.rs:14197-14208` covers both live carrier locations: `FrameBuildCursor.input_candidate` and `AppFrameAfterChrome.input_candidate`.
- `renderer/🦀️.rs:14909-14930` and `15109-15117` discard a presenter-held candidate during close and abort.
- `ui/engine/🦀️.rs:1209-1218` clears each matching sealed window, while a nonmatching witness is rejected.

I found no additional reachable normal drop path. In particular, an `AppFrameAfterChrome` close can nest a preparation owner, but normal `ActiveFrameBuild` cancellation calls `discard_presented_input_candidate` *before* it invokes that close ladder, so its candidate is already absent.

The existing fail-first coverage has one carrier gap: it should construct a completed chrome boundary with `FrameTransaction.after_chrome = Some(... input_candidate ...)`, then exercise cancellation and assert a later `UiEngine::seal_presented_input_candidate` succeeds. The current frame-job tests cover build, preparation, and completed presentation, but this carrier is independently owned at `renderer/🦀️.rs:13147-13158` and is explicitly cleared at `14197-14208`. This is a regression-test extension, not evidence of a current source defect.

## Confirmed component-close admission race

`OsHost::advance_component_surface_close` at `wgpu/os-host/🦀️.rs:694-713` only waits for `presenter.has_pending_presentation()` before taking a bridge request. It does **not** wait for `FrameBuildHandle::has_live_session()`.

A live frame worker can still own a pre-close `AppFrameBuild`, including its engine packets and staged candidate. Its liveness authority is exactly `frame-job/🦀️.rs:697-701`. Taking the external component close while that worker is live allows the close owner to retire the exact CPU/GPU token at `os-host/🦀️.rs:356-431`, while the worker is still capable of returning the old packet to the presenter. The presenter guard alone does not cover it: a worker that has not handed out a presentation is not in `AppPresenter`.

### Required admission invariant

Do not take a pending component-close request until both are empty:

```rust
!presenter.has_pending_presentation() && !frame_build.has_live_session()
```

This is an admission fence only. It must not wait for unrelated work after the external close owner has been admitted.

The bridge request is issued only after `Scenes::SceneSurfaceRetirement` has already removed the component from the UI retirement path (`Scenes/🦀️.rs:493-505`). Thus, after the dual fence and first admitted close step, subsequent B-window frames cannot create new A scene work through normal UI publication. The old A frame is the race that must be fenced.

## Fairness split

`winit-app/🦀️.rs:239-276` currently returns from the entire coordinator whenever `advance_component_surface_close()` reports Busy (lines 248-250). A close whose Asset, World, CPU, GPU, or raster phase needs many bounded turns therefore suppresses all event draining, frame admission, and presentation for unrelated B windows.

Keep the dual fence while the bridge request is **pending admission**. Once the bridge is **active**, perform exactly one `ComponentSurfaceCloseOwner::close_step` per redraw and continue the ordinary coordinator for B:

1. Pump only the existing bounded native asset/decode steps and one exact close step.
2. Drain B events, poll/resubmit its `FrameBuildHandle`, and drive its presenter normally.
3. Invalidate `RESOURCE_READY` whenever the active close is still nonterminal.
4. Do not use `holds_presented_input_publication()` as the fence; it is narrower than `has_pending_presentation()` and excludes retirement and gate acknowledgement.
5. Represent pending-admission and active-close separately. The current boolean result of `advance_component_surface_close` and `component_surface_close_pending()` both conflate these states (`os-host/🦀️.rs:318-320, 694-713`).

The external owner is already exact-host scoped: Asset and World receive `request.owner` (`os-host/🦀️.rs:361-380`), raster uses `request.owner.host_id` (`410-413, 431+`), and engine close uses the captured `EngineSurfaceToken`. This permits B to progress after admission; it does not permit a whole-host parallel GPU close.

## Smallest native regression

Extend the existing two-component OS-host standalone fixture. Arrange:

1. A has a live frame build that has constructed its engine packet but has not been handed to the presenter.
2. Request A component close.
3. Assert the bridge remains pending and A's exact CPU/GPU token is still present until that build has closed or entered the presenter.
4. Complete/retire the A frame; admit the close.
5. Stall A in a later exact close phase and enqueue a B pointer/action plus B frame.
6. Drive bounded redraw turns. Assert B's action/presentation completes before A's close terminal acknowledgement; assert no A packet becomes presentable after the close token is admitted.

This is behavioral coverage for both safety and fairness. It should use the real `OsHost::build_and_publish_snapshot`/presenter and `FrameBuildHandle`, not a source-text assertion.

## Confidence

High for the missing live-frame admission fence and for current whole-coordinator starvation: both follow direct ownership and call ordering. High that standard candidate exits are now covered. The after-chrome case is a test gap only; no reachable drop was established.

