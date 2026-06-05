# Optimize Puzzle 3D Fill Slider Performance

## Problem

The fill slider blocked the UI for minutes because:
1. `buildBrushFillSequence` ran synchronously (up to 1000 greedy placements with mesh clone + AABB probes per candidate).
2. Each slider tick replayed all N placements via `applyBrushFillPlacementsToFixture` → `buildSnapshot` per step.

## Solution

### Part A — O(1) prefix application
- Extended `Puzzle3dFillSessionState` with `appendedObjects` and `appendedAttractions` captured during build.
- `applyPuzzle3dFillCount` now composes `{ base + slice(appended*) }` instead of replaying placements.

### Part B — Collision hot path
- Cached per-`meshUrl` local-space AABB (`brushMeshLocalCollisionBox`); posed world box via matrix transform (no per-probe `clone(true)`).
- Memoized `brushCompatibleCandidates` per `(objectKind, vortexKind)` within each build pass.

### Part C — Chunked build + progress UI
- `createBrushFillSequenceStepper` yields placements in chunks (`PUZZLE_3D_FILL_BUILD_CHUNK_BUDGET = 8`) via `setTimeout(0)`.
- `puzzle3dFillBuildProgressRef` drives slider label `Fill N (building X/1000)` and caps slider max while building.
- Playground host clamps `onFillCount` to computed-so-far during build.

## Validation

- `bun ./script.ts test` in `puzzle/3d/react`: **306 passed**
- Diagnostic `fill-prefix-timing.mts`: compose prefix ~0.015ms vs replay ~0.117ms at n=1

## Files

- `puzzle/3d/react/index.tsx` — AABB cache, stepper, engagement progress label
- `puzzle/3d/play/index.ts` — chunked session prep, O(1) apply
- `framework/product/playground/renderer/react/index.tsx` — progress wiring, clamp on drag
- `.repo/🎫/26/06/06/OPTIMIZE-PUZZLE-3D-FILL-SLIDER-PERFORMANCE/fill-prefix-timing.mts` — timing diagnostic
