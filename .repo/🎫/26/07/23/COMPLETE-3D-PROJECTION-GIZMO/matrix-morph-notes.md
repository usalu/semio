# Projection-matrix morph (continuation of smooth-transition work)

Prior work in this ticket unified pane/gizmo projection changes through `WorldProjectionSnapDriver`'s
280ms pose tween (position/up/zoom), but the *projection matrix itself* still flipped instantly at the
camera remount in `WorldProjectionRig` (keyed by `mode.kind`), and cross-family zoom lerped onto the
wrong (old) camera. Oblique shear, two-point shift, and the curvilinear fisheye/panini pass all
popped on/off at the same remount boundary.

## What changed (`infinite/world/r3f/index.tsx`)

- `worldProjectionPerspectiveFov` — extracted the fov-resolution logic duplicated in `WorldProjectionRig`.
- `worldProjectionMatchedOrthoZoom` / `worldProjectionMatchedPerspectiveDistance` / `worldProjectionFovCompensatedDistance`
  — scale-preserving zoom/distance math (pixels-per-world-unit matching, dolly-zoom).
- `worldObliqueShearMatrix` — extracted from `WorldProjectionMatrixDriver`, reused by the goal-matrix builder.
- `worldProjectionGoalMatrix` — builds the destination projection matrix for a spec at the live viewport
  size using real `ThreeOrthographicCamera`/`ThreePerspectiveCamera` instances (so it matches drei's own
  camera construction exactly), plus oblique shear / two-point shift.
- `worldProjectionMorphMatrix` — element-wise projection-matrix lerp (valid persp↔ortho via the w-row,
  persp↔persp via cot-space fov interpolation, shear/shift ramp linearly).
- `worldProjectionTransitionPose` — extended with optional `viewport`/`fov` on `live`; when supplied,
  persp→ortho / ortho→persp / persp-fov-change mode-only switches preserve apparent scale via the helpers
  above instead of the legacy zoom-50/zoom-1 defaults. No-viewport callers keep the old behavior exactly.
- `WorldOrbitViewSnapGateContextValue` gained a `morphRef` (refs only, no re-renders) carrying
  `{ fromSpec, toSpec, fromMatrix, fromFov, toFov, eased, holding }`, read by `WorldProjectionMatrixDriver`
  (early-returns while a morph is active — the goal matrix already bakes in the destination shear/shift)
  and `WorldCurvilinearPass` (ramps `uStrength`/`uFov` in/out instead of popping; `uStrength=0` is an exact
  identity blit).
- `WorldProjectionSnapDriver` now captures `fromMatrix` at tween start and, each frame, writes
  `projectionMatrix = lerp(fromMatrix, goalMatrix(liveSize), eased)` (+ inverse), skipping the old
  cross-family `camera.zoom` write (fixed a real bug: it used to lerp zoom 1→50 onto the *perspective*
  camera on a persp→ortho switch). At completion it pins the goal matrix and "holds" it (re-pinning every
  frame) until the real destination camera mounts (matched params ⇒ identical matrix ⇒ invisible swap),
  then a cleanup effect keyed on camera identity clears the hold.
- `WorldProjectionRig` gained an optional `pendingSpec` prop so `WorldCurvilinearPass` can pre-mount
  (render target warm) while only the *pending* spec is curvilinear, letting its strength ramp in before
  the remount for perspective-family sources; ortho-family sources can't render curvilinear content until
  the camera is literally a `PerspectiveCamera`, so that specific cross-family case still gets a same-frame
  strength jump at the remount instant — a known, documented residual limitation, but the matrix/pose are
  smooth throughout even there.
- `WorldOrbitViewControls` gained `onPendingSpecChange` reporting the unified pane-or-gizmo pending spec
  up to the host.

## Host wiring (`framework/renderer/react/index.tsx`)

- New `pendingProjectionSpec` state, set via `WorldOrbitViewControls`'s new `onPendingSpecChange`, passed
  to `<WorldProjectionRig pendingSpec={...}>`.

