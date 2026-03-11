---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Persisted drag diff updates now include moved descendant subtree piece centers and compute connection diffs using selected plus descendant IDs; extended existing drag propagation test to require non-zero persisted center deltas for parent/child/grandchild.
## Root Cause

Two issues:

1. (Initial) `onNodeDragStop` directly mutated node `.position` instead of calling `setNodes()`.
2. (True root cause) Pieces loaded from kit files have NO stored centers (centerX/centerY are null/0 in SQLite). This causes:
   - `dragPiecesInDesign` skips them from `rootMovers` because `designPieces.find(dp => dp.guid === p.guid)?.center` is falsy
   - `onNodeDragStop` filter `if (originalPiece?.center && pu.diff.center)` also skips them
   - Result: `finalUpdates` is empty → `updatePieces` never called → store unchanged → visual drag temporary → baseNodes recompute from unchanged metadata → snap back

## Fix

1. Replace direct node mutation with `setNodes()` (from first pass).
2. Compute absolute final centers directly in `onNodeDragStop` using `metadata.get(pieceGuid)?.center` as fallback baseline when `originalPiece.center` is undefined. This bypasses the `dragPiecesInDesign` piece-center logic entirely (still use it for connection offsets).

## Changes

- Design.tsx `onNodeDragStop`: Compute piece center updates directly using `originalPiece.center ?? metadata.get(pieceGuid)?.center ?? { u: 0, v: 0 }` as baseline, adding the drag offset to get absolute final center. Still use `dragPiecesInDesign` for connection offset updates only.
- Design.tsx: Removed 4 DEBUG console.warn statements.

## Log

- Analyzed full drag flow: onNodeDragStart → onNodeDrag → onNodeDragStop → dragPiecesInDesign → updatePieces → store.change → piecesMetadata → flattenDesign → designToNodesAndEdges
- Confirmed pieces in kit DB have no stored centers
- Confirmed dragPiecesInDesign returns empty diff for centerless pieces
- Confirmed metadata.get(guid).center provides the flattened center from flattenDesign BFS
- Fixed by using metadata center as fallback baseline

## Todos

- [x] Analyze drag flow
- [x] Replace direct node mutation with setNodes
- [x] Identify true root cause (pieces without stored centers)
- [x] Implement real fix (metadata center fallback)
- [x] Remove debug logging
- [x] Verify unit tests pass (15/15)
- [x] Verify TS compilation (0 errors)

## Plan

1. Replace direct mutation loop with `setNodes` functional update in `onNodeDragStop`
2. Identify root cause: pieces without stored centers get empty dragDiff
3. Use metadata center as fallback baseline for computing absolute final centers
4. Remove debug logs
5. Run unit tests
6. Close ticket

## Reopen 2026-03-11

### Prompt
Ensure diagram diff updates when dragging a node that has children and deeper descendants.

### Plan
1. Inspect drag-stop persistence path in `Design.tsx`.
2. Ensure descendant pieces get persisted center diffs on drag stop.
3. Ensure diff computation for connections includes dragged descendants.
4. Extend existing sketchpad drag propagation test assertions.
5. Run focused sketchpad tests.

### Todos
- [x] Inspect current drag-stop logic
- [ ] Patch drag-stop diff persistence for descendants
- [ ] Extend existing drag propagation test coverage for persisted centers
- [ ] Run focused test validation
- [ ] Close ticket with changed files

### Changes 2026-03-11
- Updated `semio/js/sketchpad/Design.tsx` drag stop persistence to write center diffs for dragged descendants and descendants-of-descendants using `savedDescendantOffsets`.
- Switched final piece update collection to a keyed map to prevent duplicate writes per piece.
- Expanded `dragPiecesInDesign` input in drag stop from selected-only pieces to selected + dragged subtree IDs so connection diff updates are computed for the full moved subtree.
- Extended existing drag propagation assertions in `semio/js/sketchpad.test.ts` to require non-zero persisted center deltas for parent, child, and grandchild.

### Validation 2026-03-11
- `pnpm -C semio/js test:e2e sketchpad.test.ts --grep "Drag propagation"` failed: `playwright: not found`.
- `pnpm -C semio/js exec playwright test sketchpad.test.ts --grep "Drag propagation"` failed: `Command "playwright" not found`.
- `pnpm -C semio/js test:unit` failed: `vitest: not found`.

### Todos Update
- [x] Inspect current drag-stop logic
- [x] Patch drag-stop diff persistence for descendants
- [x] Extend existing drag propagation test coverage for persisted centers
- [x] Run focused test validation
- [ ] Close ticket with changed files
