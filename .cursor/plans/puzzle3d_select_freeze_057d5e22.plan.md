---
name: puzzle3d select freeze
overview: Eliminate the ~6s UI freeze when selecting an object in the puzzle 3D play harness by routing selection through the Tree's existing per-id selectedIds store instead of rebuilding and fully re-rendering the entire 538-node document tree on every click.
todos:
  - id: ticket
    content: Open repo MCP ticket for the puzzle 3D selection freeze and associate with the appropriate goal
    status: completed
  - id: core-node
    content: Add selectedIds/highlightedIds to UiTreeNode and playgroundTreePanelRootItems in playground core
    status: completed
  - id: renderer
    content: Memoize TreeData conversion by sections identity and pass selectedIds/highlightedIds to <Tree> in DeclarativeTreeWorkbenchPanel/DeclarativeSidePanelBody
    status: completed
  - id: play-tree
    content: Build stable structural document tree (no per-item selected), derive selectedIds, and cache sections by fixtureRevision in Puzzle3dPlayShellController
    status: completed
  - id: tests
    content: Extend existing play and framework renderer tests for stable sections identity + selectedIds; run suites and confirm green
    status: completed
  - id: verify
    content: Verify selection is instant (temporary [DEBUG] timing), then remove debug logs and close ticket
    status: completed
isProject: false
---

# Fix Puzzle 3D Selection Freeze

## Root cause
Each object click bumps the full shell generation (`emit()`), which rebuilds the entire document tree (~538 nodes: 180 objects + 358 vortices) with selection baked into each row, hands `<Tree>` a brand-new `sections` array, and forces all `TreeDataItemView` rows to re-render (memo fails on new item/path objects). In dev/StrictMode this is the ~6s freeze.

- Bump + rebuild: [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) L721-729, L1126-1140, L433-491, L1929-1933
- Fresh TreeData each render + effect reset: [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) L397-412, L719-740; [ui/react/index.tsx](ui/react/index.tsx) L9325-9335, L9129-9211
- The canvas is NOT the cause (scene children + meshes are memoized): [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) L8588-8598

The `<Tree>` already supports efficient selection via a `selectedIds` prop and a per-id `TreeSelectionStore` (`useSyncExternalStore`), so only changed rows re-render: [ui/react/index.tsx](ui/react/index.tsx) L7814, L7874-7882, L9270-9319. The fix uses that path.

## Required invariants for the win
1. The document `sections` array identity must stay STABLE across selection-only changes (only change when fixture structure changes).
2. The renderer must NOT re-map to new `TreeData` arrays on every render; it must memoize on `sections` identity.
3. Selection must flow via `selectedIds` (Tree store), not via per-item `isSelected` in the rebuilt data.

## Changes

### 1. Playground core: carry selection on the tree node
[framework/product/playground/core/index.ts](framework/product/playground/core/index.ts)
- Add optional `readonly selectedIds?: readonly string[]` and `readonly highlightedIds?: readonly string[]` to `UiTreeNode` (L164-167).
- Update `playgroundTreePanelRootItems` (L208-216) to accept an optional `{ selectedIds?, highlightedIds? }` and include them on the returned node.

### 2. Playground renderer: memoized conversion + controlled selection
[framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)
- Replace the plain `renderPlaygroundDeclarativeTree` usage in `DeclarativeTreeWorkbenchPanel` (L719-740) and `DeclarativeSidePanelBody` (L743-768) with logic that:
  - `useMemo`s `uiTreeSectionsToTreeData(node.sections, bus)` and `buildUiTreeDragAndDropController(node.sections, bus)` keyed on `node.sections` identity (so a stable `sections` yields stable `TreeData` arrays across generations).
  - Passes `selectedIds={node.selectedIds as string[] | undefined}` and `highlightedIds={node.highlightedIds}` to `<Tree>`.
- Keep per-item `isSelected` working for other callers, but the 3D document will stop setting it (selection comes from `selectedIds`).

### 3. Puzzle 3D play: stable structural tree + selectedIds
[puzzle/3d/play/index.ts](puzzle/3d/play/index.ts)
- Split `buildPuzzle3dPlayDocumentTree` (L433-491): build the structural tree WITHOUT `selected` flags on rows; add a helper `puzzle3dPlayDocumentSelectedIds(selection)` that maps selection to row ids (`puzzle-3d-play-document.object.<id>`, `...vortex.<fullId>`, `...attraction.<id>`).
- Cache the structural tree in `Puzzle3dPlayShellController` keyed by `fixtureRevision` (and fixture identity), so `buildPuzzle3dPlayDocumentPanelBody` (L1929-1933) returns the SAME `sections` object across selection-only generations, attaching the freshly derived `selectedIds`.
- No change needed to `notifySelection`/`emit`: the small inspector still rebuilds correctly; only the tree stops thrashing.

### 4. Tests (extend existing files, no new files)
- [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) inline tests (around L2279-2505): assert the document `sections` identity is stable across two selections of the same fixture, and that `selectedIds` reflects the picked row.
- Framework renderer tests: assert `<Tree>` receives stable `sections` + updated `selectedIds` when only selection changes.
- Run the existing puzzle 3D play test suites via nx; confirm green.

## Validation
- Run the puzzle 3D play suite (`nx` test for `@semio-tech/puzzle-3d-play` and the framework playground renderer) and confirm pass.
- Manually confirm selecting an object is instant (add a temporary `[DEBUG]` timing log around the tree render to verify only affected rows re-render, then remove).

## Scope note
The framework renderer change benefits puzzle 2D/5D play too, but they will only get the speedup once their play builders also provide stable `sections` + `selectedIds` (same pattern). Out of scope for this ticket unless requested.

## Process
Open a repo MCP ticket (e.g. `Puzzle 3D Selection Freeze`) before editing; close it with a summary and touched files when done. Use regions/subregions; start docstrings with an emoji; use `kind` not `type` for any new domain naming.