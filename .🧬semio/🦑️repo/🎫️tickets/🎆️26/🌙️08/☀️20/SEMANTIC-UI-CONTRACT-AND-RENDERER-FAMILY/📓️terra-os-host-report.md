# 📓️ terra-os-host-report — OsHost decomposition (Wave W4)

## Done

New files, all in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/`:

- **`🦀️deadlines.rs`** — every ad-hoc `app_now_ms()`-stamped deadline field `AppRuntime` used to carry,
  replaced by pure, `ui_render::FrameScheduler`-driven sources: `sweep_expired`/`arm`/`cancel` (the
  token-replace camera-settle/wheel-zoom-settle shape, moved verbatim from `📦️glue.rs`'s
  `sweep_expired_camera_dispatch_deadlines` plus its 4 original tests, generalized), `CaretBlink`
  (repeating deadline, armed only while a caret is present), `HotSwapPoll` (coarse ~1 s gate, decoupled
  from any scheduler so `AppRuntime`'s own clock can use it without needing `OsHost`'s), and
  `on_asset_ready`. 20 unit tests, all pure (no window/GPU).
- **`🦀️kernel_seam.rs`** — `KernelSeam { submit_intents, drain_outcomes, set_waker }` plus the single
  concrete impl `AppKernelSeam` (U3: no `dyn`, and — better than the ticket's own two-impl-per-platform
  anticipation — no cfg pair either, since it's built on `crate::spawn_app_task`, which already
  branches native/wasm internally). `HostWaker` is the winit-thread-side "please wake" callback;
  `KernelOutcome.detail: Box<dyn Any>` is a deliberately opaque payload pending the protocol-flip
  packet. `default_intent_exchange` is an explicit, documented stub (see "what remains blocked").
  2 tests, including the ticket-specified "fake seam receives intents, outcomes reach the host on
  wake" case.
- **`🦀️os_host.rs`** — `OsHost { runtime, scheduler, kernel, clock, caret, hot_swap, wheel_zoom_settle,
  camera_settle }`, the composition root. `OsClock` is a from-scratch clock (`Instant`-origin native,
  `performance.now()`-origin wasm), not a reuse of `ui_host`'s own — see "what remains blocked" #1 for
  why. **Deviation from the master plan's literal `OsHost { engine, backend, … }` sketch**: no
  `FrameEngine`/`ActiveBackend` fields — see the file's own module docstring; this crate's rendering is
  still the old `DrawList` pipeline, and instantiating the new Element-tree types with nothing feeding
  them would be dead scaffolding. `scheduler` lives on `OsHost`, not `AppRuntime` — a load-bearing
  soundness reason, not a style choice: `WindowDelegate::scheduler_mut` returns a real `&mut
  FrameScheduler`, which a `Rc<RefCell<AppRuntime>>`-shared field could never satisfy without `unsafe`.
- **`🦀️winit_app.rs`** — `impl WindowDelegate for OsHost` (the `ui_host` seam, fully implemented) plus
  `WinitApp`, a hand-rolled `ApplicationHandler` **replacing `SemioApp`** — see "the redraw audit"
  below and the file's own module docstring for why it is hand-rolled rather than
  `ui_host::window::NativeHost<OsHost>` (a real, evidence-based blocker: `NativeHost` creates its window
  itself, after the delegate must already exist; this crate's `GpuContext::from_window` needs the
  window to exist *before* `AppRuntime`/`OsHost` can be built). Input normalization ports
  `NativeHost::normalize`'s exact match arms (that method is private) over `ui_host::event`'s public
  fns. `PointerButton`↔`i16`, `EventModifiers`↔`ui_wgpu::wgpu::PointerModifiers`, and a DOM-key-string→
  `KeyAction` table are the three small adapters bridging the new normalization to the unchanged
  `AppRuntime::handle_pointer_move/handle_pointer_button/handle_key`.

Surgical edits to `📦️glue.rs`:
- Mounted the four modules in a new `//#region 🔖️OsHostDecomposition` block, placed before the
  peer-owned `parallel_runtime` mount.
- Deleted `start_frame_loop`, `enum HostUserEvent`, `struct SemioApp` and its `ApplicationHandler`
  impl — replaced by a region comment pointing at `winit_app::{HostUserEvent, WinitApp}` with the exact
  old line numbers (see redraw audit).
