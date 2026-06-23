---
name: Fix Brep Geometric Operations
overview: The brep JS kernel adapter was written against a replicad-style API, but the installed brepjs@18 has different signatures and returns Result wrappers. Rework the kernel operation layer to match brepjs@18 (unwrap Results, fix arg order, fix angle units, honor direction/axis vectors), expose missing parameters as neuron input ports, and add real geometric-correctness tests.
todos:
  - id: ticket
    content: Open/reopen ticket via repo mcp and associate with the most appropriate goal from repo://goals
    status: completed
  - id: kernel-unwrap
    content: Add uniform Result unwrap helper and apply to all Result-returning kernel ops
    status: completed
  - id: kernel-booleans
    content: "Fix booleans: cutSync, intersectSync, and 2D booleans to brepjs@18 signatures"
    status: completed
  - id: kernel-solid
    content: "Fix solid ops: extrude (honor direction, remove profile shortcut), revolve, loft, sweep, support/twist extrude, fillet, chamfer, shell, thicken, offset, hull, convexHull, minkowski, draft"
    status: completed
  - id: kernel-transforms
    content: "Fix transforms: rotate (degrees+axis), mirror (normal/at), clone (unwrap), patterns; verify translate/scale"
    status: completed
  - id: kernel-intersections
    content: Fix section/sectionToFace (plane), split (tools array), slice (planes array); fix gears, polygon, polyhedron
    status: completed
  - id: kernel-interface
    content: Update BrepKernelInterface signatures for new axis/plane/center/direction params
    status: completed
  - id: neuron-ports
    content: Extend BREP_FLOW_KINDS and BREP_EVAL_HANDLERS to expose rotate/revolve axis, mirror plane, scale center, draft direction, slice plane, pattern params
    status: completed
  - id: tests-brep
    content: Extend geometry/brep/js tests with geometric-correctness assertions; update centered-footprint extrude test
    status: completed
  - id: tests-procedural
    content: Extend procedural/react tests for cut, non-Z extrude, rotate axis, mirror plane via evaluateBrepFlowKind
    status: completed
  - id: verify
    content: Run @semio-tech/geometry-brep-js:test and @semio-tech/procedural-react:test; ensure launch.json entries; close ticket with summary
    status: completed
isProject: false
---

# Fix Brep Geometric Operations

## Root cause
`[geometry/brep/js/index.ts](geometry/brep/js/index.ts)` calls `brepjs` with the wrong API surface. Against `brepjs@18`:
- Many ops return `Result<T>` (via `isOk`/`unwrap`) but are registered raw, so the shape is a Result wrapper and tessellation produces nothing. This is why `cut` (`cutSync`, line ~1038) "does nothing", while `fuseAllSync`/`cutAllSync` (which unwrap) work.
- Argument signatures differ: `extrude(face, height: number|Vec3)`, `sketchExtrude(sketch, distance, {extrusionDirection})`, `rotate(shape, angleDeg, {axis,at})`, `mirror(shape, {normal,at})`, `section/slice(shape, plane(s))`, `shell/draft(shape, faces, …)`, `rectangularPattern(shape, options)`, `convexHull(points)`, `makeExternalGear(params)`.
- The extrude "profile shortcut" (`profileRectangleSolid`/`profileCircleSolid`, used in `extrudeSync`) always builds a Z-axis solid centered at origin, so non-Z extrude vectors are ignored.
- brepjs rotate/revolve/circularPattern angles are in **degrees**; handlers pass radians.

