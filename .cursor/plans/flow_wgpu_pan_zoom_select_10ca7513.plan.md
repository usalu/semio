---
name: Flow Wgpu Pan Zoom Select
overview: Wire real pointer/wheel interaction into the wgpu NodeGraphScene (used by Flow) by mirroring the proven World3d interception pattern, and stop the render loop from clobbering live camera/selection state every frame — restoring pan, zoom, select, hover, and LOD (LOD is purely zoom-driven, so it self-heals once zoom works).
todos: []
isProject: false
---


## Root causes (verified by direct grep/read, not the sub-agent's word alone)

Flow's wgpu canvas already paints through the real `FlowHost`/`DagHost` engine (`framework/renderer/wgpu/rs/engine_canvas.rs`), so rendering itself is correct. The problem is entirely in the input pipeline and in how the render loop resyncs that engine from the document every frame:

1. **Dead input plumbing.** `handle_scene_pointer_move` and `handle_scene_pointer_button` in `framework/renderer/wgpu/rs/scenes.rs:337` and `:404` (which are the only call sites that would invoke `engine_canvas::node_graph_pointer_down/move/up`) have **zero callers anywhere in the repo** (confirmed via repo-wide grep). Only wheel is reached, through `render_component_scene` → `apply_scene_wheel` (`scenes.rs:614-630`), and even that discards the returned commands with `let _ = handle_scene_wheel(...)`.
2. **Command shape mismatch.** `engine_canvas::graph_interaction_commands` (`engine_canvas.rs:457-495`) sends `nodeGraphSelect` as `{"selectionJson": "[...]"}` (a JSON-encoded string), but `flow/plugin/rs/lib.rs:707-714`'s handler only reads `args.nodeIds` (a real array), falling back to `selection_ids(args)` which doesn't understand `selectionJson` either — so every selection attempt would silently clear selection instead of setting it. React's `flow-graph-canvas-host.tsx:288-289` sends the correct shape: `dispatch(nodeGraphCommands.select, { nodeIds: JSON.parse(session.selectedWidgetIds()) })`.
3. **Self-inflicted resync fighting.** `paint_node_graph` (`engine_canvas.rs:254-323`) unconditionally calls `sync_flow_host` (which does `host.replace_fixture(...)`, `host.set_camera(...)`, etc. from the scene's `fixture_json`/`viewport_json`) on **every single render frame** (60/s), regardless of whether the document actually changed. Since pointer/wheel handlers would mutate this exact same live `FlowHost` instance, any local pan/zoom/selection gets stomped back to the stale document value on the very next frame — before the async command round-trip can persist it. This is the same class of bug that `infinite/world/rs/lib.rs`'s `World3dState` already guards against with cached `scene_camera_json`/etc. fields that gate re-sync on an actual diff (`infinite/world/rs/lib.rs:147,836,855-856`).
4. **LOD needs no separate fix.** `dag_draw_lod(self.fixture.camera.zoom)` (`mathematical/graph/port/directed/dag/rs/lib.rs:1705-1707`) computes LOD live from `camera.zoom` on every paint. Once zoom is real and persists across frames, LOD selection follows automatically.
5. **No bounds tracking for NodeGraphScene**, unlike World3d's `ShellState.world3d_states: HashMap<String, World3dState>` (`shell.rs:198`), which `lib.rs`'s event handlers use to intercept pointer/wheel events directly (`lib.rs:181-275`) before/alongside generic shell dispatch. There is no equivalent map for node-graph surfaces, so there's no way for `lib.rs`'s real DOM event handlers to find which node-graph surface a pointer event landed in.

## Fix plan

### 1. Track node-graph surface bounds (`framework/renderer/wgpu/rs/shell.rs`, `scenes.rs`, `interpreter.rs`)
- Add a small `NodeGraphSurface { bounds: Rect, controller_id: String }` struct (in `scenes.rs` next to `render_node_graph`).
- Add `pub node_graph_states: HashMap<String, NodeGraphSurface>` to `ShellState` (mirrors `world3d_states` at `shell.rs:198`), initialized alongside it.
- Thread it through `render_chrome` → `render_ui_node` → `render_component_scene` the same way `world3d_states` is threaded today (`shell.rs:3301,3621`, `interpreter.rs:56`), and update the entry for `scene.surface_id` inside `render_node_graph` every frame with the current `bounds` and `scene.controller_id`.

### 2. Decouple `engine_canvas` node-graph functions from the transient scene node (`framework/renderer/wgpu/rs/engine_canvas.rs`)
- Change `node_graph_wheel`, `node_graph_pointer_down`, `node_graph_pointer_move`, `node_graph_pointer_up`, and `graph_interaction_commands` to take `surface_id: &str, controller_id: &str, inner: Rect` instead of `scene: &UiComponentSceneNode`. Determine flow-vs-dag from the already-stored `entry.node_graph` enum variant instead of re-parsing `capabilities_json`.
- Fix `graph_interaction_commands` to emit `nodeGraphSelect` as `{"nodeIds": [...]}` — parse `host.selected_widget_ids_json()` (a JSON array string) into `Vec<String>` and embed as a real array, matching `flow/plugin/rs/lib.rs:707-714` and React's shape exactly. Keep `nodeGraphHover`/`nodeGraphViewport` argument shapes as-is (`hoverJson`, `viewportJson`), which already match the plugin.
- Update `scenes.rs`'s remaining call sites (`render_node_graph`'s wheel/pointer branches) to pass `surface_id`/`controller_id`/`inner` instead of `scene`.

