---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fix UV position snap-back after drag in Design app diagram.

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
