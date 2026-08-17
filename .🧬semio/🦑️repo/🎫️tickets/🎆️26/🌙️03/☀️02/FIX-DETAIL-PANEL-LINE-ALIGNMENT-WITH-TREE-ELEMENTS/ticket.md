---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Final continuation complete: TreeRow no longer wraps rows in TreeContent indentation, and tree-aware Label now renders document indentation/lines only in the left cell while controls remain fixed in a 160px right column across nesting levels. compose/js unit tests pass.

## Changes

- Replaced nested `TreeContent` leaf rendering in `ControlTree` with dedicated shared grid rows.
- Added fixed control column sizing for all rows (`minmax(0, 1fr)` + `160px`).
- Kept folder nesting and collapse behavior while moving document-only layout concerns into left cells.
- Moved control labels to left tree column and removed duplicated inline control labels in ControlTree renderer paths.
- Added tree-aware `Label` layout switch: tree rows now use `grid-cols-[minmax(0,1fr)_160px]`, non-tree rows keep `grid-cols-[96px_1fr]`.
- Extended `TreeContext` with `isTree` and propagated this through tree providers to scope fixed-column behavior to tree UIs.
- Removed `TreeContent` wrapping from `TreeRow` so form controls are no longer nested inside depth-indented subtree content.
- Added tree-specific label-cell indentation and line rendering inside `Label` for tree contexts (`IndentationLines` + left spacer).
- Added/updated tests for ControlTree tree building behavior in existing unit suite.
- Verified `npm --workspace compose/js test -- compose.test.ts` passes with 16/16 tests.

## Log

- Reopened this ticket to implement right toggle panel two-column alignment refactor.
- Implemented ControlTree-specific row components (`ControlTreeRow`, `ControlTreeFolderRow`, `ControlTreeLeafRow`) in `compose/js/sketchpad/elements.tsx`.
- Confirmed nested folders still collapse/expand and keep indentation lines in the left column only.
- Continued by aligning real `TreeRow` label/control rows in tree contexts via `Label` + `TreeContext.isTree`.
- Final continuation: eliminated TreeRow depth shift by rendering children directly and letting tree-aware Label own left-side document indentation.
- Ran JS unit tests successfully after refactor.

## Todos

- [x] Implement shared grid row layout for ControlTree folders and controls.
- [x] Keep collapse/expand and nesting behavior intact.
- [x] Ensure right control/value column x-position is fixed for every row depth.
- [x] Extend existing tests and run them.

## Plan

- Refactor only the `ControlTree` section in `compose/js/sketchpad/elements.tsx`.
- Avoid broad `TreeItem`/`TreeSection` regressions by introducing ControlTree-specific row renderers.
- Validate with existing JS unit tests.
