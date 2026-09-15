# Puzzle 3D Fill Orbit Camera Reset

## Symptom

With fill active (or when fill planning starts), the camera jumps or slowly reframes as if the view were being auto-fit — fill should not move the camera whether the tool is armed or not.

## Root cause

`WorldProjectionContentFrame` keeps auto-fitting `worldSceneContentBounds` while `!viewportOwned`. Two mechanisms:

1. **During orbit** — viewport ownership was only taken in `handleCameraChange` (`WorldOrbitGated.onEnd`). During an active gesture, fill-driven `contentBounds` changes still triggered layout-time reframing.
2. **During fill** — fill provisional placements are published on the instances lane with `provisional: true`. `worldSceneContentBounds` included those positions, so every planning step changed the bounds key and reframed seeded projection panes (and could fight a user-owned perspective pose when `viewportOwned` was reset).

## Fix

1. Wire `WorldOrbitGated.onCameraNavigate`: on `start`, set `cameraNavigating` and `viewportOwned` so fill-driven content framing is suppressed for the whole gesture, not only after release.
2. Frame only **committed** instances: `world3dFramingInstances` drops `provisional` records before `worldSceneContentBounds`.
3. Lock continuous projection reframing while fill is armed or any provisional instance is visible: `world3dFitProjectionContent(..., lockContentFrame)`.

## Files

- `🧰️framework/…/🌐️World3dHost/🟦️.tsx`
- `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts`
- `🧰️framework/…/🎯️targets/⚛️react/🟦️.tsx` (re-exports)
