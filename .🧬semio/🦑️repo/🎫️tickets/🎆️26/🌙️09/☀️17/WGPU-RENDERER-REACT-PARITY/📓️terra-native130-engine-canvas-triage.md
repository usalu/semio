# Native130 Engine Surface Triage

## Scope and evidence

This is a read-only source audit of [renderer-native130-full/failures.json](🗑️generated/astra-runtime/renderer-native130-full/failures.json) and its [run receipt](🗑️generated/astra-runtime/renderer-native130-full/run.log). The receipt has 53 failures. This packet classifies the nine within the requested EngineCanvas, NodeGraph, Map-gesture, and fixed-slot scope. I did not run Cargo or edit production or tests.

## Direct scene-paint faults are fixture defects

Six failures are caused by direct test helpers bypassing the only production admission seam:

- `a_board_window_paints_its_tool_run_trace_lane_and_echoes_the_cursor`
- `tiled_map_and_board_windows_attach_their_engines_on_the_same_production_seam`
- `tiled_map_paint_reserves_the_visible_tiles_react_fetches`
- `node_graph_window_attaches_the_flow_engine_and_paints_a_non_empty_draw_list`
- `node_graph_paint_publishes_its_captions_over_the_engine_raster`
- `tutorial_semantic_points_resolve_through_live_graph_geometry`

The failing helpers call `scenes::render_component_scene_step` directly:

- [wgpu-engine-surfaces test helper](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:261) reaches its direct call at line 279.
- [wgpu-node-graph test helper](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs:82) reaches its direct call at line 98.

A direct scene walk can create an engine surface in phase 4, then fails when it tries to bind that surface token. [The bind requires an already mounted scene owner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1331); [phase 4 faults when the bind fails](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3022). There is only one production caller of `render_component_scene_step`: [the Interpreter slot painter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2469), which first constructs the exact `ScenePointerTarget` and calls [`mount_scene_identity`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2471).

The production mount records an identity-bearing owner under `host_id`, rejects a concurrent different owner, and is required before the engine token is accepted. See [`mount_scene_identity`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1310).

### Exact fixture repair

Replace both direct helpers with one retained-document driver shaped like [`paint_retained_map_document`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:155):

1. Publish a one-node `UiDocumentLease` containing the real `Component::Surface` record.
2. Drive `interpreter::render_ui_document_step` to completion, rather than `render_component_scene_step`.
3. Read the mounted owner through `retained_scene_target_at`; use its `host_id` for engine/scene assertions and exact cleanup, and its `surface_id` only for public wire-action assertions.
4. Drive explicit document/component retirement and close the attached engine by that returned `host_id`.

Do not have the test directly call `mount_scene_identity`: that would duplicate the exact Interpreter protocol these integration laws cover. No live production path calls `render_component_scene_step` without that Interpreter mount.

## Map lifecycle gestures are keyed by host ID

The two failing lifecycle laws already paint through `render_ui_document_step`. Each retrieves the resulting `ScenePointerTarget` ([refresh line 774](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:774)) then starts the gesture through `tiled_map_pointer_down_into(&owner, ...)`.

That API stores the gesture under `owner.host_id` ([source](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8404)). Their assertions query `tiled_map_drag_active(surface_id)` at [refresh lines 776 and 783](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:776) and [replacement lines 806, 811, 814, and 819](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:806). [`tiled_map_drag_active`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8536) treats its argument as the internal state key, so the wire document ID yields a false negative.

### Exact fixture repair

- Use `owner.host_id` for `tiled_map_drag_active` and the first argument of `tiled_map_pointer_cancel_into`, `tiled_map_pointer_up_into`, and `tiled_map_pointer_leave_into`.
- Preserve `owner.surface_id` as the second, public `surface_id` argument of APIs that publish actions.
- After a key replacement, inspect the original host ID for retirement; inspect the successor host ID after its down event and after removal.
- Close EngineCanvas by the mounted host ID. The current lifecycle helper calls `drop_engine_surface(surface_id)`, which cannot find an allocation created with the generated host key.
- Stop using [`map_fixture_owner`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:151) in retained-document paths: it invents `host_id = "map.fixture"`.

This is fixture protocol drift, not evidence of a user-driven Map gesture defect.

## Fixed-slot receipt requires a fixture update

The receipt measures:

```
EngineSurfaceRegistry: capacity 256, element bytes 76216, owner bytes 16
committed element bytes: 75824
```

The current law measures [`size_of::<EngineSurfaceSlot>()`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:629). The common fixed-slot fixture records 75824 bytes for that one table. The two other EngineCanvas table measurements still agree: 352 and 384 bytes.

Update only `renderer::engine_canvas / engine_canvas::EngineSurfaceRegistry / elementSizeBytes` from 75824 to 76216. Keep the 256-slot capacity and 16-byte owner budget. Do not weaken or bypass the heap-first measurement.

## Outcome

Within scope, all eight scene/gesture failures are test-harness identity/admission drift, and the ninth is committed-budget drift. I found no source evidence here of a production EngineCanvas, NodeGraph, Board2d, or TiledMap attach failure.