### 3. Stop clobbering live interaction state every frame (`framework/renderer/wgpu/rs/engine_canvas.rs`)
- Extend `EngineSurface` (or the `NodeGraphEngine` wrapper) with cached last-applied strings for each synced field: `fixture_json`, `selection_json`, `preview_off_json`, `catalogue_json`, `computing_json`, `lod_json`, `viewport_json` — mirroring `World3dState`'s `scene_camera_json`/etc. diff-gating pattern (`infinite/world/rs/lib.rs:147,836,855-856`).
- In `sync_flow_host` (and the equivalent `GraphHost::sync_from_scene_json` call for non-flow graphs), only apply a field when it differs from the cached value, then update the cache. This lets local pointer/wheel-driven camera/selection mutations persist visually across frames instead of being reset before the round-trip command lands.

### 4. Wire real pointer/wheel events (`framework/renderer/wgpu/rs/lib.rs`)
Mirror the existing World3d interception in `AppRuntime` (`lib.rs:181-275`), which already proves this pattern works:
- In `frame()`'s wheel block (`lib.rs:69-81`, synchronous): for any `node_graph_states` entry whose bounds contain the pointer, call `engine_canvas::node_graph_wheel(...)` and dispatch the returned commands asynchronously via `spawn_local` (same weak-upgrade + `try_borrow_mut` pattern already used for the asset-polling block at `lib.rs:106-134`), since `frame()` itself cannot be `async`.
- In `handle_pointer_button` (`lib.rs:181-225`, already `async`): for any containing `node_graph_states` entry, call `engine_canvas::node_graph_pointer_down`/`node_graph_pointer_up` (by `down`) and `.await`-dispatch the resulting commands, **in addition to** (not instead of) the existing `self.shell.handle_pointer_button(...)` call — unlike World3d, node-graph interaction must coexist with existing shell-level chrome (context menu, panel resize), so this must not early-return.
- In `handle_pointer_move` (`lib.rs:227-275`, already `async`): call `engine_canvas::node_graph_pointer_move(...)` for any containing surface on every move event (so hover works without the button held), dispatch commands, alongside the existing `self.shell.handle_pointer_move(...)` call.
- Generalize the existing `dispatch_world3d_commands` (`lib.rs:173-179`) into a shared `dispatch_commands` helper used by both world3d and node-graph paths.

### 5. Verification
- `cargo test` and `cargo build --target wasm32-unknown-unknown` for `semio-framework-renderer-wgpu` and its dependents (`flow-core`, `framework-graph`).
- Re-check whether `bun nx run @semio-tech/framework-os-dev:dev` still fails to compile for unrelated reasons (previous session noted `parse_plugin_entries`/borrow/`UiToggleNode.checked` errors); fix if still blocking, since it's required to actually observe the fix.
- Manually verify in the running Flow wgpu playground: scroll-wheel zoom changes node detail level (LOD), drag-pan moves the canvas and it doesn't snap back, click selects a node and the selection highlight persists, hover highlights the node under the cursor.

## Explicitly out of scope (pre-existing, separate gaps — will call out to the user, not fix here)
- Per-node `HitTarget`s and the `.node.`-suffixed context menu / double-click-to-open-instance behavior in `scenes.rs` are already broken independently of this issue (no per-node hits are registered since the vello/engine_canvas rewrite) — a separate follow-up.
- `moveMediaNode` persistence for **non-Flow** (media/DAG) node graphs on drag is not wired to a concrete node id/position yet; Flow itself doesn't use this command (it commits via `nodeGraphEdit`/`setFixture` instead), so it's not required for the user's reported Flow issue, but is a natural follow-up for parity on the generic `GraphHost` path.
