# Wave 2 → Wave 3 integrator wiring pass

Applied directly by the orchestrator (acting as Integrator for choke-point files) after all 7 Wave 2 agents landed, closing out the small mechanical wiring requests they filed. All edits in `framework/renderer/wgpu/rs/lib.rs`.

## 1. `queue_event`/`drain_events` consumer wired (was: zero consumers anywhere)
In `AppRuntime::frame()`, right after `self.shell.render_chrome(...)` returns, added:
```rust
let scene_events = self.input.drain_events();
if !scene_events.is_empty() {
    let runtime = self.self_weak.clone();
    spawn_app_task(async move {
        if let Some(runtime) = runtime.upgrade() {
            if let Ok(mut app) = runtime.try_borrow_mut() {
                app.dispatch_actions(scene_events).await;
            }
        }
    });
}
```
This is the exact pattern already used for `graph_actions`/`map_actions` dispatch a few lines below. Unblocks all of Wave 2's `apply_scene_pointer`/`apply_scene_wheel`/text-editor dispatch work at runtime.

## 2. `sync_dock` now uses keyed diff instead of teardown-rebuild
Replaced the `layout_override` branch's `self.dock.root = dock_from_window_layout(&layout.root); self.dock.active_window_id = ...` with `self.dock.apply_layout_diff(&layout);` (falling back to the old active-window-id resolution only if `apply_layout_diff` left it `None`, since it already preserves the current tab when still present — the existing subsequent `sync_active_window(id)` call still re-applies the outer `active_window_id` afterward, unchanged).

## 3. `poll_pending_assets` wired for UI-image fetch + world3d asset polling
- Added `let ui_images = collect_pending_ui_image_fetches();` alongside the existing `glb`/`map` collection, widened the early-return guard to include it.
- Native branch: added a `for item in ui_images { ... apply_ui_image_bytes(...) }` loop mirroring the existing glb loop.
- Wasm branch: added the equivalent fetch-then-apply-inside-borrow pattern.
- Both branches now also call `self.shell.poll_world3d_assets()` (blocking on native, awaited inside the wasm borrow) — this closes a SEPARATE, more severe pre-existing bug discovered while wiring: `ShellState::poll_world3d_assets` (which calls `fetch_pending_glb_meshes`/`fetch_pending_reference_images`) had **zero callers anywhere in the file**, meaning world3d GLB mesh loading and reference-image loading were themselves dead code, not just the terrain gap `w2-world3d` flagged. All three (GLB, reference images, terrain — after item 4 below) are now reachable.

## 4. Terrain tile fetch wired
Added `fetch_pending_terrain_tiles` to the `use infinite_world::{...}` import list and to `ShellState::poll_world3d_assets`'s body (third line, alongside the two pre-existing calls). Reachable via item 3's new `poll_world3d_assets()` call site.

## 5. `RenderEntry`'s `SurfaceKind::BlockList` arm
Added `SurfaceKind::BlockList => render_block_list(scene, bounds, ctx),` to the match statement (was falling through to the placeholder catch-all) and removed the `#[allow(dead_code)]` w2-block-table had left on `render_block_list` pending this wiring.

## Deliberately NOT fixed — flagged for a future pass
**Paint2d left-click pointer-tool dispatch** (`handle_scene_pointer_button`'s `SurfaceKind::Paint2d` arm only handles `button == 1 || button == 2` for pan; `button == 0` falls through to nothing). Investigated `framework/surface/paint/rs/lib.rs` for the real plugin action verb a left-click-to-paint dispatch should use and could not find a clear, confirmed handler function/verb there in the time available — per CLAUDE.md's "must not assume, must validate assumptions" rule, did not guess at an unverified action string. Left as an open item rather than risk a silently-wrong plugin dispatch.

## Verification
First `cargo test -p semio-framework-renderer-wgpu --lib` run surfaced 2 real compile errors: `collect_pending_ui_image_fetches`/`apply_ui_image_bytes` are defined inside `pub mod interpreter { ... }` and weren't imported into `AppRuntime`'s module scope. Fixed by adding `use interpreter::{apply_ui_image_bytes, collect_pending_ui_image_fetches};` alongside the neighboring `use plugin_bridge::...`/`use infinite_world::{...}` imports (matching the file's existing unprefixed-sibling-module convention at that exact location).

Final: `cargo test -p semio-framework-renderer-wgpu --lib` — **121 passed, 0 failed.**
