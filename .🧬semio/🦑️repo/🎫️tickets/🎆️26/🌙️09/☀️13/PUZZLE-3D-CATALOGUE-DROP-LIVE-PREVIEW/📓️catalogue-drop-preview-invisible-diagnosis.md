# Puzzle 3D Catalogue Drop Live Preview

## Symptom

Dragging an object kind from the catalogue onto the 3D viewport places the object correctly on drop, but no mesh ghost follows the cursor during the drag.

## Root cause

1. **`WorldCanvas` uses `frameloop="demand"`** — scene updates do not paint until something calls `invalidate()`. `BrushPreviewGhost` already mounts `DemandInvalidateOnToken` for this; `CatalogueDropGhost` did not, so preview state could update while the canvas stayed on the last frame (invisible ghost until the user orbits).

2. **Brush preview took precedence** — when a retained `brushPreviewJson` was still latched (vortex hover), `visibleBrushPreview` rendered instead of `CatalogueDropGhost`, so an active catalogue drag showed no catalogue mesh even though drop commit still worked.

## Fix

- Align `CatalogueDropGhost` with `BrushPreviewGhost`: `brushPreviewGhostMeshUrl`, `DemandInvalidateOnToken`, `pickEnabled={false}` on the GLB.
- While `catalogueDropPreview` is live, render the catalogue ghost before the brush ghost.

## Files

- `🧰️framework/…/🌐️World3dHost/🟦️.tsx`
