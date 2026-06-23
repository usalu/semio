---
name: Tree Row Remount Fix
overview: Eliminate the Ctrl+A ~2s 3D freeze by fixing the shared Tree component so its per-row renderers stop being redefined on every render, which currently forces React to unmount and remount all ~700 workbench hierarchy rows whenever selection changes.
todos:
  - id: context
    content: Add module-level TreeDataContext in ui/react carrying row maps, resolvedSelectedIds, dragAndDropController, and all row handlers
    status: completed
  - id: hoist
    content: Move DataItemView and DataSectionView to module scope (memoized), consuming TreeDataContext and keeping per-row useTreeOpenState/branch/effect hooks
    status: completed
  - id: provider
    content: In Tree, build memoized context value and wrap sections render in TreeDataContext.Provider
    status: completed
  - id: tests
    content: Run @semio-tech/ui-react:test, @semio-tech/puzzle-3d-react:test, @semio-tech/puzzle-3d-play:test; fix any breaks
    status: completed
  - id: runtime
    content: "Runtime-verify Ctrl+A on Nakagin: instant 3D highlight, working expand/collapse + drag"
    status: completed
  - id: ticket
    content: Reopen and close ticket 26/06/01/PUZZLE3D-SELECTION-PERF with summary and files
    status: completed
isProject: false
---

## Root cause

`Tree` in [ui/react/index.tsx](ui/react/index.tsx) defines its recursive row renderers `DataItemView` (line ~8986) and `DataSectionView` (line ~9050) **inside the component body**. Each `Tree` render gives them a new function identity, so React treats them as new component types and **unmounts/remounts every row** (DOM teardown + all per-row effects rerun).

On Ctrl+A, `selectAllSelection` selects ~180 objects + ~358 vortices, then `emit()` bumps `runtime.generation`. `DeclarativeSidePanelBody` ([framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) line ~728) rebuilds `buildPuzzle3dPlayHierarchyTree` (fully expanded via `defaultOpen: true`, ~700 rows) and re-renders `<Tree>`, remounting all rows. Since `emit()` is deferred only with `queueMicrotask` (pre-paint), the 3D highlight cannot paint until that remount finishes -> "3D frozen ~2s". The 3D scene itself is already fine.

## Fix: hoist row renderers out of `Tree` (ui/react)

Refactor so `DataItemView`/`DataSectionView` have stable module-level identity and rows reconcile instead of remounting.

- Add a module-level `TreeDataContext` providing everything the rows currently close over:
  - maps/state: `sectionItemsById`, `itemItemsById`, `loadingById`, `resolvedSelectedIds`, `resolvedSections`
  - controller: `dragAndDropController`
  - handlers (already `useCallback`): `handleSelectItem`, `handleDoubleClickItem`, `handleDragStart`, `handleDragEnd`, `handleDragOver`, `handleDrop`, `buildPalettePointerProps`, `resolveItemDragData`, `loadItemItems`, `loadSectionItems`
- Move `DataItemView` and `DataSectionView` to module scope (just below `Tree`). They:
  - read shared data via `useContext(TreeDataContext)`
  - keep their own per-row hooks unchanged: `useState(activeBranchIndex)`, `useTreeOpenState(...)`, the `useEffect` lazy-load
  - recurse by referencing the hoisted `DataItemView` (stable identity)
- In `Tree`, build a single `contextValue` (memoized; handlers stable, maps from state) and render `<TreeDataContext.Provider value={contextValue}>` around the `resolvedSections.map(... <DataSectionView .../>)` block (lines ~9105-9118).
- Wrap the hoisted `DataItemView`/`DataSectionView` in `reactHostPort.memo` to skip untouched rows on unrelated re-renders.

Per-row open/branch state is unaffected: `useTreeOpenState` persists by id in `TreeStateProvider` above the rows, and `TreeContext` level nesting is already handled inside `TreeItem`/`TreeSection`.

Expected result: the Ctrl+A generation bump reconciles ~700 rows (no unmount/remount, no effect churn) instead of tearing down and rebuilding them, removing the multi-second block and letting the 3D highlight paint immediately. This also speeds up every other panel using `Tree` (e.g. puzzle 2d).

## Validation

- Run `@semio-tech/ui-react:test` (Tree behavior: selection, expand/collapse, drag) and confirm green.
- Run `@semio-tech/puzzle-3d-react:test` and `@semio-tech/puzzle-3d-play:test` (unchanged, sanity).
- Runtime-check Ctrl+A on the Nakagin fixture: 3D selection should appear within ~1 frame, no multi-second freeze; expand/collapse and drag in the hierarchy still work.

## Ticket

- Reopen `26/06/01/PUZZLE3D-SELECTION-PERF` (repo MCP `ticket_reopen`), close with summary + touched files when done.

## Optional follow-up (not in this change unless wanted)

The hierarchy still fully rebuilds its node tree on every `generation` bump. A later optimization could drive selection through `Tree`'s `selectedIds` prop instead of baking `isSelected` into rebuilt nodes, so selection changes skip the rebuild entirely.