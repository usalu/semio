# 📓️ terra-ui-host-report

Packet `ui-host` — platform layer: window/event-loop hosting, event normalization, per-target
`ActiveBackend` alias. Crate `semio-framework-ui-host`
(`🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/`).

## Done

Replaced the three scaffold files (registrar-owned `Cargo.toml` untouched):

- **`🦀️backend_alias.rs`** — the four `cfg`-exclusive aliases exactly as instructed and as
  `ui_render::backend`'s own docstring prescribes: wasm32 → `backend_webgpu::WebGpuBackend`, macOS →
  `backend_metal::MetalBackend`, Windows → `backend_d3d12::D3d12Backend`,
  `all(target_os = "linux", not(wasm32))` → `backend_vulkan::VulkanBackend`. No enum, no box, no
  vtable. File docstring explains it is unverified — see Decisions.

- **`🦀️event.rs`** — platform → `ui_render` normalization, native (`winit`) and browser (raw DOM
  values, no `web_sys` types in these signatures) side by side:
  - `PointerRegistry` (native-only): device-slot + finger-id → distinct `PointerId` per simultaneous
    contact, even under `DeviceId::dummy()`. `pointer_id_from_web`/`pointer_kind_from_web_type` use the
    DOM's own already-unique `PointerEvent.pointerId`/`pointerType`.
  - `pointer_button_from_winit`/`pointer_button_from_web`, `modifiers_from_winit`/`modifiers_from_web`.
  - `normalize_wheel_delta_native` (`LineDelta`/`PixelDelta`) and `normalize_wheel_delta_web` (all
    three `deltaMode`s: pixel/line/page), sharing `WHEEL_LINE_HEIGHT_PX = 40.0` ported verbatim from
    `wgpu-old`'s `host.rs`.
  - `PhysicalKeyCode` (hand-rolled, partial-by-design) + `physical_key_from_winit`/
    `physical_key_from_web_code` (parses the DOM `KeyboardEvent.code` string), and
    `logical_key_to_dispatch_string`/`named_key_label` → DOM `KeyboardEvent.key`-shaped strings.
  - `ime_event_from_winit`: `winit::event::Ime` → `ui_render::ImeEvent` (`Disabled` → `Cancel`,
    documented).
  - `key_dispatch_event` assembles the final `DispatchEvent::KeyDown`/`KeyUp`.
  - 20 `#[cfg(test)]` unit tests: wheel-delta normalization (native 2 variants + web all 3 modes),
    modifier mapping (native + web), physical-vs-logical key mapping (incl. an AZERTY-style example),
    native/web physical-key agreement, IME preedit-cursor fallback and `Disabled→Cancel`, two
    simultaneous touches on one (dummy) device staying distinct, mouse+touch on the same device staying
    distinct, web pointer ids staying distinct, button mapping agreement/gaps.

