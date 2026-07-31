# Notes

## Bug

Catalogue drag-drop preview in aggregator Top (orthographic) window did not track the cursor–grid intersection. Perspective worked.

## Root cause

`raycastGroundPoint` in `framework/renderer/react/index.tsx` always built a perspective ray (`origin = camera.position`). Orthographic Top needs parallel near→far unproject rays.

## Fix

- Added `worldRayFromNdc` with `isOrthographicCamera` duck-typing (near→far for ortho, pinhole for perspective).
- `raycastGroundPoint` and `axisDragParam` both use it.
- Vitest covers ortho Top center/offset and perspective center.
