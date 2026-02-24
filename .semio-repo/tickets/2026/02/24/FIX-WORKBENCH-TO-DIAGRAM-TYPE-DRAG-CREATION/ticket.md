---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Normalized design workbench drag-drop activator handling (pointer/mouse/touch) so diagram drops create a new piece reliably; extended existing Design e2e coverage for mouse-activator drag-end path; Design test run remains blocked by pre-existing timeout in workbench section expansion.
## Changes
- Updated `semio/js/sketchpad/Design.tsx`:
- Added shared drag activator coordinate resolver supporting pointer, mouse, and touch events.
- Removed `PointerEvent`-only guard from diagram and scene drag-end handlers.
- Updated `semio/js/sketchpad.test.ts` (existing test file only):
- Added explicit synthetic `design-drag-end` assertion using `MouseEvent` activator.
- Asserted new piece creation and source `typeGuid` match for this path.

## Log
- Reopened existing ticket for same scope.
- Gathered context with `./semio-repo/cli/cli tree "workbench diagram drag drop type"` and inspected Design drag handlers.
- Implemented drag-end activator normalization in `Design.tsx` and extended existing Design test path in `sketchpad.test.ts`.
- Ran targeted verification:
- `cd semio/js && npx playwright test sketchpad.test.ts --grep "Design" --timeout 240000 --workers=1 --max-failures=1 --reporter=list`
- Result: failed by pre-existing `Design` test timeout at `semio/js/sketchpad.test.ts:2148` after repeated inability to expand workbench sections (`draggableWorkbenchAvatarCount` stayed `0` through section `60`).

## Todos
- [x] Reuse existing sketchpad test file.
- [x] Fix workbench-to-diagram drag-end handling for pointer/mouse/touch activator events.
- [x] Add explicit existing-test coverage for mouse-activator drag-end piece creation.
- [x] Validate targeted Playwright run result and record exact outcome.

## Plan
1. Normalize drag-end coordinate extraction from activator events in Design handlers.
2. Keep existing workbench drag flow and only fix drop acceptance logic.
3. Extend existing Design test to prove mouse-activator drag-end creates the expected piece.
4. Run targeted Design Playwright test and capture the exact status.
