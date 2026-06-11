---
name: Concrete Puzzle Suggestion Fix
overview: Pin the puzzle/3d concrete-forest brush suggestions for connector b-p1-t-t1-c3-l to exactly the 13 expected beam connectors via a real-geometry vitest, then fix the collision/compat path until that exact set is produced (with WASM/BVH parity).
todos:
  - id: test
    content: Add real-GLB vitest in puzzle/3d/react/index.tsx asserting the collision-free candidate set for seed-left-001:v0 maps to exactly the 13 kit connector names (import kit type JSONs for name mapping).
    status: completed
  - id: diagnose
    content: Run the new test with [DEBUG] logs (under the ticket folder) to capture the current free set and per-candidate overlap volumes; identify why it is not the 13.
    status: completed
  - id: fix-ts
    content: Fix the collision/budget path in react/index.tsx (drive overlapBudget from fixture meta) so exactly the self-overlap is rejected and the 13 beams remain.
    status: completed
  - id: fix-rs
    content: Mirror the change in rs/lib.rs, rebuild wasm pkg, and extend the WASM/BVH parity test to compare exact mapped sets.
    status: completed
  - id: verify
    content: Run full puzzle/3d react+play vitest suites, fix any fallout, remove [DEBUG] logs, and update/close the FIX-BRUSH-FILL-COLLISION ticket.
    status: completed
isProject: false
---

# Concrete Puzzle Suggestion Fix

## Goal
For target vortex `seed-left-001:v0` (connector `b-p1-t-t1-c3-l`, port `b-l`) in `puzzle/3d/fixture/concrete-forest.3d.json`, the compatible + collision-free brush suggestions must be EXACTLY these 13 connectors:

```
b-p1-t-t1-c3-r, b-p1-t-t2-c3-l, b-p1-b-t1-c2-l, b-p1-b-t1-c1-r, b-p1-b-t1-c1-l, b-p1-t-t2-c1-l,
b-p2-t-t1-c3-l, b-p2-t-t1-c3-r, b-p2-t-t2-c3-l, b-p2-b-t1-c1-l, b-p2-b-t1-c2-l, b-p2-b-t1-c1-r, b-p2-t-t1-c1-l
```
i.e. every beam connector of both pieces except the self-connector, and no `c-*` columns.

## Pipeline (confirmed)
- Compatibility: `brushCompatibleCandidates` -> `vorticesAttractionCompatibleForDrag` using `meta.kindCompatibility` in [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) (~3229, ~3014). All `b-*` ports are mutually compatible; `c-*` are not compatible with `b-*`, so columns are already excluded.
- Collision: `brushCollisionFreeCandidates` -> `brushCandidateCollidesAtPose` -> `brushPreviewCollides` -> `solidOverlapVolume` vs `overlapBudget` (~3636, ~3467). Missing BVH => `unknownPending` (candidate skipped, not free).
- Candidate -> connector name: catalog vortex index is 1:1 with kit type connectors (`semio/fixture/kit/dev/abbau-aufbau/wip/initialKit/type/hexagonal-cut-concrete-forest-left/right.type.semio.json`), verified by position.

## Step 1 - Add the exact-set test (real geometry)
Add a vitest in the `Puzzle3dPrecompute`/brush block of [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) (next to the existing concrete-forest test at ~12336, which only checks `free.length > 0`). The new test must:
- Load the REAL GLBs via `GLTFLoader` from `semio/fixture/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-left.glb` and `-right.glb` (same loader pattern as `.repo/.../FIX-BRUSH-FILL-COLLISION/verify-fix.mts`), and `registerBrushCollisionGltfScene` them. Do NOT use the `BoxGeometry(13,5,3)` stub - the box over-collides and cannot yield the 13.
- Build `target`/world pose for `seed-left-001:v0`, call `brushCompatibleCandidates` then `brushCollisionFreeCandidates` with `meshRootForUrl: brushCollisionGltfRoot` and the fixture's overlap budget.
- Import the two kit type JSONs, read `connectors.items[].name` in order, and map each free `(objectKindId, sourceVortexIndex)` to its connector name (Left -> `b-p1-*`, Right -> `b-p2-*`). Assert position alignment between catalog vortex and connector to guard the index mapping.
- Assert `unknownPending === false` and the mapped free set equals exactly the 13 names above (set equality, order-independent).

## Step 2 - Run and diagnose
Run the test (agent mode) to capture the actual current free set and the per-candidate overlap volumes for `seed-left-001:v0`. Add temporary `[DEBUG]` logs under the FIX-BRUSH-FILL-COLLISION ticket folder. Determine which of these is happening:
- self (Left idx0) wrongly kept (=> collision not catching full overlap / budget too high),
- some of the 13 wrongly dropped (=> budget too low or pose wrong),
- extra candidates kept (=> compat/columns leaking).

## Step 3 - Fix to exactly 13
Most likely root cause: overlap budget. The self-coincident placement is a 100% overlap and must always exceed budget; the 13 outward placements must stay under it. Fix at root in [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx):
- Set the concrete-forest overlap budget so exactly the self is rejected (candidate route in `brushCollisionFreeCandidates` / `brushPreviewCollides`, ~3636/3467). Prefer driving it from `meta.overlapBudget` in the fixture rather than a magic number, adding `overlapBudget` to `concrete-forest.3d.json` if needed and threading it through the play snapshot `brushPlacementOverlapBudget`.
- If budget alone cannot separate self from the 13 (e.g. a neighbor also fully overlaps), add a precise root-cause fix (e.g. correct mating pose or self/duplicate-occupancy exclusion), not a heuristic.

## Step 4 - WASM/BVH parity
The Rust path in [puzzle/3d/rs/lib.rs](puzzle/3d/rs/lib.rs) mirrors the TS collision/compat (`brush_compatible_candidates`, `solid_overlap_volume`) and the test at ~12398 asserts `wasm.free.length === bvh.free.length`. Apply the same budget/logic change in `lib.rs`, rebuild the wasm pkg, and keep parity green. Extend the parity assertion to compare the exact mapped sets, not just lengths.

## Step 5 - Verify and finalize
- Run the full `puzzle/3d/react` and `puzzle/3d/play` vitest suites; fix any fallout in the existing concrete-forest tests (3209/3250/3291 in play, 12336 in react) caused by the budget change.
- Remove `[DEBUG]` logs; keep diagnostics under the ticket folder per repo rules.
- Reopen/extend the existing repo MCP ticket `26/06/05/FIX-BRUSH-FILL-COLLISION` (read `repo://goals` first) and close with a summary of files touched.

## Notes / open risk
- The exact "13" depends on real GLB geometry + budget. If running reveals the intended set genuinely cannot be 13 beams-minus-self under any single budget (geometry causes additional full overlaps), I will surface the actual collision-free set and confirm before forcing a heuristic.
- Scope: puzzle/3d only (concrete-forest). No nakagin or semio-kit changes.