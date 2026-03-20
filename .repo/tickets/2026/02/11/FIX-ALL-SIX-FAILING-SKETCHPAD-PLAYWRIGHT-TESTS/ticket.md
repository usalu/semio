---
goal: fix-e2e-tests
---

# Ticket

## Summary

All 6 sketchpad playwright tests pass. Fixed: (1) CSS import path in globals.css, (2) toolbar auto-expand in Sketchpad.tsx, (3) role=treeitem on TreeItem/SortableTreeItem in elements.tsx, (4) HUD panel pieces section in Design.tsx using usePieces() in App scope, (5) Feedback.tsx bypassed LayoutCanvas and fixed duplicate import.
## Changes
- `globals.css`: Fixed @import path from `"semio/js/theme.css"` to `"./theme.css"`
- `Sketchpad.tsx`: Added useEffect to auto-expand first available toolbar group
- `elements.tsx`: Added `role="treeitem"` to TreeItem and SortableTreeItem components
- `Design.tsx`: Added `usePieces()` hook in App function, registered "hud" section with pieces as TreeItems (using data closure, not nested component with scope-dependent hooks)
- `Feedback.tsx`: Bypassed LayoutCanvas (GoldenLayout v2 duplicated single-window layouts), render FeedbackForm directly in Canvas; removed duplicate import; cleaned unused imports/variables

## Log
- Session 1: Fixed CSS import (all 6 tests were failing due to blank page)
- Session 1: Added toolbar auto-expand (Home, Kit, Feedback toolbar items now visible)
- Session 1: Added role="treeitem" to TreeItem/SortableTreeItem (left panel detection works)
- Session 1: 4/6 pass (Home, Kit, Type, Docs)
- Session 2: Implemented HUD pieces section in Design.tsx (initial approach used PiecesHudContent component with useDesign() hook)
- Session 2: Fixed Feedback by bypassing LayoutCanvas entirely (GoldenLayout v2 creates 2 stacks for single-component)
- Session 2: Fixed duplicate import in Feedback.tsx (caused all 6 tests to regress)
- Session 2: Fixed HUD panel: replaced PiecesHudContent component with usePieces() called in App function scope (DesignScopeProvider only available inside GoldenLayout windows, not in HUD panel render context)
- Session 2: All 6 tests pass (6 passed, 2.7m)

## Todos
- [x] Fix CSS import in globals.css
- [x] Auto-expand toolbar groups in Sketchpad.tsx
- [x] Add role="treeitem" to TreeItem/SortableTreeItem in elements.tsx
- [x] Add "hud" section with pieces tree in Design.tsx
- [x] Fix Feedback.tsx (bypass LayoutCanvas, fix duplicate import)
- [x] Run all 6 tests and verify they pass
