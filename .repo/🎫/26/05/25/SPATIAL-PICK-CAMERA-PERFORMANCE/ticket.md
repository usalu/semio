# Spatial Pick Camera Performance

**Repo MCP:** unavailable; ticket recorded manually.

## Summary

After committing a box, the spatial REPL rendered one raycast-enabled mesh per topology pick target (vertices, edges, faces, cells, …). Every pointer move during camera orbit ran Three.js raycasting across all of them and updated React hover state per target — causing severe pan/zoom lag.

## Fix

- Pick-target visuals are display-only (`raycast={raycastNone}`).
- Selection and hover use a single `SpatialPickRayCatcher` with CPU `spatialPickTargetsFromRay`.
- Hover is throttled (~32ms) and skipped while any mouse button is held (orbit/dolly).
- Hover state updates skip React re-renders when the key is unchanged.

## Files

- `spatial/js/renderer-r3f/index.tsx`