## 1. Kernel adapter rewrite — `geometry/brep/js/index.ts`
Add one private `unwrap`-style helper (reuse existing `isOk`/`unwrap`) and apply it uniformly. Fix each op to the real signature:
- Booleans: `cutSync`, `intersectSync` -> unwrap `cut`/`intersect`. Keep `fuseAllSync`/`cutAllSync`. `cut2DSync`/`fuse2DSync`/`intersect2DSync` -> verify 2D return types and unwrap/normalize.
- Solid: `extrudeSync` -> remove profile shortcut; for drawings use `sketchExtrude(sketch, distance, { extrusionDirection: direction })`, for faces use `extrude(face, scale(direction, distance))`; unwrap. `revolveSync` (axis+angle via options, unwrap), `loftSync`, `sweepSync`, `supportExtrudeSync`, `twistExtrudeSync`, `filletSync`, `chamferSync` (`shape, undefined, value`), `shellSync`/`draftSync` (faces arg), `thickenSync`, `offsetSync` (solid path), `hullSync`, `convexHullSync` (points), `minkowskiSync` -> correct args + unwrap.
- Transforms: `rotateGeomSync` (`rotate(shape, deg, {axis, at})`), `mirrorGeomSync` (`mirror(shape, {normal, at})`), `cloneGeomSync` (unwrap), `linearPatternSync`/`circularPatternSync`/`rectangularPatternSync` (real signatures + unwrap). `translate`/`scale` already correct.
- Intersections: `sectionSync`/`sectionToFaceSync` (plane input), `splitSync` (tools array, unwrap, then split parts), `sliceSync` (planes array, unwrap).
- Gears: `makeExternalGearSync`/`makeInternalGearSync` (params object, unwrap `GearResult`, register its solid).
- `polygonSync`/`polyhedronSync` -> unwrap.
- Angle units: keep kernel sync methods in **radians** (existing convention) and convert to degrees inside the adapter right before brepjs rotate/revolve/circularPattern calls; localize the quirk.
- Update the `BrepKernelInterface` method signatures (region `🔌BrepKernelInterface`) to accept the new params (axis/plane normal+origin/center/direction/count/spacing) where added.

## 2. Neuron kinds + handlers — `procedural/react/index.tsx`
Expose the now-parameterizable inputs (chosen scope) by extending `BREP_FLOW_KINDS` (lines ~144-251) and the matching entries in `BREP_EVAL_HANDLERS` (lines ~413-870):
- `xform.rotate`: add `axis` (vector) input; `xform.mirror`: add plane `origin`+`normal`; `xform.scale`: add `center`; `solid.revolve`: add `axis`; `solid.draft`: add `direction`; `intersect.slice`: add plane `origin`+`normal`; patterns: add `direction`/`axis`/`spacing` where missing.
- Handlers read these via `parseVec3Input`/`parseNumber` with sensible fallbacks (current hardcoded values become defaults), then call the updated kernel methods.
- Keep handler angle inputs in radians (kernel converts).

## 3. Tests (extend existing files only)
- `geometry/brep/js/index.ts` test region: add geometric-correctness assertions (volume/area/bounds), not just "renderable":
  - extrude honors non-Z vector (bounds shift along the vector); update the existing centered-footprint test (`sketch rectangle and extrude share centered footprint`) to expect plane-anchored z.
  - `cut`/`intersect` reduce/define volume; `section`/`slice`/`split`; `revolve` volume; `loft`/`sweep`; `fillet`/`chamfer`/`shell`/`thicken`/`offset`; `hull`/`convexHull`/`minkowski`; `rotate`/`mirror`/`scale` via bounds; patterns; gears.
- `procedural/react/index.tsx` test region: `evaluateBrepFlowKind` end-to-end for `brep.bool.cut` (volume < base), `brep.solid.extrude` non-Z vector, `brep.xform.rotate` with `axis`, `brep.xform.mirror` with plane.

## 4. Repo workflow + verification
- Work inside a ticket (reopen the related `2026/06/08/PROCEDURAL-PREVIEW-RECTANGLE-EXTRUDE` if it covers this, else open a new ticket); associate with the best goal from `repo://goals`.
- Use regions/subregions; concise code; emoji docstrings; no in-definition comments; `[DEBUG]` prefix for any temporary logs.
- Verify by running the suites (do not claim passing without running): `@semio-tech/geometry-brep-js:test` and `@semio-tech/procedural-react:test` via `nx`; register/confirm any commands in `launch.json` following existing grouping.