---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Moved the WindowLibrary (windows) section from the workbench tab to the tools tab in the left toggle panel of the Design app. Changed addSection/removeSection panel key from "workbench" to "tools" in Design.tsx.
## Changes
- `semio/js/sketchpad/Design.tsx`: Changed `addSection("workbench", ...)` to `addSection("tools", ...)` for the windows section, and updated the cleanup `removeSection` call accordingly. Set order to 1 since it's the first section in the tools tab.

## Log
- Identified windows section registered at Design.tsx:10126 under "workbench" panel
- Confirmed TOOLS panel is already registered in Design app's getPanels()
- Moved section registration from "workbench" to "tools" panel key

## Todos
- [x] Move windows addSection from "workbench" to "tools"
- [x] Update removeSection cleanup
- [x] Verify TOOLS panel is registered in Design app

## Plan
1. Change `addSection("workbench", {...windows...})` to `addSection("tools", {...windows...})`
2. Update the cleanup return to `removeSection("tools", ...)`
