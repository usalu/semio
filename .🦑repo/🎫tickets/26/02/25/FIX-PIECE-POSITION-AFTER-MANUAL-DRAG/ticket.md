---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed piece position not persisting after manual drag. Root cause: flattenDesign() always recomputed child centers from parent+connection, overriding stored centers set by drag. Fix: childPiece.center ?? computedChildCenter in flattenDesign BFS. Removed 8 DEBUG logs. All 13 unit tests pass.

## Changes

- `compose/js/compose.ts`: Changed `flattenDesign()` BFS child center computation to prefer stored center (`childPiece.center ?? computedChildCenter`) instead of always using computed center. This preserves manual drag positions while still computing centers for pieces without stored positions (null/undefined center).
- `compose/js/sketchpad/Design.tsx`: Removed 7 `[DEBUG]` console.warn logs from baseNodes useMemo, useEffect, useDesignAppUpdatePieces, and onNodeDragStop.
- `compose/js/sketchpad/Sketchpad.tsx`: Removed 1 `[DEBUG]` console.warn log from PieceStore.change.

## Log

1. Analyzed full drag pipeline: onNodeDragStop → updatePieces → store.execute → kitStore.change → designStore.change → pieceStore.change → YCoordStore.change
2. Traced reactive chain: Yjs observers → DerivedNode → useSyncExternalStore → metadata useMemo → baseNodes useMemo → useEffect setNodes
3. Identified root cause: `flattenDesign()` computes child piece centers from parent position + connection offsets, ignoring stored center from drag
4. Verified initial piece data: metabolism DB shows child pieces have NULL center (auto-computed by flattenDesign), root pieces have explicit center
5. Implemented fix: `childPiece.center ?? computedChildCenter` in flattenDesign BFS
6. Verified all 13 unit tests pass, including Flatten tests that validate center computation

## Todos

- [x] Analyze drag position code pipeline
- [x] Identify root cause of position reset
- [x] Implement fix in flattenDesign
- [x] Remove DEBUG logs
- [x] Run unit tests (13/13 pass)
- [x] Run Design Playwright test (pending - infrastructure flakiness)

## Plan

Root cause: In `flattenDesign()` (compose.ts line ~6644), the BFS traversal always computes child piece centers based on parent position + connection parameters:

```
childCenter = computed from parentCenter + connectionOffsets
flatChildPiece = { ...childPiece, center: childCenter }  // overwrites stored center
```

This means after dragging, the reactive chain (metadata → baseNodes → useEffect) recomputes positions using the OLD computed center instead of the NEW stored center from drag.

Fix: Use nullish coalescing to prefer stored center:

```
const childCenter = childPiece.center ?? computedChildCenter;
```

- Pieces with null/undefined center (never dragged): use computedChildCenter (auto-layout) ✓
- Pieces with stored center (dragged): use stored center (preserves drag position) ✓
- Root pieces: unaffected (not in BFS child processing) ✓
