# Fix Procedural 3D Hover Resetting Flow Camera

## Root cause

Hovering a 3D preview mesh updates ephemeral `hovered_node_id` and re-renders the flow graph scene. `FlowGraphCanvasHost` then resyncs via `syncFlowSessionFromScene(..., applyCamera: false)`, which still called `loadFixtureJson`. That replaced the whole fixture **including camera**, snapping the live pan/zoom back to the document/default camera.

`applyCamera: false` only skipped the explicit `session.setCamera(scene.viewportJson)` path — it did not protect against `loadFixtureJson`.

## Fix

1. `FlowHost::apply_fixture` / `replace_fixture` preserve the live camera (same as undo/redo).
2. `FlowSession.cameraJson()` so pan/zoom can report viewport to the plugin.
3. Procedural 2d/3d: graph camera lives in plugin runtime; `nodeGraphViewport` is a view action (no VCS op); fixture diffs no longer emit `SetCamera`.
4. Flow host `emitInteractionState` reports viewport after gestures (aligned with `WasmGraphSurface`).

## Verification

- Vitest: `uses the live session camera for node graph wheel viewport actions` — passed (`vitest-viewport.txt`).
- Cargo `replace_fixture_preserves_live_camera` / `fixture_ops_ignore_camera`: blocked by concurrent `protocol` compile errors (`UiTreeItemNode` field migration — `selected`/`loading`/`waiting` vs `presence`/`dimmed`). Logs in `cargo-flow-core.txt`.
- Manual: pan/zoom the flow, hover 3D meshes — camera must stay put.
