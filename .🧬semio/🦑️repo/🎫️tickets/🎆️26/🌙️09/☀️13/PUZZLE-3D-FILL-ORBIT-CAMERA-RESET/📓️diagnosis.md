# Puzzle 3D Fill Orbit Camera Reset

## Symptom

With fill active, orbiting the camera holds for a while, then the view snaps back as if reframed — often aligned with fill planning ticks expanding scene bounds.

## Root cause

`WorldProjectionContentFrame` keeps auto-fitting `worldSceneContentBounds` while `!viewportOwned`. Viewport ownership was only taken in `handleCameraChange`, which runs on orbit **end** (`WorldOrbitGated.onEnd`). During an active orbit/pan/zoom gesture, fill-driven `contentBounds` changes still triggered layout-time reframing.

## Fix

Wire `WorldOrbitGated.onCameraNavigate`: on `start`, set `cameraNavigating` and `viewportOwned` so fill-driven content framing is suppressed for the whole gesture, not only after release. Mount the content-frame rig via `world3dProjectionContentFrameMounted` and pass an explicit `enabled` flag instead of JSX boolean shorthand.

## Files

- `🧰️framework/…/🌐️World3dHost/🟦️.tsx`
- `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts`
- `🧰️framework/…/🎯️targets/⚛️react/🟦️.tsx` (re-exports)
