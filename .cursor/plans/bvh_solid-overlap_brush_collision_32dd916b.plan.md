---
name: BVH solid-overlap brush collision
overview: Replace the AABB-based brush/fill collision (which is fundamentally inadequate for the ~13%-fill sparse lattice meshes) with exact three-mesh-bvh geometry collision behind an interface, and change the brush tolerance to a solid-overlap volume budget in cubic meters.
todos:
 - id: dep
   content: Add three-mesh-bvh to @semio-tech/infinite-world-r3f (and resolve for @semio-tech/puzzle-3d-react) via bun
   status: completed
 - id: interface
   content: "Add Collision region in infinite/world/r3f/index.tsx wrapping MeshBVH: collisionBodyFromObject, bodiesIntersect, solidOverlapVolume"
   status: completed
 - id: cache
   content: Replace brushCollisionGltfScenes with CollisionBody cache keyed by meshUrl in puzzle/3d/react
   status: completed
 - id: probes
   content: Rewrite brushPreviewCollides, fillPreviewCollidesAccumulated, brushCandidateCollidesAtPose, ghost and commit to use solidOverlapVolume > budget; include host
   status: completed
 - id: budget
   content: Repurpose tolerance constants/props to solid-overlap volume budget (m3) in react + play UI
   status: completed
 - id: tests
   content: Update react/play unit tests to volume semantics and add sparse-lattice regression; add ticket verification script and run test targets
   status: completed
isProject: false
---

## Root cause (confirmed by measurement)

The concrete-forest meshes are sparse lattices that fill only ~12.7% of their AABB (solid ~16.8 m3 inside a 132.5 m3 box). Vortex-aligned pieces have AABBs that only touch (min-axis overlap 0.00), so `boxesPenetrationExceeds` reports "free", while the actual branches interpenetrate. A real fill at tol 0.1 produced 16 pairs with >0.5 m3 true mesh overlap, worst 17.69 m3 (~a whole duplicate piece). No tolerance value fixes AABB.

## 1. Add three-mesh-bvh behind an interface in the engine layer

Per repo rules (external libs behind an interface), isolate the dependency in `@semio-tech/infinite-world-r3f`.

- `bun add three-mesh-bvh` in [infinite/world/r3f/package.json](infinite/world/r3f/package.json) (latest, three 0.182-compatible). Mirror the dep where `@semio-tech/puzzle-3d-react` resolves it.
- In [infinite/world/r3f/index.tsx](infinite/world/r3f/index.tsx) add a `//#region Collision` exposing a small interface that fully encapsulates `MeshBVH`/`computeBoundsTree`:
  - `collisionBodyFromObject(root: Object3D): CollisionBody` - traverse meshes, build a `MeshBVH` per geometry in the GLB mesh frame (reuse `GLB_MESH_FRAME_ROTATION_X`).
  - `bodiesIntersect(a, worldMatrixA, b, worldMatrixB): boolean` - BVH `intersectsGeometry` fast path.
  - `solidOverlapVolume(a, worldMatrixA, b, worldMatrixB, opts?): number` - quick AABB-intersection reject; if `bodiesIntersect`, estimate intersection volume by sampling points in the AABB-intersection box and testing inside-ness of both bodies via `MeshBVH.closestPointToPoint` + face-normal sign (standard inside test); volume = fraction x box volume. Adaptive sample count.

## 2. Swap the puzzle 3d collision core to solid overlap

In [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx):

- Replace the `brushCollisionGltfScenes` map with a cache of `CollisionBody` keyed by meshUrl, built lazily from `brushCollisionGltfRoot` / the same registration points: `usePooledGltf` (~4364), `BrushCatalogMeshPreloadEntry` (~7015). Keep `brushCollisionMeshExtentOk` guard.
- Rewrite the three probe paths to use `solidOverlapVolume(...) > budget` instead of `boxesPenetrationExceeds` + `brushPreviewCollisionBox` insets:
  - `brushPreviewCollides` (~3553)
  - `fillPreviewCollidesAccumulated` (~3922) and `fixtureObjectCollisionBox` (store `CollisionBody`+worldMatrix instead of `Box3`)
  - `brushCandidateCollidesAtPose` (~3653) and `BrushPreviewGhost` (~7140) / `commitCurrentPreview` (~7423)
- Stop excluding the host: correct assembly has ~0 solid overlap, so include all scene objects. Simplify/retire `brushPlacementCollisionExcludeObjectIds` usage (empty set), and drop `brushSceneObjectCollisionBox` AABB inset logic.
- Keep `boxesPenetrationExceeds`/AABB helpers only if still referenced elsewhere; otherwise remove with their tests.

## 3. Tolerance becomes a solid-overlap volume budget (m3)

- In [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx): repurpose `DEFAULT_BRUSH_PLACEMENT_COLLISION_TOLERANCE` -> `DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET` (default `0.02` m3 for seam tolerance), max `1` m3, step `0.01`. Rename the prop/threshold consistently (`brushPlacementOverlapBudget`).
- In [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts): slider drives the budget directly in m3 (min 0, max 1, step 0.01), label `Overlap budget (m3)`, value via `formatNumber`; settings input via `formatNumber`. Drop the 0-100 slider index mapping.
- Fill session already rebuilds on tolerance change in [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) (~1916) - just follow the rename.

## 4. Tests and verification

- Update [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) tests: replace AABB penetration tests with `solidOverlapVolume` cases on simple meshes with known overlap (e.g. two unit cubes overlapping 0.5 m3), a sparse-lattice regression (two synthetic meshes: deep overlap detected, clean interleave free), and budget-threshold behavior. Update fill/preview tests.
- Update [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) tolerance tests to the m3 budget.
- Add a verification script under `.repo/🎫/26/06/05/FIX-BRUSH-FILL-COLLISION/` re-running the real-mesh-overlap audit after the fix (expect 0 pairs over budget). Run the existing `@semio-tech/puzzle-3d-react` and `@semio-tech/puzzle-3d-play` test targets.

## Defaults to confirm during implementation

Budget default 0.02 m3, slider 0-1 m3 (step 0.01); BVH sample budget tuned for fill performance (thousands of probes) with intersect fast-path so sampling only runs on actual contacts.
