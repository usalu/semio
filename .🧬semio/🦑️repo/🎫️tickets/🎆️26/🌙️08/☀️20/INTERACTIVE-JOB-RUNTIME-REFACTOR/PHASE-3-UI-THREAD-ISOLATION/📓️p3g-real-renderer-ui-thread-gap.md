# P3g — Real Renderer UI-Thread Gap

Date: 2026-08-22  
State: gate open

## Source truth

The mounted native renderer still executes its full immediate-mode frame from the UI callback:

```text
WindowDelegate::redraw
  -> OsHost::build_and_publish_snapshot
     -> AppRuntime::frame
        -> ShellState::render_chrome
        -> atlas/icon/raster GPU uploads
        -> GpuContext::render_frame
        -> apply_window_cursor
  -> OsHost::present_snapshot
```

The relevant implementation is `renderer/.../wgpu/🦀️winit_app.rs`; `AppRuntime::frame` is in the target's `📦️glue.rs`. Their existing documentation also states that chrome layout, tessellation, and GPU submission remain on that thread. Therefore Phase 3's “zero product/plugin/domain code on the UI thread” exit gate is source-false even though event callbacks are enqueue-only and the narrow camera-deadline scan already uses `FrameBuildJob`.

## Available completed seam

The Phase 5 UI-WGPU code already supplies `PreparedRenderInput`, resumable `PreparedRenderJob`, a capacity-one `PreparedRenderReceiver`, immutable `PreparedRenderPacket`, `PreparedRenderGate`, and `GpuContext::submit_prepared`. `PreparedRenderJob`, receiver, packet, and gate are compiler-asserted `Send`; `UiPresentToken` is deliberately `!Send`. Native `AppRuntime` itself is also compiler-asserted `Send` after the self-weak field was removed.

That seam is not wired into the real OS renderer. Running only `PreparedRenderJob` after `render_chrome` would still leave the expensive product traversal on the UI thread and does not satisfy the gate.

## Required repair

- Extract UI-only window/GPU presentation authority from the worker-owned frame state.
- Move input reduction, shell/domain traversal, layout/tessellation, DrawList construction, and upload preparation into bounded worker jobs with persistent cursors.
- Publish immutable generation/revision-stamped prepared packets.
- Keep only non-blocking latest-packet acquisition, stale validation, GPU submit/present, cursor/IME, and mandatory platform window calls on the UI capability.
- Preserve the last valid packet on worker stall/device loss.
- Verify the real mounted redraw path, native and browser targets, zero product/plugin/domain UI reachability, and stress callback p99 ≤2 ms.

No Phase 3 completion claim is permitted until that repair and measurement are green.

## Exact 2026-08-22 dependency boundary

The smallest honest source slice is larger than replacing `GpuContext::render_frame` with
`submit_prepared`. The current call graph is:

```text
winit_app::WindowDelegate::redraw
  -> OsHost::build_and_publish_snapshot
     -> AppRuntime::frame
        -> ShellState::render_chrome
           -> ShellState::render_chrome_build
              -> render_main_window / render_left_panel / render_right_panel
                 -> Interpreter::render_ui_node
                    -> Ui::frame
                       -> FrameworkSceneHost::paint_slot
                          -> Scenes::render_component_scene
                             -> EngineCanvas device/queue/texture work
        -> GpuContext::render_frame
        -> apply_window_cursor
```

`AppRuntime: Send` is now compiler-proven on native, but moving the whole value to a worker would
also move `GpuContext` and `Window`, violating the platform-affinity boundary. Merely sealing the
already-constructed `DrawList` with `PreparedRenderJob` would leave every product/plugin/domain and
layout/tessellation call above it on the UI callback and therefore does not count as progress against
the exit gate.

