# Findings

## Root cause

`World3dHost` treated live orbit as shared document state:

1. Orbit end called `dispatch("setCamera", …)` into the plugin fixture.
2. `seedKey={scene.cameraJson}` reseeded every mounted host when that JSON changed.
3. Extra/dropped Puzzle 3D windows all render the same `windowUiByKind` scene, so they all followed the shared camera.

## Fix

Viewport-owned camera in `World3dHost`:

- Orbit / projection / zoom-to-selection update local state only (no shared `setCamera`).
- Seed key becomes stable `viewport:{epoch}` after detach so sibling mounts are not reseeded.
- External `scene.cameraJson` changes (view preset, focus, example load) reattach all viewports on purpose.
