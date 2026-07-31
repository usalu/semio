# Notes

## Bug

LOD grid step adjusts when scrolling in perspective panes, but not in orthographic panes (e.g. Top).

## Root cause

`LodFrameRunner` derives automatic LOD from `camera.position.distanceTo(target)`. Orthographic scroll changes `camera.zoom` (pixels-per-world-unit) without moving the camera, so distance and therefore grid step stay frozen.

## Fix

Derive an equivalent orbit distance from orthographic zoom via the inverse of `worldProjectionMatchedOrthoZoom` (reference FOV 50°), and feed that into `lodFromCameraDistance` in the shared infinite-world R3F bridge so every consumer (puzzle3d, cad, aggregator, …) inherits the behavior.
