# w2-scene-wiring — final report

**File touched:** `framework/renderer/wgpu/rs/lib.rs` (only inside `scenes::SceneRuntime`, `scenes::SceneInput`-adjacent helpers, and `scenes::RenderEntry` — owned regions).

## How pointer events reach ComponentScene nodes today (traced)

Two independent pointer paths exist:

1. **Bespoke per-surface hosts** (`World3d`, `NodeGraph`, `TiledMap`, `Board2d`): the top-level `AppRuntime::handle_pointer_button`/`handle_pointer_move`/`frame()` keep dedicated `HashMap<String, ...State>` registries (`world3d_states`, `node_graph_states`, `tiled_map_states`, `board2d_states`) populated during render, and iterate them directly on each real OS pointer/wheel event, calling `engine_canvas::node_graph_pointer_*`/`tiled_map_*`/`scenes::puzzle_board_*`/`handle_world3d_*` and dispatching via `AppRuntime::dispatch_actions`. Fully independent of `render_component_scene`.
2. **Everything else**: `render_component_scene` runs every render frame and receives `ctx.input` (`InputState<ActionDescriptor>`), kept live by `ShellState::handle_pointer_button`/`handle_pointer_move`. `handle_scene_wheel`/`handle_scene_pointer_button`/`handle_scene_pointer_move` (in `scenes::SceneInput`) already contained correct per-`SurfaceKind` logic but had zero call sites anywhere in the file — confirmed dead — and `apply_scene_wheel` discarded its result (`let _ = ...`).

## Genuinely dead vs. already-working surfaces (verified, not assumed)

- Dead (no interaction at all): `Canvas2d`, `Paint2d`, `TextEditor`, `InkCanvas`, `GraphTimeline`, `Table`, `VirtualFileSystem` — matches the ticket's candidate list.
- Working via bespoke hosts: `World3d`, `NodeGraph`, `TiledMap`, `Board2d` — confirmed via their dedicated state maps in `AppRuntime`, and confirmed that `handle_scene_wheel`/`handle_scene_pointer_button`/`handle_scene_pointer_move` also contain real (would-be-duplicate) `NodeGraph`/`TextEditor` arms, so these must stay excluded from the generic path to avoid double-dispatch.

## What was wired (in `scenes::RenderEntry`)

- Added `SceneSurfaceState::last_pointer_pos` and a cheap `scene_pointer_edge_state()` reader (in `SceneRuntime`).
- Added `scene_has_bespoke_pointer_dispatch(kind)` — excludes `World3d`/`NodeGraph`/`TiledMap`/`Board2d`.
- Added `apply_scene_pointer(scene, bounds, ctx)`, called from `render_component_scene` right after `apply_scene_wheel`: edge-detects button down/up via `pointer_was_down` and only re-runs move handling when `(x,y)` actually changed frame-to-frame, then calls `handle_scene_pointer_button`/`handle_scene_pointer_move` and queues resulting actions via `ctx.input.queue_event(...)`.
- Fixed `apply_scene_wheel` to loop over `handle_scene_wheel(...)`'s results and `queue_event` each one instead of `let _ = ...`, gated with the same bespoke-exclusion check.
- Added `#[cfg(test)] mod render_entry_tests`: 6 tests covering bespoke exclusion, action dispatch for a previously-dead surface (`Canvas2d` button, `InkCanvas` wheel → `setCamera`), edge-triggering (no re-fire while held), and non-interference with bespoke surfaces (`NodeGraph`).

## Important out-of-scope finding (flagged, not fixed)

`ctx.input.queue_event`/`pending_events`/`drain_events` (the only producer-side API reachable from `render_component_scene`) is entirely disconnected from any consumer repo-wide — `drain_events` has zero callers, and `AppRuntime::frame()`'s `clear_frame()` doesn't even clear `pending_events`. This is a pre-existing bug (already silently drops `render_text_editor`'s own `queue_event` calls for submit/format actions) that lives in `AppRuntime`/`shell` — reserved for Wave 3, out of this agent's region. Used `queue_event` anyway since it's the correct-shaped, established API; flagged the drain-side fix via `spawn_task` (`task_4d596467`) with exact file/line pointers and the fix pattern to mirror (`graph_actions`/`map_actions` dispatch already in `AppRuntime::frame()`). **Wave 3 must wire this or the whole fix here is inert at runtime — treat as a blocking dependency for Wave 3's ShellInput/renderer-cutover workstream.**

## Verification

- `cargo check -p semio-framework-renderer-wgpu --lib` — clean (only pre-existing warnings).
- `cargo test -p semio-framework-renderer-wgpu --lib` — **66/67 passed**; the 6 new `render_entry_tests` all pass. The 1 failure (`dock::tests::apply_drop_tab_moves_window_across_stacks`) is in `dock`, owned by concurrent agent `w2-dock-dnd`, mid-refactor on that exact function at time of this check — unrelated to this change.
- No `w2-block-table` `BlockList` wiring request had arrived by the time this agent finished — coordinator applies that separately.
