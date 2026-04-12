---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Implemented geometric paddingLeft formula for all detail panel tree components. Base padding is 10px at level 0, multiplied by 1.5 for each subsequent hierarchy level (level 0: 10px, level 1: 15px, level 2: 22.5px, level 3: 33.75px). Indentation guide lines repositioned to match.

## Changes

- Added `detailPanelIndentPx(level)` helper: `10 * 1.5^level` px
- Added `indentationLinePx(i)` helper: midpoint of adjacent level paddings
- Updated `IndentationLines` line positions from `i * 0.75 + 0.375rem` to `indentationLinePx(i)px`
- Updated `TreeContent` paddingLeft from `level * 0.75 + 1.25rem` to `detailPanelIndentPx(level) + 20px`
- Updated `TreeSection` paddingLeft (2 locations) from `level * 0.75rem` to `detailPanelIndentPx(level)px`
- Updated `SortableTreeItem` paddingLeft from `level * 0.75rem` to `detailPanelIndentPx(level)px`
- Updated `TreeItem` paddingLeft (2 locations) from `level * 0.75rem` to `detailPanelIndentPx(level)px`
- Updated `FileTreeItem` paddingLeft from `level * 1 + 0.75rem` to `detailPanelIndentPx(level) + 12px`

## Log

## Todos

## Plan
- Add `detailPanelIndentPx` helper: `10 * 1.5^level` px
- Update TreeSection paddingLeft (2 locations: no-children and collapsible)
- Update TreeContent paddingLeft
- Update TreeItem paddingLeft (3 locations: with-children, leaf, label-less fallback)
- Update SortableTreeItem paddingLeft (2 locations: with-children and leaf)
- Update IndentationLines line positions to match new geometric spacing
- Update FileTreeItem paddingLeft
- Run tests to verify