## CAD renderer

No changes — `cad/renderer/js/index.tsx` only uses `WorldOrbitViewControls` (no `WorldProjectionRig`/
curvilinear pass of its own), so it inherits the matrix morph automatically.

## Tests

Extended the in-source vitest block in `infinite/world/r3f/index.tsx`: goal-matrix equality against real
Three.js cameras, oblique/two-point composition, morph endpoints + shear half-ramp at t=0.5, scale-matching
round-trips, and three new viewport-aware `worldProjectionTransitionPose` cases (persp→ortho, ortho→persp,
persp-fov dolly-zoom) alongside the two pre-existing no-viewport cases. `bun nx run
@semio-tech/infinite-world-r3f:test-quick` → 174/174 passed (see `vitest-matrix-morph.txt`).
`@semio-tech/framework-renderer-react:test-quick` → 252/252 passed.

## Runtime verification — blocked this session

Tried `puzzle-3d-react-dev` (port 6013): stuck in a long from-scratch `cargo`/`trunk` release rebuild of a
large shared Rust workspace (dozens of crates), consistent with other concurrent sessions' in-progress Rust
edits (`framework/core/rs/lib.rs`, `ui/wgpu/rs/lib.rs`, `infinite/world/rs/lib.rs` were all mid-edit in git
status at session start) — never finished booting in this session.

Tried Storybook (port 6010) as the documented fallback: its Vite dependency scan fails because
`ui/js/react/index.tsx` (a concurrent session's in-progress, currently-staged edit — `git status` showed it
`M` before this session touched anything) has a live syntax error at the time of writing (`dragState:
ModeDragState | null;` around line 24832, "Expected '}' but found ':'"). `ui/js/react` is the source of
`@semio-tech/ui-react`, which `infinite/world/r3f/index.tsx` imports, so nothing that bundles the World3d
canvas can build until that unrelated edit lands. Not caused by this ticket's changes.

Recommend re-running the manual cycle (pane + gizmo: threePoint↔orthographic↔curvilinear↔oblique↔twoPoint,
mid-tween re-clicks, orbit right after a snap) once either dev server boots cleanly.

## Regression fix — dolly-zoom branches removed

Dev feedback after the first pass: "transitions are resetting the camera point and it flies fore and back."

Root cause: `worldProjectionTransitionPose`'s ortho→persp and persp→persp-fov-change branches (added above)
computed a *new* eye position along the existing view direction to preserve apparent scale (a deliberate
dolly-zoom). But the pre-existing, load-bearing invariant documented on `applyWorldProjectionToCameraState`
is "pure parameter tweaks like angle/depth/fov never move the camera" — before this ticket's changes,
*no* orientation-unchanged projection-kind switch ever moved the camera position, including switches that
change FOV a lot (e.g. threePoint 50°→curvilinear 120°, or any ortho→persp switch). My two new branches
silently broke that invariant, so every click into/out of Curvilinear (the only wired kind with a
different default FOV at the same "free" orientation) now dollied the camera — perceived as "flies fore
and back."

Fix: removed `worldProjectionMatchedPerspectiveDistance` and `worldProjectionFovCompensatedDistance` and
both branches entirely. `worldProjectionTransitionPose` now only special-cases persp→ortho (picks a
scale-matched *zoom* value via `worldProjectionMatchedOrthoZoom` — this never touches `position`, only
which zoom number is chosen); every other mode-only case (including ortho→persp and persp-fov changes)
falls through to the original zoom-only, position-preserving path. Updated/removed the corresponding
vitest cases; `infinite/world/r3f` is 172/172 green, `framework/renderer/react` still 252/252 green.

The projection-*matrix* morph itself (persp↔ortho, shear, curvilinear ramp) is unaffected by this fix —
only the destination-*pose* calculation changed. Camera position for any orientation-unchanged mode
switch is now byte-identical to the pre-ticket behavior; only the matrix/zoom interpolate smoothly.
