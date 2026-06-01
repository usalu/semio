---
name: marquee startup perf
overview: Eliminate the ~2s stall on the first puzzle-3d group-selection gesture by removing the one-time cold cost of computing geometry bounding boxes inside the synchronous marquee-candidate capture.
todos:
  - id: validate
    content: Add [DEBUG] timing around captureMarqueeCandidates/buildMarqueeCandidates/projectObjectGroupToScreenPoints and confirm setFromObject cold cost at runtime
    status: pending
  - id: warm-bbox
    content: In registerObject, compute geometry.boundingBox for group meshes (deferred/idle) so first marquee capture is warm
    status: pending
  - id: overlay-first
    content: Reorder activation to paint overlay rectangle before capture; defer capture+preview to next frame and cancel on early release
    status: pending
  - id: cleanup-test
    content: Remove [DEBUG] logs, extend existing marquee tests for warm-up, and verify in the play scene that the startup delay is gone
    status: pending
  - id: ticket
    content: Reopen/open repo MCP ticket and close it with summary of touched files
    status: pending
isProject: false
---

# Fix Puzzle 3D Group-Selection Startup Stall

## Root cause (to confirm, then fix)
On the first marquee gesture, `captureMarqueeCandidates()` runs synchronously in the `pointermove` handler **before** the overlay rectangle is drawn ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) ~6146). It calls `buildMarqueeCandidates()` (6425) → `projectObjectGroupToScreenPoints()` (5228) → `new Box3().setFromObject(group, false)`. That call lazily runs `geometry.computeBoundingBox()` for every mesh whose box is uncached. Three.js only computes bounding *spheres* during rendering, so the first gesture pays a one-time bounding-box cost across the whole scene (~2s). Subsequent gestures reuse cached boxes, hence "fast after startup".

## Step 1 - Validate (repo rule: confirm cause at runtime)
- Add `[DEBUG]` `performance.now()` timing around `captureMarqueeCandidates` / `buildMarqueeCandidates` (6479/6425) and a per-object accumulator inside `projectObjectGroupToScreenPoints` (5228), plus around the first `previewMarqueeSelection` and `BulkSelectionVisualBridge` layout effect (3809).
- Run the puzzle 3d play scene, start a group selection, read console to confirm `buildMarqueeCandidates` (specifically `setFromObject`) dominates on the first gesture and is cheap afterward.

## Step 2 - Primary fix: warm bounding boxes at object load
- In `registerObject` ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) ~6643), after storing the group, traverse its meshes and ensure `geometry.boundingBox` is computed (`mesh.geometry.computeBoundingBox()` when null). Schedule this off the critical path (e.g. via the existing invalidate/idle mechanism or a microtask/`requestIdleCallback`-style deferral already used in the file) so model loading is not blocked.
- This makes `setFromObject` warm before any selection, removing the cold cost entirely. Add the new logic inside the existing region structure (regions/subregions) per repo rules; do not create new files.

## Step 3 - Defensive: paint overlay before capture
- Reorder the activation branch (6144-6164) so `puzzle3dMarqueeOverlayStore.setSnapshot(...)` (the rectangle) is set first, and defer `captureMarqueeCandidates()` + first `previewMarqueeSelection()` to the next animation frame. This guarantees the rectangle appears immediately even if any future cold cost reappears, with selection highlighting following one frame later. Ensure an in-flight deferred capture is cancelled in `cancelGesture`/`onPointerUp` so a quick click-drag-release stays correct (fallback `buildMarqueeCandidates()` already exists in `commitMarqueeSelection` at 6512).

## Step 4 - Cleanup & verify
- Remove all `[DEBUG]` logs.
- Extend the existing marquee tests in [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) (~8348-8470) to cover the warm-up (bounding boxes computed on register) rather than adding new test files.
- Re-run the play scene and confirm the first gesture now previews the rectangle and selects without the multi-second delay.

## Ticket workflow
- Reopen the existing `PUZZLE-3D-MARQUEE-PREVIEW-PERF` ticket (or open a new one if none matches) via repo MCP, keep temp logs/scripts inside the ticket folder, and close with a summary listing touched files.