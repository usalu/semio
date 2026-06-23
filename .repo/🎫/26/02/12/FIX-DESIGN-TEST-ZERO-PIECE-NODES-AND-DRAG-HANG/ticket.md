---
goal: R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS/RUNNING-DESIGN-APP
---

# Ticket: Fix Design Test Zero Piece Nodes and Drag Hang

## Summary
Three bugs fixed to make the Design Playwright test pass:

1. **Hook order violation in PanelTabContent** (Sketchpad.tsx): `useLabel(section.id)` was called inside `.map()` callback, violating Rules of Hooks. When section count changed between renders, React threw "change in the order of Hooks called by PanelTabContent" and unmounted the entire component tree, destroying all 180 ReactFlow nodes. **Fix**: Extracted `PanelTabSectionItem` as a separate FC wrapping each section.

2. **onNodeDragStop center update guard too restrictive** (Design.tsx): The `else if (node && currentSelection?.pieces?.length)` guard prevented updating unselected dragged nodes because `currentSelection.pieces` was empty at capture time. **Fix**: Changed to `else { ... }` and compute `pieceIdsToUpdate` from selection or fallback to dragged node.

3. **KitStore dirty flag not propagated** (Sketchpad.tsx): `DesignStore.markDirty()` only set `this.dirty = true` on itself, NOT propagating to parent `KitStore`. After `PieceStore.change()` updated the CRDT, `KitStore.snapshot()` still returned cached stale data. **Fix**: Added `this.parent.markDirty()` to `DesignStore.markDirty()`.

## Changes
- `compose/js/sketchpad/Sketchpad.tsx`: Extracted `PanelTabSectionItem` component (line ~15274). Added `this.parent.markDirty()` to `DesignStore.markDirty()` (line ~4948).
- `compose/js/sketchpad/Design.tsx`: Changed `onNodeDragStop` fallback from `else if (node && currentSelection?.pieces?.length)` to `else { ... }` with proper pieceId computation (line ~7005). Fixed `useDesignAppUpdatePieces` PieceId mapping (line ~2112).

## Log
- Diagnosed hook order violation via browser error capture: "React has detected a change in the order of Hooks"
- Confirmed 180 nodes survive after PanelTabSectionItem fix
- Traced updatePieces dispatch chain: useDesignAppUpdatePieces → DesignStore.executeCommand → kitStore.change → DesignStore(entity).change → PieceStore.change
- Found KitStore.snapshot() returning stale cached data due to missing dirty propagation
- Design test passes with center changing from (0,0) to (1.8,-0.9) after drag

## Todos
- [x] Fix hook order violation in PanelTabContent
- [x] Fix onNodeDragStop center update guard
- [x] Fix KitStore dirty flag propagation
- [x] Verify Design test passes
- [x] Remove all [DEBUG] console.warn from Design.tsx
- [x] Run all 6 Playwright tests (6 passed in 3.7m)
- [x] Close ticket

## Plan
1. ~~Diagnose why 0 piece nodes render~~ → Hook order violation in PanelTabContent
2. ~~Fix node rendering~~ → Extracted PanelTabSectionItem
3. ~~Fix onNodeDragStop center not updating~~ → Guard too restrictive + dirty flag not propagated
4. Clean up debug logs
5. Run all 6 tests
6. Close ticket
5. Run full test suite to verify all 6 tests pass with drag working