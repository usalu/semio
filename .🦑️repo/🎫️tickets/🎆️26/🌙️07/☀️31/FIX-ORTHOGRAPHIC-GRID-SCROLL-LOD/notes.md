# Notes

## Bug

LOD grid step adjusts when scrolling in perspective panes, but not in orthographic panes (e.g. Top).

## Root cause

`LodFrameRunner` derives automatic LOD from `camera.position.distanceTo(target)`. Orthographic scroll changes `camera.zoom` (pixels-per-world-unit) without moving the camera, so distance and therefore grid step stay frozen.

## Fix

Derive an equivalent orbit distance from orthographic zoom via the inverse of `worldProjectionMatchedOrthoZoom` (reference FOV 50°), and feed that into `lodFromCameraDistance` in the shared infinite-world R3F bridge so every consumer (puzzle3d, cad, aggregator, …) inherits the behavior.

## Follow-up: missing grid on the right when zoomed

Screenshot `grid-missing-right.png` shows a hard vertical cutoff — drei's `Grid` plane is sized from `fadeDistance` (`localPosition *= 1 + fadeDistance`). Fade used to track only camera Z height, so orthographic zoom (which changes visible world extent without moving the camera) could make the frustum larger than the plane.

### Fix
- `cameraGridVisibleRadius` — frustum radius on the grid plane from ortho zoom or perspective FOV
- `cameraGridFadeDistance` — at least `visibleRadius * 32` so corners stay near-full opacity under fadeStrength 1.5
- `WorldLodGridHelper` passes live viewport size every frame
