# Tree Path Hover Highlight

## Summary

Generic tree-path hover highlight for the compose/sketchpad tree system. When any tree element is hovered — row header, content, input/control, or actions — the full ancestor path highlights from that element up to the root, including the terminal branch segment at the hovered depth.

## Changes

### `elements/ui/index.tsx`

- **IndentationLines**: Added `data-slot="tree-guide"` to wrapper div and `data-tree-guide-line` attribute to the actual line div for CSS targeting. Added `transition-[width,background-color] duration-150` for smooth animation.
- **TreeDocumentGutter**: Added `transition-[height,background-color] duration-150` to elbow connector for smooth animation.
- **TreeHoverPath region**:
  - Extracted `rowForBranch()` helper to derive the row element that owns a branch container. Handles all DOM shapes: `tree-item-row`/`control-tree-row` as previous siblings, `tree-section-row` behind `collapsible-content`, `tree-property-item` as parent.
  - Added `resolveHoverRow()` — two-pass resolution: first tries `closest(treeHoverPathRowSelector)` for direct row matches (tree-item-row, tree-section-row, tree-property-item, control-tree-row, tree-content); when no row wrapper exists (pass-through TreeRow, raw controls inside branch containers), falls back to `closest(treeHoverPathBranchSelector)` and derives the owner row via `rowForBranch()`.
  - Added `treeHoverPathBranchSelector` for the fallback detection of branch containers.
  - `markTerminalBranch()` marks the hovered row's own children container (next sibling branch) to highlight the final vertical guide segment at the hovered depth. Handles all row variants.
  - Simplified `applyTreeHoverPath()` by using `rowForBranch()` in the ancestor walk instead of duplicated inline row-discovery logic.
  - Clears hover path when pointer moves to non-row areas (gaps between sections).
- **Tree root component**: `handleTreePointerOver` uses `resolveHoverRow()` for robust detection and clears on miss.

### `elements/ui/globals.css`

- `#region TreeHoverPath` CSS rules:
  - Guide lines in ancestor/terminal branches: `width: 1.5px`, `background-color: color-mix(in srgb, var(--muted-foreground) 80%, transparent)`
  - Elbows in hovered/ancestor rows and tree-content: `height: 1.5px`, same color treatment
- Theme-aware: uses `--muted-foreground` CSS variable which adapts to light/dark mode

## Architecture

Uses DOM-based hover path tracking via event delegation (no React state/context changes needed). Two-pass row resolution: direct row-slot match first, then branch-container fallback for content without a row wrapper. This avoids unnecessary re-renders and handles all tree surfaces generically:

- **Details panel**: TreeSection, TreeItem, TreeRow (with and without labels), TreeContent, SortableTreeItems, property rows
- **Workbench**: TreeItem, ControlTreeRow, ControlTreeFolderRow
- **Nested collections**: Sortable attribute/author items with correct per-item path isolation

The `rowForBranch()` helper centralizes the branch→row derivation used both in hover resolution and ancestor walking, eliminating duplicated logic.
