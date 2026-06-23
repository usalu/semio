# Fix Design App Piece Snapping on Connection

## Goal
SKETCHPAD-IMPROVEMENTS

## Prompt
Pieces don't snap together (connector piece A to connector piece B) in the scene window after creating a connection between two connectors from different pieces in the diagram window.

## Plan
1. Understand flattenDesign snapping logic in compose.ts
2. Understand connection creation flow in Design.tsx diagram
3. Find root cause of missing snap
4. Implement fix
5. Verify fix works with tests

## TODOs
- [x] Understand flattenDesign snapping logic
- [x] Understand connection creation in diagram
- [x] Find root cause of missing snap
- [x] Implement snapping fix
- [x] Verify fix works with tests

## Root Cause
In `ModelPiece` (Design.tsx Scene region), the plane used for 3D rendering was:
```tsx
const originalPlane = piece.plane || flatPlane;
```
This always used `piece.plane` (the stored/original plane from piece creation) when available, ignoring `flatPlane` (the computed plane from `flattenDesign` via `useFlatPiecePlane()`). Since pieces created via drag-and-drop always have `plane` set, the flattened/snapped plane from connections was never used for rendering.

The `flattenDesign` function correctly computes child piece planes based on parent piece + connectors + connection geometry. The `usePiecesMetadataMap` hook correctly detects connection changes and recomputes. But the `ModelPiece` render logic short-circuited the flat plane with the stored plane.

## Fix
Changed `ModelPiece` to check `useIsConnectedPiece()`:
- **Root pieces** (no parent connection): Use `piece.plane || flatPlane` (preserves user-placed position)
- **Connected child pieces**: Use `flatPlane` (the computed snap plane from `flattenDesign`)

This ensures that when a connection is created between two pieces, the child piece snaps to the computed position based on connector geometry.

## Changes
- `compose/js/sketchpad/Design.tsx`:
  - Added `useIsConnectedPiece` to imports from `./Sketchpad`
  - Changed `ModelPiece` plane resolution to use `flatPlane` for connected child pieces

## Summary
Fixed piece snapping by using the computed flat plane from flattenDesign for connected child pieces in the Scene window. Root cause was ModelPiece always preferring stored piece.plane over the computed flatPlane. All 14 unit tests pass.
