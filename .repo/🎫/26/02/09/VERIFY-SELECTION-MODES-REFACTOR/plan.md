# Plan - Verify Selection Modes Refactor

1. **Verify `SELECTION_INTERSECT` Logic:**
   - [x] Check `onNodeDragStart` in `Design.tsx` handles `ToolKind.SELECTION_INTERSECT`.
   - [x] Check `onNodeDragStop` in `Design.tsx` handles `action === "intersect"`.

2. **Verify Tool Toggles:**
   - [x] Verify `DesignSelectSettings` renders toggles for additive/subtractive/intersect.
   - [x] Verify `activeTool` updates correctly when toggles are clicked.

3. **Verify Behavior (Mental Check/Code Review):**
   - Additive: `pendingSelection` has `action: "add"`. `onNodeDragStop` calls `addPieceToSelection`. Correct.
   - Subtractive: `pendingSelection` has `action: "remove"`. `onNodeDragStop` calls `removePieceFromSelection`. Correct.
   - Intersect: `pendingSelection` has `action: "intersect"`. `onNodeDragStop` checks if already selected, then `selectPiece` (exclusive select of that piece) or `deselectAll`.
     - *Correction*: This logic is for single click intersection. If I drag-select, `onNodeDragStop` only runs for the node I started dragging on? Yes, `onNodeDragStop` is for Dnd-Kit drag.
     - Is there a rectangular selection that needs this? The request mentions "selection tools", implying the general selection mode. Rectangular selection usually uses `onSelectionChange` from React Flow or a custom lasso.
     - `Design.tsx` has `useDesignAppSelection` and usually handles react flow selection changes via `onSelectionChange`.
     - Let's check `onSelectionChange`.

4. **Verify Rectangular Selection (Lasso):**
   - If I use the lasso (standard React Flow selection), `onSelectionChange` is called.
   - I should check if `onSelectionChange` respects the active tool mode.
