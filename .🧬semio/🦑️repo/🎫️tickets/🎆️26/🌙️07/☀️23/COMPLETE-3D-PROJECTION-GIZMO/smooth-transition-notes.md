# Smooth Projection Transitions

## Changes

- `worldProjectionOrientationsEqual` + `worldProjectionTransitionPose` in `infinite/world/r3f/index.tsx`
  - Mode-only (orientation unchanged): keeps position/target/up; swaps projection family + spec via `applyWorldProjectionToCameraState`
  - Orientation change: keeps target + orbit distance + family-safe zoom; recomputes eye via `computeWorldProjectionPose`
- `WorldProjectionSnapDriver` uses transition helper with `currentProjectionSpec`
- `WorldOrbitViewControls` accepts `externalPendingSpec` + `onExternalPendingSpecClear` (gizmo internal pending unchanged)
- Framework `handleProjectionKindChange` sets external pending instead of instant `computeWorldProjectionPose`
- CAD `handleProjectionSpecChange` fixed `spec.mode` via `worldProjectionFamily`; routes pane through pending snap

## Tests

`vitest-transition-pose.txt` — 140 passed (includes 2 new `worldProjectionTransitionPose` cases)
