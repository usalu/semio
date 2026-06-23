# Plan - Fix Auto Select Bug and Enable Standard Multi Selection in Kit Diagram

## Diagnosis
1.  **Analyze `js/compose/sketchpad/Kit.tsx`**: Search for the diagram interaction logic. Look for where node selection is handled.
2.  **Verify React Flow Configuration**: Check if `selectionOnDrag` and other selection-related props are correctly configured.
3.  **Investigate Event Handlers**: Check if there are custom `onClick` or `onNodeClick` handlers that are forcing selection on every interaction.

## Implementation Tasks
1.  **Modify Diagram Component**:
    -   Enable `selectionOnDrag`.
    -   Ensure standard React Flow selection behavior is preserved (Shift+Click for multiple selection).
    -   Fix the CSS to make the selection rectangle visible if it's currently hidden.
2.  **Sync Selection with App State**:
    -   Ensure that selection changes in the diagram are correctly propagated to the `KitAppStore` (or whoever owns the selection).
    -   Verify that the selection state in the app store is what drives the highlighted nodes in the diagram.
3.  **Refactor Node Interaction**:
    -   Remove any custom "auto-select" logic that triggers on simple clicks or drags if they shouldn't change selection.

## Verification
1.  **Manual Test (User Instructions)**:
    -   Click a node: selects only that node.
    -   Shift+Click another node: adds/removes it from the selection.
    -   Drag on canvas: shows a rectangular selection box and selects nodes inside.
    -   Drag a node: moves the node without auto-selecting everything else if not already selected.
2.  **Run Tests**: Run existing unit/e2e tests for the Kit application to ensure no regressions.
