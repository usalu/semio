# EngineCanvas Retained Fixture Migration

## Evidence

Native 130 recorded six EngineCanvas/NodeGraph paint failures. The direct test helpers called `scenes::render_component_scene_step` without the Interpreter's prior `mount_scene_identity`; the new engine-token bind correctly rejected that unmounted owner. Source classification and anchors are in `📓️terra-native130-engine-canvas-triage.md`.

## Repair

The shared test helper now publishes a one-node `UiDocumentLease` containing the real encoded NodeGraph, TiledMap, or Board2d surface, drives `interpreter::render_ui_document_step`, and obtains the actual generated `ScenePointerTarget.host_id` through `retained_scene_target_at`. Cleanup requests document close and advances the established CPU fixture bridge, which closes EngineCanvas by that generated host. The direct World3d unit helper remains unchanged.

The migrated laws are:

- `node_graph_window_attaches_the_flow_engine_and_paints_a_non_empty_draw_list`
- `node_graph_paint_publishes_its_captions_over_the_engine_raster`
- `tutorial_semantic_points_resolve_through_live_graph_geometry`
- `tiled_map_and_board_windows_attach_their_engines_on_the_same_production_seam`
- `a_board_window_paints_its_tool_run_trace_lane_and_echoes_the_cursor`
- `tiled_map_paint_reserves_the_visible_tiles_react_fetches`

Every internal EngineCanvas, raster, geometry, and host assertion now uses the generated host ID. Public scene/action identity remains the document surface ID.

## Next RED boundary

The retained tutorial law preserves its public point ID. `Shell::tutorial_resolve_gesture_point` resolves the generated engine host for the public window and calls `resolve_tutorial_surface_point(host_id, point)`. That function currently rejects unless `point.id == host_id`, although `point.id` is the public wire window ID and the `EngineSurface` separately retains that wire ID. The migrated law should therefore capture a production RED before the check is changed to compare the point ID with the mounted engine's wire surface ID.

Both changed Rust fixture files pass `rustfmt --edition 2021 --check`. Native execution is root-owned.
