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

## Files

- `spatial/js/renderer-r3f/index.tsx`
