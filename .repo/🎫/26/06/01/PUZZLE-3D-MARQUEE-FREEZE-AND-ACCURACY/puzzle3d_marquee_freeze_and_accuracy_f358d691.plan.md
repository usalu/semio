---
name: puzzle3d marquee freeze and accuracy
overview: Fix the puzzle3d group-selection 2s first-gesture freeze by moving candidate building off the gesture hot path with a persistent, reliably-warmed footprint cache, and fix the inaccurate selection by hit-testing against each object's tight projected convex hull instead of an inflated screen-space AABB. All changes are in puzzle/3d/react/index.tsx inside existing regions; tests extend existing describe blocks.
todos:
  - id: validate
    content: Add [DEBUG] timing in capture/buildMarqueeCandidates/projectObjectGroupToScreenPoints; run puzzle3d play scene and confirm the first-gesture cost and post-fix removal
    status: completed
  - id: hull-footprint
    content: Extend MarqueeCandidate with hull; make projectObjectGroupToScreenPoints return convex hull + screenBounds; update buildMarqueeCandidates for object/vortex/attraction
    status: completed
  - id: hull-hittest
    content: Rewrite marqueeCandidateSelected to hull-based window/crossing for rectangle and lasso (with screenBounds fast-reject); add screenPolygonsIntersect helper
    status: completed
  - id: footprint-cache
    content: Add persistent footprint cache, expand warmObjectGroupMarqueeBounds to record meshes+local corners, warm reliably after mount, invalidate on unregister/pose change
    status: completed
  - id: prefetch
    content: Project cached corners in buildMarqueeCandidates (no computeBoundingBox); prefetch candidates on pointerdown rAF; activation uses prefetch with cheap fallback; cancel in-flight on release
    status: completed
  - id: tests
    content: Extend existing marquee describe blocks for hull tightness, window/crossing correctness, lasso, and warm/cache behavior; run tests and confirm pass
    status: completed
  - id: ticket
    content: Open/reopen repo MCP ticket, keep temp files in ticket folder, validate in play scene, remove [DEBUG] logs, close ticket with summary and touched files
    status: completed
isProject: false
---

# Fix Puzzle3d Group-Selection Freeze and Accuracy

All edits in [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx), inside existing regions/subregions (no new files). Work inside a repo MCP ticket.

## Root causes (confirmed by reading code)
- 2s freeze: on the first marquee, `captureMarqueeCandidates()` runs synchronously inside `pointermove` activation (line 6269) -> `buildMarqueeCandidates()` (6521) -> `projectObjectGroupToScreenPoints()` (5320) traverses every mesh of every object and lazily calls `geometry.computeBoundingBox()`. The "Marquee Hit Testing" ticket removed the prefetch/footprint cache, so this cold, full-scene cost is back on the gesture path. The idle `warmObjectGroupMarqueeBounds` (5306, scheduled at `registerObject` 6747) races GLB loading and is not reliably done before the first gesture.
- "Often wrong": each candidate's footprint is one screen-space AABB (`screenBounds`, union of all projected mesh-box corners). That rect is far larger than the visible silhouette -> window misses enclosed objects (false negatives), crossing grabs undrawn objects (false positives).

## Step 0 - Validate at runtime (repo rule)
- Add `[DEBUG]` `performance.now()` timing around `captureMarqueeCandidates`/`buildMarqueeCandidates` and a counter inside `projectObjectGroupToScreenPoints` (cold `computeBoundingBox` count). Run the puzzle3d play scene, start a marquee, read console to confirm the dominant first-gesture cost and that it disappears after the cache lands. Remove `[DEBUG]` logs before closing.

## Step 1 - Tight convex-hull footprint (accuracy)
- Extend `MarqueeCandidate` (5088) to carry `hull: readonly ScreenPoint[]` (convex silhouette) plus the existing `screenBounds` (kept only as a cheap reject box).
- Rewrite `projectObjectGroupToScreenPoints` (5320) to return `{ hull, screenBounds }`: project all in-front mesh-box corners (existing `writeMarqueeBoxCorners` + `projectWorldToClientMarquee`), then reduce to `convexHullScreenPoints(...)` (already exists, 5112). `screenBounds` = `screenBoundsFromClientPoints(projected)`.
- Update `buildMarqueeCandidates` (6521): object candidates use the hull; vortex (single point) and attraction (two points) set `hull` to their projected point(s).
- Rewrite `marqueeCandidateSelected` (5185) to use the hull (with `screenBounds` as a fast AABB pre-reject):
  - Window/rectangle: every hull point inside `rect` (tight enclosure).
  - Crossing/rectangle: reuse `screenRectIntersectsPolygon(rect, hull)` (5165) - point-in-rect, rect-corner-in-hull, or hull-edge crossing.
  - Lasso window: every hull point in the lasso polygon (`pointInPolygon`, 5133).
  - Lasso crossing: polygon-vs-hull intersect (any hull point in polygon, any polygon point in hull, or any edge pair crosses via `segmentIntersectsSegment`, 5146) - add a small `screenPolygonsIntersect(a, b)` helper next to the existing intersect helpers.
  - Generalize for 1-2 point hulls (vortex/attraction) by iterating points for containment and edges for crossing.

## Step 2 - Kill the freeze (perf, off the hot path)
- Add a persistent footprint cache so capture never does `computeBoundingBox` or geometry traversal:
  - Maintain `objectFootprintCacheRef: Map<id, { meshes: Mesh[]; localCorners: Vector3[][] }>` (or world corners) populated by an expanded `warmObjectGroupMarqueeBounds` that also records each mesh and its 8 local box corners.
  - Make warming reliable: in addition to the idle `scheduleDeferredCallback` at `registerObject` (6742-6748), warm in a frame/effect after objects mount (drive off the existing R3F env/`invalidate`) so all boxes are computed before the first gesture; invalidate the cache for an object on unregister and on relocate/pose change.
- `buildMarqueeCandidates` projects cached corners (transform local corners by current `mesh.matrixWorld`, project, hull) - cheap matrix math only, no `computeBoundingBox`, no cold traversal.
- Reinstate prefetch: on `pointerdown` (MarqueeBridge `onPointerDown`, 6215) schedule a rAF that runs `captureMarqueeCandidates()` ahead of the drag; activation (6268) uses the prefetched candidates, falling back to a (now-cheap) synchronous build. Cancel any in-flight prefetch in `cancelGesture`/`onPointerUp` so a quick click stays correct.

## Step 3 - Tests (extend existing describe blocks ~8602-8650; no new files)
- `projectObjectGroupToScreenPoints`: returns a tight hull (more than a 4-corner AABB; tighter than the union rect for a rotated/multi-mesh group).
- `marqueeSelectionFromCandidates`/`marqueeCandidateSelected`: hull window selects a fully-enclosed object the old AABB missed; crossing selects on hull-edge crossing; an object whose AABB overlaps but whose hull does not is rejected; lasso window/crossing via hull.
- Warm/cache: `warmObjectGroupMarqueeBounds` populates the footprint cache; capture after warm performs no `computeBoundingBox`.
- Run the package test command and confirm all pass.

## Step 4 - Ticket workflow
- Reopen the existing `PUZZLE-3D-MARQUEE-PREVIEW-PERF`/`FIX-PUZZLE3D-MARQUEE-HIT-TESTING` ticket via repo MCP (or open a new one if none matches), keep temp logs inside the ticket folder, validate in the play scene, then close with a summary and touched files.