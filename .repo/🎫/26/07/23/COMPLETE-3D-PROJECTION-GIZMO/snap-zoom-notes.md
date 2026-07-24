# Snap end zoom jump

## Cause
`WorldProjectionSnapDriver` read/wrote `camera.zoom` via `instanceof Three*Camera`. With duplicate `three` copies that fails, so:
1. Target pose got `zoom: 1` (fallback)
2. Mid-animation zoom was not applied (live camera kept the correct zoom → transition looked right)
3. End commit via `applyWorldCameraState` (duck-typed) / React remount applied `zoom: 1` → large jump

## Fix
- Duck-typed `worldCameraZoom` / `writeWorldCameraZoom` / `worldCameraIsOrthographic`
- `worldProjectionSnapZoom` preserves parallel zoom; maps perspective unit zoom → ortho default 50
- Lerp zoom during the snap