- `run_native`/`semio_wgpu_mount` now build `winit_app::WinitApp` instead of `SemioApp`.
- Removed the 5 imports that became dead after `SemioApp`'s deletion (`ApplicationHandler`,
  `WindowEvent`, `ActiveEventLoop`, `EventLoopProxy`, `WindowAttributes`, `WindowId`,
  `dispatch_window_event`, `WindowInputState`, `schedule_frame` — `winit_app.rs` imports each itself).
- **The waker-correctness fix** (see below): `kernel_runtime::poll_tasks()` no longer hard-codes
  `Waker::noop()`; a new `kernel_runtime::install_waker`/`REAL_WAKER` thread-local lets `winit_app.rs`
  install a real, cross-thread `Waker` once at boot.
- Updated the now-stale `semio_wgpu_mount` gap doc-comment (the `start_frame_loop`-based disposal gap
  it described no longer exists in that shape).

## Acceptance: UNRUN

Per U4, no cargo command was run. This packet's four new files and the `📦️glue.rs` edits above were
**not compiled or checked** — the commands `sol` should run:

```
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-os-renderer-wgpu --lib --timeout 600000
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-os-renderer-wgpu --all-targets --timeout 600000
CARGO_TARGET_DIR=<scratchpad>/target cargo test -p semio-framework-os-renderer-wgpu deadlines:: --lib --timeout 600000
CARGO_TARGET_DIR=<scratchpad>/target cargo test -p semio-framework-os-renderer-wgpu kernel_seam:: --lib --timeout 600000
```

## The redraw audit — every `request_redraw`/`Poll`/per-frame-poll site removed

| old `📦️glue.rs` site (pre-edit line) | what it did | replaced by |
|---|---|---|
| `fn start_frame_loop` (~2271) | recursive `schedule_frame` (rAF/timer) chain calling `app.frame()` then immediately rescheduling itself, unconditionally, forever | deleted outright — no replacement construct exists; redraw is now driven entirely by `winit`'s own `RedrawRequested`, itself gated by `FrameScheduler::should_render` |
| `event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll)` in `user_event` (~2383) | set once at boot, made the loop spin continuously forever after | `WinitApp::recompute_control_flow` — `WaitUntil(next deadline)` when the scheduler has one pending, `Wait` otherwise; called after every callback |
| `window.request_redraw()` inside `WindowEvent::RedrawRequested` (~2406) | unconditional — every dispatched `RedrawRequested` immediately queued the next one | `WinitApp::window_event`'s `RedrawRequested` arm only calls `host.redraw(reason)` if `self.pending_reason` was actually armed by `about_to_wait`; it does not itself request another redraw |
| `kernel_runtime::poll_tasks()` in `about_to_wait` (~2422) | ran every iteration of an infinite `Poll` loop | still runs every real wake (correctness-preserving), but wakes are now sparse — bounded by `should_request_redraw`/deadlines, not per-frame |
| `window.request_redraw()` in `about_to_wait` (~2424) | unconditional, every iteration | `if let Some(reason) = should_request_redraw(&mut host.scheduler, now) { … window.request_redraw() }` — only when the scheduler actually has something dirty or a deadline due |

**The zero-idle-frames property is enforced by `ui_render::FrameScheduler::should_render` (already
shipped by the `render-scene` packet, reused verbatim, not reimplemented) and exercised by that
crate's own `should_render_returns_none_for_a_clean_window` test — this packet's job was removing every
call site that used to bypass it, not reproving the property itself.**

## The vello finding, with evidence

**Not the minimap.** `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️minimap.rs`
contains zero `vello` references; the one `vello::Scene` mention anywhere near it is a doc-comment
aside in a *different* file (`ui/📦️packages/…/📦️glue.rs:64`, `paint_minimap_widget`'s doc), not real
code.

**The real, unconditional (native included) vello/wgpu consumer is
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs`**
— mounted directly as `pub mod engine_canvas;` in this crate's own `📦️glue.rs`. Evidence:
- `use vello::peniko::Color; use vello::wgpu; use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions};` at lines 21–23, **not** behind any `#[cfg(target_arch = "wasm32")]` (only the
  file's `use js_sys;` at line 25 is wasm-gated).
- `struct EngineSurface { …, vello: Renderer, texture: wgpu::Texture, view: wgpu::TextureView, … }`
  (~line 78) and `fn ensure_surface`/`fn create_target_texture`/`fn render_vello_scene` (~lines
  307–395) are all unconditionally compiled — they construct a real `vello::Renderer` against
  `gpu.device(): &wgpu::Device` and render `GraphHost`/`FlowHost`/`EditorHost`/`MapHost`/`BoardHost`
  scenes to an offscreen `wgpu::Texture`, on every target this crate builds for, native included.
