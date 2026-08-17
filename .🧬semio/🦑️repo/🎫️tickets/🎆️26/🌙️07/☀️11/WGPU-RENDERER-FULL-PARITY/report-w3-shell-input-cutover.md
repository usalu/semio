# w3-shell-input-cutover — final report

## Event routing/dispatch boundary traced

`ShellInput` (lines 15985-17917, shifts constantly) is the single funnel for ALL raw pointer/keyboard events today, both chrome and content, via `ui_wgpu`'s old immediate-mode `InputState<ActionDescriptor>`/`HitTarget`/`HitKind` system:

- `handle_pointer_button`/`handle_pointer_move`/`handle_pointer_wheel` call `input.hit_at(x, y)` against `hit_targets` registered during the paint pass (`interpreter::render_ui_node` → `ui_wgpu::render_widget` for content widgets; `dock`'s own paint fns for tabs/splits/resize handles). There is NO existing internal split between "chrome dispatch" and "content dispatch" — `handle_shell_hit` matches chrome/dock `control_id` prefixes first, and anything else falls through to a generic `if let Some(action) = hit.event { dispatch_action(action) }` / `HitKind::Input → focus_input` — this generic fallback IS today's content dispatch, just via the old model.
- `HitKind::Window` is a red herring name-wise: registered only for `dock.tab.*`/`dock.stack.*` chrome hit targets, not "window content."
- Confirmed via grep: zero references to `ui_wgpu::engine::Ui`, `UiTree`, `EventRouter`, or `dispatch_event` exist anywhere in `framework/renderer/wgpu` — `render_ui_node` still fully used the old path at the time of this check. `ShellState` had no `ui_wgpu::engine::Ui` field yet.

## Double-dispatch decision (from the brief's item 3)

Because no `Ui`/`UiTree` instance exists yet to dispatch into, and creating one requires a `ShellTypes` struct-field addition reserved for the Integrator, no `ui_wgpu::UiEvent`/`dispatch_event` calls were added. Once `w3-interpreter-cutover` lands `apply_tree`/`frame` and a `Ui` field gets added, the natural single choke point for `dispatch_event` is the `AppRuntime`-level pointer/keyboard wrappers in `ShellChrome` (lines ~21918+/22155+, which already own real winit access) — NOT inside `ShellInput`'s current `ShellState` methods, which have no per-frame render loop to piggyback on.

## MAJOR FINDING — `handle_keyboard_async` is entirely dead code

Grepped every call site repo-wide: it is never called. `ShellChrome`'s `AppRuntime::handle_key` calls the SYNC `handle_keyboard` directly and hand-duplicates just the search/find-Enter-activation logic around it. Consequences, all currently non-functional in the running app:
- The P4 app-declared keybinding dispatch (`match_app_keybinding`/`dispatch_app_keybinding`) never fires.
- Escape-deactivates-active-utility (P5) never fires.
- **Pressing Enter/Escape while a focused `Input` widget has text typed never commits it** (`commit_focused_input` is only reachable from a pointer-click-elsewhere flush, or from this dead path).

Could not fix the wiring itself — the call site is in `ShellChrome` (do-not-touch), and `ShellState` has no `self_weak`/spawn mechanism to replicate `AppRuntime`'s async-dispatch pattern from inside `ShellInput`.

**HIGHEST-PRIORITY WIRING REQUEST**: `AppRuntime::handle_key` needs to spawn `handle_keyboard_async` (mirroring its existing `spawn_app_task` pattern for `activate_search`/`activate_find`) instead of calling the sync `handle_keyboard` directly. This is a significant, user-visible bug (basic text-input commit via Enter/Escape is broken) — should be prioritized for the very next integrator pass.

## Fixes made (all in the reachable sync `handle_keyboard`)

1. **Escape precedence** — Escape previously never closed an open Select dropdown (`open_selects`) or the right-click `context_menu` at all. Added topmost-first handling (Select dropdown, then context menu) before falling through to the existing dock-drag/sync-card/search-find Escape paths, matching `ui_wgpu`'s overlay-manager precedence (`close_topmost_overlay`) even though these ad-hoc chrome overlays aren't wired through that stack.
2. **Focus-editable chord suppression** — the six hardcoded meta-chords (Cmd/Ctrl+P search, +F find, +[ / +] nav, +ArrowUp nav-up, +B / +Shift+B panel toggles) fired unconditionally, even while typing in a focused `Input` or the sync-attach draft buffer (e.g. Ctrl+B while typing would silently toggle a panel instead of inserting "b"). Added an `editing` guard (`input.focused_id.is_some() || self.sync_card_kind.is_some()`) gating all six, matching `os-shell.tsx`'s `isEditableEventTarget`/`useActionHotkey`.
3. **Cross-window Tab-order focus cycling** — was completely absent. Added `Tab`/`Shift+Tab` handling (idle-gated: no focus, no palette, no dock-drag) that cycles `active_window_id` across the entire dock via two new private associated fns: `dock_window_order` (a local read-only depth-first walk of `crate::dock::DockNode`, duplicated rather than reusing `dock`'s module-private `find_stack_path` since that's off-limits) and `cycle_active_window` (wraps around, uses `dock.set_stack_active` — not `sync_active_window`, which would leave the stack's own visible tab stale).

## Tests added
New `#[cfg(test)] mod shell_input_tests` (no prior test module existed anywhere in `shell` to extend — `ShellState` has 90+ fields and no `Default`, impractical to fixture for a full behavioral test, so scoped to the new pure `dock_window_order` fn): 3 tests covering single-stack tab-order flattening, Row/Column depth-first walk, and path-per-window correctness. All pass.

## Build/test output
`cargo check -p semio-framework-renderer-wgpu --lib`: clean, 0 errors (35 pre-existing warnings, none from this code).
`cargo test -p semio-framework-renderer-wgpu --lib`: **134 passed, 0 failed** (baseline was 121; +13 includes 3 new tests plus ~10 that landed concurrently from `w3-panel-dock-6anchor`'s `shell::panel_anchor_model_tests` — confirmed no interference).

## Files touched
- `framework/renderer/wgpu/rs/lib.rs` — `shell::ShellInput` region only.