The concrete extraction must first replace the 35 renderer signatures carrying `GpuContext` through
Shell/Interpreter/Scenes/EngineCanvas with an owned render-resource request interface. Most resources
map directly to existing `PreparedRenderUpload::{GlyphAtlas,IconAtlas,Raster,Mesh}`. EngineCanvas is
the exceptional boundary: it currently calls `GpuContext::{device,queue,register_engine_texture}` to
render Vello scenes into GPU textures during traversal. Its worker result therefore needs an owned,
Send engine-scene directive (or a deterministic CPU raster upload); the UI presenter may consume that
directive using device/queue, but may not repeat product traversal. Only after that interface exists
can `AppRuntime` shed `gpu`/`window`, become worker-owned, and publish a `PreparedRenderPacket` plus
platform directives through the capacity-one receiver.

No runtime p99 is recorded here because the real source path is still false. A synthetic sink or a
post-build packet-sealing benchmark would measure the wrong boundary.

## 2026-08-22 implementation update

The source path above has now been replaced, but the gate remains open until the mounted runtime
measurement and both target compilers complete. Redraw now non-blockingly polls and submits a
generation-stamped frame build to the process WorkerPool. That worker owns AppRuntime::frame,
ShellState::render_chrome, EngineCanvas and Infinite packet construction, and PreparedRenderJob.
The UI-owned AppPresenter only realizes the already-owned EngineCanvas GPU directives, submits the
immutable PreparedRenderPacket under UiPresentToken, and applies fullscreen/cursor directives.

AppRuntime is now held in Arc<Mutex<_>>, remains compiler-asserted Send, and contains no window, GPU
context, or UI capability. The UI callback uses try_lock/try_recv only and preserves the last
published cursor/packet while a worker build is in flight. A completion-specific FrameReady event
wakes and invalidates the scheduler without advancing the source generation; ordinary
input/metric/kernel wakes advance it. Completed packets whose generation is not the latest requested
generation are discarded before presentation.

Native Shell and Scenes state that participates in frame construction was migrated from
pool-thread-affine thread-local cells to process-owned mutex cells. Browser-only boot state remains
thread-local because the current wasm host is single-threaded.

### 2026-08-22 live task-pool reachability correction

The frame-build half is worker-owned on native, but the broader Phase 3 gate is still source-red.
Native `spawn_app_task` stores futures in `kernel_runtime::TASK_POOL`, and
`WinitApp::about_to_wait` calls `kernel_runtime::poll_tasks` on the winit thread. Drained input
reduction and multiple product continuations therefore remain UI-thread-reachable even though
`AppRuntime::frame` itself moved. The static deny audit exempts this runtime root and does not prove
semantic reachability. Native `spawn_app_task` must use the process worker pool with `Send` state,
or each remaining UI-polled future must be split so only capability-bound presentation stays local.
Browser `FrameBuildHandle::poll_runtime_and_resubmit` also still drives the complete frame inline
and needs a real Web Worker/cooperative bounded-job path before the cross-target gate can close.

The focused native compiler reached the Puzzle dependency and stopped on 26 concurrent stale
UtilityDefinition awaits from the manifest-constructor de-async migration before compiling the
renderer crate. The owning agent was sent the exact residual category. No renderer diagnostic is
known yet. Required remaining evidence is focused native compile/tests after the shared dependency
is stable, both wasm target compilers, mounted real-renderer callback p99 at or below 2 ms, and a
trace/capability census proving no product/plugin/domain execution under the UI capability.

## 2026-08-22 native continuation closure

The native task-pool gap above is closed. `TASK_POOL`, `REAL_WAKER`, `install_waker`, and
`poll_tasks` are deleted. Native `spawn_app_task` now wraps each `Send` future in a wake-driven
`KernelPoolFuture` and schedules each ready turn on the process `WorkerPool` interactive lane. It
does not create an executor thread, call `block_on`, or depend on `about_to_wait`; winit callbacks
only enqueue bounded input/state messages, invalidate the scheduler, non-blockingly poll a
capacity-one frame result, and present an already-prepared packet.

`RuntimeMailbox` is the single native AppRuntime ownership boundary. Its fixed capacity is 128
including both ready and in-flight work. Ordinary completions may consume 127 entries; the final
entry is reserved for returning the uniquely-owned `AppInteractionState`, so lossless commands can
never strand that state outside the runtime. Input is drained only after a lossless slot is known to
exist. Keyless command/checkpoint work is never evicted; replaceable work coalesces only with an
already-ready completion carrying the same static key. Each keyed completion carries a monotonic
revision, and stale completions are rejected before application.