- Contrast with `infinite_canvas` (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️component.rs`,
  this crate's own `infinite_world` dependency): its `gpu_session` module, which ALSO does real
  `vello::Renderer`-against-`wgpu::Device` work, is correctly `#[cfg(target_arch = "wasm32")]`-gated
  (`pub mod gpu_session` at line 1122). `EngineCanvas` does not follow that precedent.

**Recommendation:** confine `EngineCanvas`'s vello/wgpu path to `#[cfg(target_arch = "wasm32")]`,
mirroring `infinite_canvas::gpu_session`, and give native targets a CPU-raster fallback — `tiny-skia`/
`resvg`/`usvg` are **already dependencies of this exact crate** (`Cargo.toml`, confirmed), so the
fallback path does not need a new dependency, only new code (out of this packet's OWNS — it touches
`🧱️elements/EngineCanvas/`, forbidden here). Filed as a finding, not fixed.

## What remains blocked, and on what

1. **`ui_host::WindowDelegate` has no clock-access hook.** `should_render`'s `now` and
   `FrameScheduler::request_deadline`'s `due` must be the same clock; `ui_host::window::native::
   NativeHost`/`CanvasHost` own that clock privately (`MonotonicClock`/`BrowserClock`) with no
   accessor. `os_host::OsClock` is a *separately constructed* clock with the same primitive
   (`Instant::now()`/`performance.now()`) and origin epoch — correct to within microseconds for this
   file's sub-second deadlines, but not a real fix. Upstream fix: a `WindowDelegate::now_hint(&self,
   now: f64)` call, or an exposed clock handle.
2. **`ui_host::NativeHost<D>` cannot host this crate's two-phase boot.** `NativeHost::new(delegate: D)`
   requires an already-built delegate; `GpuContext::from_window` needs a live `Window`, which only
   exists after the event loop starts. `winit_app.rs` hand-rolls its own `ApplicationHandler` instead
   — see that file's own module docstring. Upstream fix: a two-phase `NativeHost` constructor, or a
   `WindowDelegate` hook that receives the just-created `Window`.
3. **`KernelSeam::submit_intents` has no real router.** `ui_contract::UiIntent` addresses a
   `SurfaceId`, not a plugin instance `u32`; the surface→instance map lives privately inside
   `kernel_runtime::KernelThreadState` on the kernel thread. `default_intent_exchange` is an explicit
   stub (echoes the surface back, no real kernel round trip). Real routing needs either exposing that
   map (in scope for a future `os-host` follow-up) or going through `ProgramBridge`'s own dispatch
   (`🧱️elements/ProgramBridge/`, forbidden here).
4. **`AppRuntime` has no real caret-focus signal at this layer.** The ticket asks for the caret-blink
   deadline to arm "only while the presented frame actually shows a visible editable caret";
   `OsHost::redraw` currently calls `self.caret.sync(&mut self.scheduler, now, true)` — always
   `true`. This is **exact parity with the old code** (which also blinked unconditionally every
   frame), not a regression, but it does not yet earn the "no caret, no timer" half of the
   optimization `deadlines::CaretBlink` was built to support (that half IS implemented and unit-tested
   in `deadlines.rs` — it's the wiring into a real focus signal that's missing).
5. **Two UNVERIFIED assumptions, flagged in-code, first build should confirm or refute:**
   - `std::time::Instant::now()` on `wasm32-unknown-unknown` (`winit_app.rs::recompute_control_flow`,
     `os_host::OsClock`) — plausible given this crate's `Cargo.toml` already enables `getrandom`'s
     `wasm_js` feature, but `ui_host`'s own `MonotonicClock` deliberately avoids `Instant` on wasm
     (ships a separate `performance.now()`-based `BrowserClock` instead) — a signal from that
     packet's own authors this bet may be wrong.
   - `EventLoopProxy<HostUserEvent>: Send + Sync` (`winit_app.rs::ProxyWaker`, needed so a kernel-
     thread `Waker::wake()` call can reach the winit thread). `HostUserEvent::RuntimeReady` carries
     `Rc<RefCell<AppRuntime>>` (not `Send`); if winit's `Send`/`Sync` impl for `EventLoopProxy<T>`
     requires `T: Send`, `ProxyWaker` will not compile. If this breaks, the fix is a raw OS-level wake
     primitive instead of reusing the product event channel for the kernel-thread wake.
6. **`AppRuntime::frame()`'s internal deadline bodies are untouched.** Wheel-zoom-settle,
   camera-settle and caret-blink's actual *business logic* (what to dispatch when a deadline fires)
   still lives exactly where it did, reading `app_now_ms()` directly — this packet did not thread
   `deadlines.rs`'s `arm`/`sweep_expired`/`CaretBlink` into those call sites inside `frame()` itself
   (a ~2700-line file this packet cannot compile-check). The **outer** gating (redraw only happens on
   real invalidation or an `OsHost`-armed deadline) is real and load-bearing; the **inner** bodies
   are deferred, correctly-scoped follow-up work, not silently dropped.

## Registrar-requests

`Cargo.toml` (`🎯️targets/🧊️wgpu/Cargo.toml`) is registrar-only (U7) — not edited. Requested changes,
exact lines (paths verified against this crate's own existing `ui_wgpu` dependency line for the
correct `../` depth):

**Add**, alongside the existing `ui_wgpu` line (~line 32):
```toml
ui_render = { path = "../../../../../../../../../🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust", package = "semio-framework-ui-render" }
ui_host = { path = "../../../../../../../../../🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust", package = "semio-framework-ui-host" }
ui_contract = { path = "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust", package = "semio-framework-ui-contract" }
```

**Resolve, not requested as a removal yet** (needs the peer `render-elements`/`runtime-*` packets to
land first, per this file's own `OsHost` deviation note): `ui_wgpu = { …, features = ["wgpu-engine"] }`
stays for now — `AppRuntime` still renders through it. Dropping it is a follow-up once the `DrawList`
pipeline itself migrates.

**The vello finding above is a recommendation, not a registrar-request** — no `Cargo.toml` change is
needed either way (`tiny-skia`/`resvg`/`usvg` are already present); the fix is code-only, in a
forbidden-to-this-packet file (`🧱️elements/EngineCanvas/`).

## Decisions

- `AppKernelSeam` is one concrete type for both platforms (not a cfg-selected pair) — see `kernel_seam.rs`'s
  own docstring; a strictly better resolution of U3's own anticipated two-impl table.
- `FrameScheduler` lives on `OsHost`, not `AppRuntime` — a soundness requirement (see os_host.rs), not
  a style choice.
- `winit_app.rs` hand-rolls its `ApplicationHandler` rather than using `ui_host::NativeHost<D>` — a
  real, evidence-based integration blocker (item 2 above), not an oversight.
- The waker-correctness fix (`kernel_runtime::install_waker`) was added even though the ticket's OWNS
  list doesn't name `kernel_runtime` explicitly, because switching to `WaitUntil`/`Wait` without it
  would silently reintroduce latency on every in-flight kernel round trip — the existing code's own
  doc comment already flagged this exact gap ("a real cross-thread `EventLoopProxy` wake… is not
  implemented"), so closing it is squarely inside "ends continuous redraw… without breaking anything
  that currently depends on `Poll`'s implicit re-poll".

## Deviations

- `OsHost`'s field set omits `engine`/`backend`/`surfaces`/`shell_model`/`theme`/`window` from the
  master plan's literal sketch — see `os_host.rs`'s own module docstring for the full reasoning.
- Cursor fidelity: `ui_render::CursorRequest` (5 variants) narrows `SemioCursor`'s 13; documented in
  `winit_app.rs`'s `redraw` doc comment. Does not currently surface as a real regression since
  `AppRuntime::frame()` already applies its own richer cursor internally before `redraw` returns.
- `WindowDelegate::handle_event`'s invalidation rule is coarse ("any normalized event is dirty"),
  not narrowed per event kind.
- IME/paste (`DispatchEvent::TextInput`/`Paste`/`Ime`) are accepted but not wired to any `AppRuntime`
  entry point — pre-existing scope (the old `SemioApp` never wired these either), not a new gap.
- The stale `semio_wgpu_mount` doc-comment (the old `start_frame_loop`-based disposal-gap description)
  was updated in place since it directly described code this packet deleted; no other doc comments
  elsewhere in `📦️glue.rs` referencing the old `SemioApp` by name were swept (two remain, both in
  `boot_app_role`'s docstring — low-value churn given the crate cannot be compile-checked either way).
