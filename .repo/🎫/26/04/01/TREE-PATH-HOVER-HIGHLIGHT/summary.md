# Tree Path Hover Highlight

## Summary

Implemented generic tree-path hover highlight for the semio/sketchpad tree system. When any tree row is hovered, the full ancestor path (vertical guide lines + horizontal elbows) highlights from that row up to the root.

## Changes

### `elements/ui/index.tsx`

- **IndentationLines**: Added `data-slot="tree-guide"` to wrapper div and `data-tree-guide-line` attribute to the actual line div for CSS targeting. Added `transition-[width,background-color] duration-150` for smooth animation.
- **TreeHierarchyGutter**: Added `transition-[height,background-color] duration-150` to elbow connector for smooth animation.
- **TreeHoverPath region**: Added helper functions `clearTreeHoverPath` and `applyTreeHoverPath` that use DOM walking to mark ancestor branch containers (`data-tree-hover-path="branch"`) and ancestor rows (`data-tree-hover-path="row"`) when a tree row is hovered. Supports all tree row variants: `tree-item-row`, `tree-section-row`, `tree-property-item`, `control-tree-row`.
- **Tree root component**: Added `ref`, `onPointerOver`, and `onPointerLeave` event handlers using event delegation. The `onPointerOver` finds the closest tree row from the pointer target and applies the hover path. The `onPointerLeave` clears all hover path attributes.
- **Tests**: Updated the guide rendering test to match new class names and data attributes. Fixed pre-existing duplicated code at end of file.

### `elements/ui/globals.css`

- Added `#region TreeHoverPath` CSS rules:
  - Guide lines in ancestor branches: `width: 1.5px`, `background-color: color-mix(in srgb, var(--muted-foreground) 80%, transparent)`
  - Elbows in hovered/ancestor rows: `height: 1.5px`, same color treatment
- Theme-aware: uses `--muted-foreground` CSS variable which adapts to light/dark mode

## Architecture

Uses DOM-based hover path tracking via event delegation (no React state/context changes needed). This avoids unnecessary re-renders and correctly identifies the ancestor chain through DOM containment. CSS attribute selectors with `>` (direct child) ensure only the correct branch's guide lines are highlighted.
