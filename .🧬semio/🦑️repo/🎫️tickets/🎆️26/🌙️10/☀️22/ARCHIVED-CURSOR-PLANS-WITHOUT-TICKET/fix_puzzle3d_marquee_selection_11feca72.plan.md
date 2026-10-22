---
name: Fix puzzle3d marquee selection
overview: Fix puzzle3d rectangle/lasso marquee selection false positives and false negatives by making screen-space projection near-plane safe and tightening object footprints to the visible silhouette instead of the inflated world AABB.
todos:
 - id: ticket
   content: Open repo ticket for puzzle3d marquee selection fix via MCP ticket_open (associate with appropriate goal from repo://goals)
   status: completed
 - id: projection
   content: Rewrite projectObjectGroupToScreenPoints to project actual geometry vertices, track anyBehindCamera, and reduce to convex hull (add convexHull helper)
   status: completed
 - id: candidate
   content: Extend MarqueeCandidate with anyBehindCamera and update buildMarqueeCandidates / vortex / attraction candidate construction
   status: completed
 - id: hittest
   content: "Fix marqueeCandidateSelected: window mode rejects anyBehindCamera and uses tight hull containment; crossing uses convex-hull edges"
   status: completed
 - id: tests
   content: Extend existing marquee describe blocks to cover behind-camera exclusion, tight rotated-object enclosure, and hull-edge crossing; run tests and confirm pass
   status: completed
 - id: close
   content: Verify runtime behaviour, then close ticket with summary and touched files
   status: completed
isProject: false
---

## Root causes (all in `puzzle/3d/react/index.tsx`)

- False positives: `projectWorldToClient` (line 5213) returns `null` for corners behind the near plane, and `projectObjectGroupToScreenPoints` (line 5228) silently drops them, leaving a collapsed/mislocated footprint rect for objects straddling or behind the camera. That bogus rect can fall inside the marquee.
- False negatives: footprint is the bounding rect of the projected corners of the inflated world AABB (`Box3.setFromObject(group, false)`), much larger than the visible silhouette, so window mode (`screenRectContainsRect`, line 5145) rejects pieces that look fully enclosed.
- Crossing mode treats the unordered 8 AABB corners as an ordered polygon (lines 5124/5131), testing arbitrary diagonals.

## Fix

### 1. Near-plane-safe, tight footprint

Rewrite `projectObjectGroupToScreenPoints` to build a footprint from the object's actual geometry (traverse meshes, apply `matrixWorld`, sample/cap vertices; fall back to AABB corners if no geometry), and return a structured result `{ hull: ScreenPoint[]; anyBehindCamera: boolean }`:

- Project each world point; track `anyBehindCamera` when a point fails the near/far guard.
- Reduce projected points to their convex hull (add a small monotone-chain `convexHull(points)` helper) so crossing-edge tests use real boundary edges.

### 2. Carry footprint metadata on candidates

Extend `MarqueeCandidate` (line 5083) to include `anyBehindCamera` (vortex/attraction single-point candidates set it `false`). Update `buildMarqueeCandidates` (around line 6425) accordingly.

### 3. Correct hit-testing in `marqueeCandidateSelected` (line 5105)

- Window mode: if `anyBehindCamera`, return `false` (object not fully visible, so never window-select). Otherwise require all hull points inside the marquee (tight silhouette fixes false negatives).
- Crossing mode: any hull point inside marquee, OR convex-hull edge intersects the marquee rect (`points[i] -> points[(i+1)%n]`), fixing the arbitrary-diagonal bug.

### 4. Tests (same file, existing `describe` blocks ~lines 8356-8469)

Extend the existing marquee tests (do not add new files): behind-camera object excluded from window selection, a tight footprint selects a fully-enclosed rotated object that the old AABB rect missed, and crossing uses hull edges. Run via the package's existing test command and confirm pass.

### 5. Ticket

Open a repo ticket (MCP `ticket_open`) for the fix, keep any temp logs inside the ticket folder, and close it (`ticket_close`) with the summary and touched files.

## Out of scope

Single-click raycast picking (lines ~4393-4916) is unaffected and untouched.
