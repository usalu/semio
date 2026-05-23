# Spatial Pick Camera Performance

**Repo MCP:** unavailable; ticket recorded manually.

## Summary

After committing a box, the spatial REPL rendered one raycast-enabled mesh per topology pick target (vertices, edges, faces, cells, …). Every pointer move during camera orbit ran Three.js raycasting across all of them and updated React hover state per target — causing severe pan/zoom lag.

## Fix

- Pick-target visuals are display-only (`raycast={raycastNone}`).
- Hover uses canvas DOM `pointermove` + CPU `spatialPickTargetsFromClientPoint` (no R3F mesh catcher).
- Removed invisible pick sphere that called `stopPropagation` on every pointer down and blocked `OrbitControls`.
- Grid helper children use `raycast={raycastNone}` so R3F does not raycast thousands of grid segments.
- `OrbitControls` `onStart`/`onEnd` pauses hover; canvas uses `frameloop="demand"` with invalidate on camera change.
- Hover throttled (~32ms); hover state skips React updates when unchanged.
- `derived.refresh` keyed on topology revision only (not every snapshot bump).

## Finalize freeze (follow-up)

On box commit, topology was applied while `interactionId` was still set. That mounted the full pick-target layer, ran `derived.refresh` (heavy brepjs), and flooded `pointer.move` into the runtime on the same frame as `createBoxFromCorners`.

- Pick layer, host hover, and drag-selection only run when `!interactionId` (idle REPL).
- `derived.refresh` deferred with `requestIdleCallback` and skipped during an active interaction.
- `pointer.move` only forwarded to the runtime when the interaction spec allows it.
- Canvas uses `frameloop="always"` during an interaction, `demand` when idle.

## Files

- `spatial/js/renderer-r3f/index.tsx`
