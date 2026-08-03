# W2 — Keyboard/IME/Focus Input Wiring (WS2a)

Re-verified `report-w3-shell-input-cutover.md`'s findings against current code first — all held, line
numbers had drifted.

## 1. `handle_keyboard_async` wiring — DONE
`AppRuntime::handle_key` (in `shell::ShellChrome`, only this fn's body touched) now spawns
`handle_keyboard_async` via `self.self_weak` + `spawn_app_task`, mirroring the existing
`on_button`/`on_move` pattern, instead of calling sync `handle_keyboard` directly. The old
hand-duplicated search/find-Enter-activation logic is gone (not moved) — `handle_keyboard_async`
already does it. Enter/Escape now reaches `commit_focused_input` on every keystroke.

## 2 & 4. Content-focus arbitration + `dispatch_ui_event` wiring + Tab traversal — DONE, one documented gap
Added `events::EventRouter::is_focused()` and `engine::Ui::window_has_focus(window_id)` as additive
read-only accessors in `ui/wgpu/rs/lib.rs`. The live `Ui` instance lives in `UI_ENGINE`, a
`thread_local!` **private to `interpreter`** (off-limits region) — `shell` can't reach it directly.
So `ShellInput` tracks content focus itself via a new `CONTENT_FOCUS` thread-local, fed by
`UiCommand::FocusChanged` returned from its own `dispatch_ui_event` calls. `handle_keyboard_async`
now routes keys to content (via `ui_event_from_key_action` → `dispatch_ui_event`) when idle and the
active window is tracked as content-focused. Tab traversal falls out for free —
`EventRouter::dispatch`'s `KeyDown{"Tab"}` arm already calls `focus_next`/`focus_prev` internally.

**Gap**: pointer-click-driven focus (set by the off-limits `render_ui_node`/`dispatch_pointer_events`
path) isn't observed by this tracker. Wiring request: a one-line
`pub fn window_has_focus(window_id: &str) -> bool` exposed from `interpreter` next to
`dispatch_ui_event`, for whoever owns that region (SceneHost/WS1 workstream) to call.

## 3. MouseButton mapping — already fixed upstream
`ui_wgpu::host::mouse_button_to_i16` already does real winit mapping and already feeds `ShellInput`'s
`button == 2` right-click check correctly. One residual inconsistency remains in
`pointer_button_from_code`, but it's inside the off-limits `RetainedEngineCutover` region — flagged
here, not touched.

## 5. IME — not implemented, scoped out
Wiring it properly needs edits to `input::PointerCallbacks` (new `on_ime` field),
`host::dispatch_window_event` (new `Ime` arm), the bootstrap `PointerCallbacks{...}` construction
site, and a brand-new `AppRuntime::handle_ime` method — four boundary crossings beyond this ticket's
narrow `handle_key`-body exception. Follow-up item.

## Tests
7 new tests added to the existing `shell_input_tests` module (no new test files) covering
`ui_event_from_key_action`'s mapping and `CONTENT_FOCUS` tracker behavior.

## Build/test status
- `cargo check -p ui_wgpu --features engine`: clean, 0 errors.
- `cargo test -p ui_wgpu --features engine`: **205 passed, 0 failed**.
- `cargo check -p semio-framework-renderer-wgpu --lib`: clean, 0 errors (verified before the blocker below appeared).
- `cargo test -p semio-framework-renderer-wgpu --lib`: **blocked** by an unrelated, actively-churning
  compile failure in `kernel_3d_brepkit` (missing `Deserialize` impls), reached transitively via
  `flow_core → flow_extension_brep → kernel_3d_brepkit`. Confirmed via `git status` as the concurrent
  `NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT` ticket's in-progress work (untracked
  `kernel/3d/brep/rs/src/{arena,topo,history}.rs`); error count dropped 40→12 between retries a few
  minutes apart, confirming active fixing elsewhere. Not chased further, per instructions.

## Files touched
- `framework/renderer/wgpu/rs/lib.rs` — `shell::ShellInput` region (new `CONTENT_FOCUS` tracker +
  helpers + routing branch + 7 tests) and `AppRuntime::handle_key`'s body only in `shell::ShellChrome`.
- `ui/wgpu/rs/lib.rs` — additive-only `EventRouter::is_focused` and `Ui::window_has_focus`.