Every formerly guard-crossing continuation now follows the same two-phase protocol:

```text
short synchronous apply
  -> take owned AppInteractionState
  -> await boot/input/actions/tutorial/assets/keyboard on WorkerPool
  -> enqueue reserved serial completion
  -> restore AppInteractionState in one short synchronous apply
```

The owned state contains Shell/Input/theme/pointer/deadline/asset state. `AppRuntime` and its mutex
guard never enter an async future. Resize, drained input, boot/hot reload, `pump_sync_events`,
`dispatch_action`, tutorial document flushing, asset polling, and keyboard/pointer fan-out all reach
the same ownership-transfer boundary. Frame construction applies at most one completion, locks only
for synchronous frame construction, and generation-checks the returned prepared frame before
presentation.

The kernel seam is independently lossless and bounded: ready plus in-flight outcomes share 64
slots; overflow intents are returned to the caller for retry, never coalesced or evicted.
`HostWaker`, opaque outcome detail, the seam cells, and the native exchange future are owned `Send`
forms (`Arc`/`Mutex`, `Box<dyn Any + Send>`, and `Future + Send`). Its focused test proves a genuinely
pending exchange resumes on the process pool and wakes the host without UI polling.

The exact before/after continuation census is: before, 16 `spawn_app_task` call sites included 14
AppRuntime continuations that retained a mutex guard across an await plus the non-`Send` Rc kernel
seam. After, the six owned renderer files contain four textual `spawn_app_task` calls, of which three
compile on native (boot, kernel exchange, and the generic reserved-completion driver; the fourth is
the cfg-wasm body). The only two `MutexGuard` matches are synchronous `try_lock` return signatures.
There are zero `.lock().*.await`, `Arc<Mutex<AppRuntime>>`, task-pool, or UI-polling matches. More
importantly than the textual scan, every native submitted future is compiler-checked by a
`Future + Send + 'static` bound.

### Final verification

- `cargo check -p semio-framework-os-renderer-wgpu --message-format=short`: passed.
- `cargo check -p semio-framework-os-renderer-wgpu --release --message-format=short`: passed.
- `cargo test -p semio-framework-os-renderer-wgpu --lib async_boundary_tests -- --nocapture`:
  4 passed, covering absence of the native UI executor, mailbox capacity/coalescing/reserved return
  capacity, the sole native entrypoint driver, and retired direct dependencies.
- Current renderer test binary, `kernel_seam::tests`: 3 passed, including continuation wake and
  capacity-64 lossless backpressure.
- Current renderer test binary, `frame_job::tests`: 6 passed, including stale-generation rejection.
- Mounted pointer-storm callback test: 1 passed for 20,000 samples with asserted p99 below 2 ms.
- Stalled-shell mailbox-poll test: 1 passed with asserted p99 below 2 ms.
- `bun ./📜️script.ts verify interactivity`: DENY mode clean.
- Owned source scan: no `TASK_POOL`, `poll_tasks`, `install_waker`, `thread::spawn`, or
  `Arc<Mutex<AppRuntime>>`; `git diff --check` passed.

### Wasm audit and remaining cross-target evidence

Wasm deliberately preserves its single-thread cooperative behavior: `spawn_app_task` uses
`spawn_local`, the same capacity-128/revision-checked runtime mailbox bounds continuations, and the
capacity-one frame path drives the small `InteractiveJob` inline. It has no native mutex guard in a
future and no unbounded task queue, but it still has no Web Worker and therefore does not claim
native-equivalent thread isolation.

The final renderer wasm check was attempted, but the compiler stopped in out-of-scope dependencies
before reaching this crate: editor `component.rs:1381` and surface `paint/component.rs:1051`,
`node-graph/component.rs:590`, and `tiled-map/component.rs:3316` call the now-async
`RenderContext::new` as if it were a result, producing E0599 plus derived E0277 diagnostics. Thus the
native continuation/UI-isolation blocker is closed and measured; a green renderer wasm compiler
claim remains blocked on those four upstream await repairs.
