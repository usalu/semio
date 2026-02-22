---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Implement directional drag propagation in the Design 2D diagram. When dragging a node, its downstream children (via parentPieceId from metadata) move along. Parents/ancestors never move.

## Analysis

### Old vs Current Implementation

The `.old` files show the prior split architecture:
- `Desing.tsx.old`: Design editor shell with DndContext for drag-and-drop from workbench
- `Design.Diagram.tsx.old`: Separate diagram component with onNodeDrag/onNodeDragStop
- `Design.Model.tsx.old`: Separate 3D model component
- `Design.Details.tsx.old`: Separate details/properties panel

Current implementation merges everything into `Design.tsx` (~9800 lines) with the diagram section at lines 5569-8203.

### Prior Drag Logic (old)

The old `Design.Diagram.tsx.old` had:
1. `onNodeDragStart`: Selection management + `startTransaction()` + `setDragState()`
2. `onNodeDrag`: Helper lines, snap, auto-connection detection. **No downstream propagation**.
3. `onNodeDragStop`: Simple `finalizeTransaction()` + cleanup

The old code had commented-out logic for updating piece centers during drag (lines ~1395-1430) but NO downstream/child propagation.

### Current Drag Logic

Current `Design.tsx` has:
1. `onNodeDragStart` (L7411): Selection + transaction + refs
2. `onNodeDrag` (L7439): Helper lines, snap, auto-connection. Updates `pendingPieceUpdatesRef` for selected nodes only.
3. `onNodeDragStop` (L8010): Persists via `updatePieces()` in one transaction.

**Neither old nor current code implements downstream drag propagation.**

### Domain Direction Semantics

- `metadata.get(pieceGuid).parentPieceId` defines the tree structure
- Children of piece X: all pieces where `parentPieceId === X.guid`
- Connections: `connecting.piece` = source (edge source in diagram), `connected.piece` = target (edge target)
- In `addConnection` command: `connected` = parent, `connecting` = child (line 858-862)

## Changes

- Design.tsx: Add `getDownstreamDescendants()` utility and integrate into `onNodeDragStart` + `onNodeDrag` + `onNodeDragStop`
- Design.tsx: Always include dragged node in pendingPieceUpdatesRef (fix for when node not pre-selected)
- Design.tsx: Expose pieces metadata to window for test accessibility via useEffect
- sketchpad.test.ts: Add `getDesignPieceConnections()` and `getDesignPieceMetadata()` helpers
- sketchpad.test.ts: Add `Drag Propagation` test region with transitive chain and center delta verification

## Log

- Analyzed 4 `.old` files and compared with current Design.tsx
- Neither old nor current code had downstream drag propagation
- Metadata parentPieceId tree is the source of truth for direction
- Implemented propagation: getDownstreamDescendants BFS + onNodeDragStart descendants + onNodeDrag movement + onNodeDragStop persistence
- Discovered bug: dragged node not in pendingPieceUpdatesRef when not pre-selected, causing setNodes(baseNodes) to overwrite drag position
- Fixed by adding dragged node to updatedPieces if not already present
- All tests pass: existing Diagram Node Drag (2 sequential drags) + new Drag Propagation (transitive chain + center deltas)

## Todos

- [x] Analyze old vs current implementation
- [x] Identify prior drag logic
- [x] Implement downstream drag propagation in Design.tsx
- [x] Fix dragged node missing from pendingPieceUpdates
- [x] Expose metadata to window for test accessibility
- [x] Extend sketchpad.test.ts with propagation tests
- [x] Run and verify all tests
- [x] Clean up debug code
- [x] Close ticket

## Plan

1. Add `getDownstreamDescendants(metadata, rootGuids)` helper inside Design.tsx Diagram section
2. In `onNodeDragStart`, compute and store descendants of dragged nodes
3. In `onNodeDrag`, move descendant nodes along with dragged nodes (preserving offsets)
4. In `onNodeDragStop`, include all moved descendants in the `updatePieces` call
5. Always include the dragged node in updatedPieces (even when not in selectionRef)
6. Add test cases to existing sketchpad.test.ts Design test
