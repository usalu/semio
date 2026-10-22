---
name: Fix 3D Matrix Chain Perf
overview: Make updateWorldMatrixChain O(chain-depth) instead of O(whole-scene) so the per-frame CableBatch vortex-world loop stops re-traversing the entire Nakagin scene graph, eliminating the multi-second interaction freezes in the puzzle 5d paired view.
todos:
 - id: rewrite-chain
   content: Rewrite updateWorldMatrixChain in puzzle/3d/react/index.tsx to compose world matrices only along the ancestor chain (no child recursion).
   status: completed
 - id: verify-profile
   content: Re-profile a pointer drag/pan on the 5d 3d surface and confirm updateMatrixWorld no longer dominates and interactions are sub-second.
   status: completed
 - id: verify-correctness
   content: Confirm cable/attraction endpoints, relocate, and vortex hover/select still render at correct world positions.
   status: completed
 - id: build-check
   content: Build puzzle/3d/play and puzzle/5d/play to confirm no type regressions.
   status: completed
isProject: false
---

## Root cause

`useFrame` in `CableBatch` ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) line ~6417) calls `reg.getVortexWorld()` twice per attraction every frame. Each getter (line ~6166) calls `updateWorldMatrixChain(root.current)` (line ~6168). That helper (lines ~5497-5505) invokes `node.updateMatrixWorld(false)` on each ancestor including the scene root; three.js `updateMatrixWorld` recurses into ALL descendants, so it re-walks the whole scene graph. Cost = O(N_attractions x whole_scene) per frame -> seconds per frame on the Nakagin fixture.

## Fix (primary, surgical)

Rewrite `updateWorldMatrixChain` in [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) to update world matrices only along the ancestor chain, without child recursion:

```ts
export function updateWorldMatrixChain(leaf: Object3D): void {
 const chain: Object3D[] = [];
 for (let cur: Object3D | null = leaf; cur; cur = cur.parent) chain.push(cur);
 for (let i = chain.length - 1; i >= 0; i--) {
  const node = chain[i]!;
  node.updateMatrix();
  if (node.parent) node.matrixWorld.multiplyMatrices(node.parent.matrixWorld, node.matrix);
  else node.matrixWorld.copy(node.matrix);
 }
}
```

This turns each getter from O(scene) into O(chain-depth) and reads `getWorldPosition` correctly afterward. All other callers (e.g. `boundsFromPuzzle3dSelection` line ~5551) benefit too. Poses are authored via position/quaternion/scale (`applyObjectPose`), so `updateMatrix()` composition is correct.

## Verification

- Re-run a CPU profile of a pointer drag/pan on the 5d 3d surface at `localhost:6014`; confirm `updateMatrixWorld`/`multiplyMatrices` no longer dominate and a ~30-move drag completes in well under a second (was 136s).
- Confirm attraction/cable endpoint lines still render in the correct world positions after relocate, and vortex hover/select still works.
- Run the puzzle/3d build (`bun ./📜️script.ts build` in `puzzle/3d/play`) and the 5d build (`puzzle/5d/play`) to confirm no type regressions.

## Optional follow-ups (only if jank remains after primary fix)

- Skip the `CableBatch` `useFrame` body when `props.attractions.length === 0` early (already returns null in render, but the frame loop still iterates) and avoid recomputing identical object-group matrices twice per attraction by caching per-group within a single frame.
- Address the separately documented pointermove full-scene raycast in [.cursor/plans/fix_vortex_pick_perf_e61c29fb.plan.md](.cursor/plans/fix_vortex_pick_perf_e61c29fb.plan.md) (gate picking while a pointer button is held + coalesce to one rAF).

## Notes

- Fix lives in puzzle/3d (root of the bug) even though the symptom appears in the puzzle 5d view; this is the correct root-cause location.
- Restart/refresh the `@semio-tech/puzzle-5d-play` dev server (port 6014) after the edit to clear stale HMR before re-profiling.