- **`🦀️window.rs`** — native + browser hosts:
  - `WindowMetrics` (platform-neutral, `logical_size()`), `should_request_redraw` (a named alias for
    `FrameScheduler::should_render`, the one decision point both hosts funnel through).
  - Native (`mod native`, `#[cfg(not(wasm32))]`): `MonotonicClock`, `control_flow_for` (→
    `ControlFlow::WaitUntil`/`Wait`, **never** `Poll`), `cursor_icon_for`/`apply_window_cursor` (dedup
    ported from `wgpu-old`'s `cursor.rs`), `apply_ime_directive`, `NativeClipboard` (wraps `arboard`),
    `NativeHost<D: WindowDelegate>` implementing `winit::application::ApplicationHandler<WakeMessage>`
    (resize/scale-factor/modifiers/redraw/close handling, event normalization via `🦀️event.rs`),
    `NativeRuntime`/`WakeProxy` (the `EventLoopProxy`-backed wake transport a background thread uses)
    and `run_native`.
  - Browser (`mod browser`, `#[cfg(wasm32)]`): `BrowserClock` (`performance.now()`-backed —
    `std::time::Instant` doesn't exist on `wasm32-unknown-unknown`), `cursor_css_for`/
    `apply_canvas_cursor`, `BrowserClipboard` (+ the one real `async fn`, `read_text_async`),
    `CanvasHost<D>` — `requestAnimationFrame` requested only through one dedup'd
    `request_wake_from_state` call site (see Decisions), `ResizeObserver` + `devicePixelRatio` driving
    `WindowMetrics`, `visibilitychange` suspending `FrameScheduler`'s visibility flag while a resize
    mid-hidden-period is still applied and dirt accumulated while hidden is woken on becoming visible
    again.
  - `WindowDelegate` trait + `RedrawOutcome` — the seam that keeps this file ignorant of
    `ui_render::Dispatcher`/`FrameEngine` (those belong to `runtime-present`/`os-host`); nothing in its
    signature names a `winit`/`web_sys` type.
  - 9 `#[cfg(test)]` unit tests: scale-factor→metrics (incl. the zero-scale-factor park case),
    `should_request_redraw` clean/dirty/deadline-due (platform-neutral, exercises the exact three cases
    the packet brief names), native `ControlFlow` no-deadline/pending-deadline, all five cursors mapping
    to distinct `CursorIcon`s, CSS cursor keyword spelling.

Every sync `fn` in all three files carries the `// 🚫️async: U1 …` tag. `//#region`/`//#endregion`
structure throughout, docstrings start with a unique emoji, no comments inside function bodies (moved
two multi-line explanations that had drifted into function bodies up into doc comments during review).

## Acceptance: UNRUN (ruling U4 — `sol` runs every cargo command)

```
CARGO_TARGET_DIR=<session-scratchpad>/target cargo check -p semio-framework-ui-host --lib --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/target cargo check -p semio-framework-ui-host --all-targets --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/target cargo test  -p semio-framework-ui-host --lib --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/target cargo check -p semio-framework-ui-host --lib --target wasm32-unknown-unknown --timeout 600000
```

Expect every target to be RED right now, for two independent, expected reasons (report
`blocked-external` with the exact missing-item error per U2 — do not fix the other packets' crates):
- **Native** (macOS/Windows/Linux): `🦀️backend_alias.rs` names `MetalBackend`/`D3d12Backend`/
  `VulkanBackend`, none of which exist yet in their still-scaffold backend crates (see Decisions). `mod
  browser` in `🦀️window.rs` is `#[cfg(target_arch = "wasm32")]`-gated and is not compiled at all on a
  native `cargo check`, so the missing `web-sys` features below do not block native.
- **wasm32**: two independent blockers — `WebGpuBackend` doesn't exist yet either (same reason as
  native), and `mod browser` will not compile until the registrar adds the `web-sys` features listed
  below.

Non-cargo checks I did run (cheap, no build):
- `rustfmt --config-path ./rustfmt.toml --check --edition 2021` on all three files — clean (ran once,
  found only formatting nits — no parse errors — applied `rustfmt` in place, re-checked clean). This is
  the strongest signal available without `cargo` that the files are syntactically valid Rust.
- `grep -n "async fn"` across all three files — the only hit is the one sanctioned exception,
  `BrowserClipboard::read_text_async`, plus its own doc-comment mentions.
- `grep -n '\bdyn\b'` — every hit is `dyn FnMut(..)` inside `wasm_bindgen::closure::Closure<dyn FnMut(..)>`
  (permitted — U3 only bans `dyn` on *first-party* traits; `Closure`'s `T: ?Sized` parameter is not a
  first-party trait) or doc-comment prose quoting the rule itself. No `dyn` on any trait this packet
  defined (`ClipboardHost`, `WindowDelegate`).
- Brace/paren balance: `🦀️event.rs` 77/77, 270/270; `🦀️window.rs` 157/157, 430/430; `🦀️backend_alias.rs`
  0/0, 9/9.
- Manual line-by-line borrow-check reasoning for every `NativeHost`/`CanvasHost` method (documented to
  myself during writing, not saved as a separate file) — every place a shared/mutable `self`-field
  borrow could overlap a later `&mut self` method call was checked; none do given NLL's field-path
  splitting, since every `winit::window::Window`/`Rc<RefCell<..>>` borrow is re-acquired fresh at each
  use rather than held across an intervening `&mut self`/`&mut self.delegate` call.
- Verified every `winit` 0.30 API call against the **vendored source**
  (`~/.cargo/registry/src/index.crates.io-*/winit-0.30.13/src/`), not recall, per the packet's own
  ACCEPTANCE instruction: `ApplicationHandler` (all callbacks are sync — confirmed no `async fn` option
  exists in the trait at all, which is itself the proof cited in `window.rs`'s docstring),
  `DeviceId::dummy()` (a real, documented test-only constructor — used in `event.rs`'s multi-pointer
  tests since `DeviceId`'s inner field is `pub(crate)`), `KeyEvent`'s fields (confirmed `physical_key`/
  `logical_key`/`text`/`location`/`state`/`repeat` are all `pub` but the struct itself cannot be
  constructed outside `winit` because of its trailing `pub(crate) platform_specific` field — this is why
  `event.rs`'s key-mapping functions take `PhysicalKey`/`Key` directly rather than a whole `KeyEvent`),
  `Touch`'s fields (confirmed all `pub`, freely constructible — used directly in tests),
  `ModifiersState`/`Modifiers` (confirmed `ModifiersState` is the freely-constructible bitflag type;
  `Modifiers` itself is not constructible outside `winit`, so `modifiers_from_winit` takes the bitflag,
  matching `host.rs`'s own old call pattern `modifiers_from_winit(modifiers.state())`), `ControlFlow`,
  `EventLoop`/`EventLoopBuilder::build(&mut self)`, `ActiveEventLoop::create_window`, `EventLoopProxy`,
  `Window::default_attributes()`/`set_cursor`/`set_ime_cursor_area`/`set_ime_allowed`, and the
  `cursor-icon` crate's `CursorIcon` variant names (`Default`/`Pointer`/`Text`/`Grab`/`Grabbing` all
  confirmed present).

## Decisions

**The alias mechanism.** Exactly as prescribed — see `🦀️backend_alias.rs`'s own docstring. **Unverified
type names**: all four (`WebGpuBackend`, `MetalBackend`, `D3d12Backend`, `VulkanBackend`) — checked
2026-08-20, every one of the four backend target crates
(`🖼️render/🎯️targets/{🧊️webgpu,🍎️metal,🪟️d3d12,🌋️vulkan}/📦️packages/🦀️rust/📦️glue.rs`) is still an empty
`//#region Backend` scaffold with no `pub type`/`pub struct` at all. This file will not compile on any
real target until its matching `backend-*` packet lands; that is expected per this packet's own
instructions, not a defect here.

**How the browser host avoids duplicate `rAF` requests.** `CanvasHost::request_wake` is the *public*
entry point but `request_wake_from_state` (a free fn taking the shared `Rc<RefCell<CanvasHostState<D>>>`)
is the *only* place that ever calls `window.request_animation_frame`, and it is guarded by a
`raf_pending: bool` flag: if a frame is already scheduled, every further call (from an input event, a
resize, a scheduler invalidation, or `on_animation_frame`'s own deadline re-arm) is a cheap no-op. The
callback clears `raf_pending` **first**, before doing anything else, so an invalidation that fires
*during* the callback's own `redraw` call is free to schedule the *next* frame rather than being
silently dropped — one pending `rAF` absorbs any number of invalidations that arrive before it fires,
mirroring `FrameScheduler::should_render`'s own N-invalidations-coalesce-into-one-frame contract. The
real "was this worth it" check is `should_request_redraw` (the same fn the native host uses) called
*inside* the callback, never at the scheduling call site — scheduling only decides "is a check already
in flight", never "is there definitely something to paint", which sidesteps double-draining
`FrameScheduler`'s dirty mask from two different call sites.

## Registrar-requests

`web-sys`'s feature list in this crate's `Cargo.toml` (`[target.'cfg(target_arch = "wasm32")'.dependencies]`)
currently has only `["Window", "Document", "HtmlCanvasElement", "ResizeObserver", "PointerEvent",
"KeyboardEvent", "WheelEvent"]`. `🦀️window.rs`'s browser half needs these added (each gates a
type/method actually called): **`"Performance"`** (`BrowserClock`'s `window.performance()`),
**`"Navigator"`** and **`"Clipboard"`** (`BrowserClipboard`'s `navigator().clipboard()`), **`"Element"`**
(`canvas.client_width()`/`client_height()` in `on_resize`), **`"HtmlElement"`** and
**`"CssStyleDeclaration"`** (`apply_canvas_cursor`'s `.style().set_property(..)`), **`"EventTarget"`**
(`document.add_event_listener_with_callback` for `visibilitychange`).

## Deviations

- **Browser long-deadline waiting re-arms via `rAF`, not `setTimeout`.** `on_animation_frame` re-requests
  a frame through the same `rAF`-based `request_wake_from_state` whenever `FrameScheduler::next_deadline`
  is still pending, rather than computing the wait and using `setTimeout` for a far-future deadline (e.g.
  a 300ms debounce). This means a pending deadline costs one no-op callback per display refresh (~16ms)
  instead of a true zero-cost sleep — no pixels repaint on those ticks (the real gate is still
  `should_request_redraw` inside the callback), but it is not the same zero-idle-cost guarantee the
  native `ControlFlow::WaitUntil` path gets. A `setTimeout`-based path is a reasonable follow-up packet,
  not attempted here given the scope of this one.
- **`CanvasHost` does not itself wire up pointer/keyboard/wheel DOM listeners.** The packet brief's
  explicit ask for the browser host was `rAF` gating + `ResizeObserver` + DPR + page-visibility, which is
  what `CanvasHost` implements. Actually delivering DOM `pointerdown`/`pointermove`/`wheel`/`keydown`
  events into `WindowDelegate::handle_event` needs `event.rs`'s already-written, already-tested pure
  functions (`pointer_info_from_web`, `normalize_wheel_delta_web`, `physical_key_from_web_code`, …) fed
  by whatever owns the canvas's own `addEventListener` calls — not yet wired into `CanvasHost` itself.
  Flagging this explicitly rather than leaving it silently incomplete; a small follow-up can add the
  remaining `Closure`s to `CanvasHost::new` using the exact same `Rc<RefCell<..>>` pattern already there.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️backend_alias.rs` (replaced scaffold)
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️event.rs` (replaced scaffold)
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️window.rs` (replaced scaffold)
- `📦️glue.rs` untouched (already mounted all three correctly); `Cargo.toml` untouched (registrar-only).
