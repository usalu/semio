# Spurious History Camera Rows on Widget Moves

## Symptom

Moving flow widgets (node drag, slider, etc.) appended history rows labelled like **camera camera** even though neither the flow-graph viewport nor the 3D preview camera had changed.

## Cause

React `NodeGraph` `emitInteractionState` (both DAG-wasm and Flow-wasm hosts) dispatched `nodeGraphViewport` on **every** pointer-up after non-pan gestures, alongside `interactionSelect` / `interactionHover`.

That routes to `Generation3dConfigMutation::SetCamera` (`nodeGraphViewport` → config lane). `SetCamera::diff` always authored a config VCS edit (no equality guard), so each widget move logged a view/camera command.

The wgpu `EngineCanvas` path already gates `nodeGraphViewport` with a digest (`publish_viewport` only when the viewport changed); React had no equivalent.

Flow/artifact `UpdateCamera` was never involved — `generation3d_fixture_operations` ignores fixture camera by design.

## Fix

1. **Renderer** — stop dispatching `nodeGraphActions.viewport` from `emitInteractionState`; keep camera persistence on `publishCamera` / wheel settle / fit-to-view / open hooks only (`🕸️NodeGraph/🟦️.tsx`).
2. **Plugin** — `SetCamera` and `SetPreviewCamera` config diffs no-op when the value is unchanged (parity with artifact `UpdateCamera`).
3. **Tests** — `node-graph-gestures` vitest law; generation3d config unit tests for no-op diffs.

## Verify

- Move a widget: history should show the graph edit only, no extra camera row.
- Pan/zoom the flow graph: after gesture settle, viewport still persists (unchanged paths).
