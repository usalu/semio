---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Added a wrapper-level wheel zoom fallback for the embedded design diagram and revalidated zoom plus fullscreen remount retention with focused Playwright coverage.
## Changes
- `semio/js/sketchpad/Design.tsx`: Persist `diagramScale` alongside `diagramCenter` when viewport movement ends.
- `semio/js/sketchpad/Design.tsx`: Persist viewport state from live React Flow move events and keep center-reset actions aligned with zoom state.
- `semio/js/sketchpad/elements.tsx`: Forward React Flow `onMove` viewport updates through the shared `Diagram` wrapper.
- `semio/js/sketchpad/elements.tsx`: Add a wheel-capture zoom fallback on the diagram wrapper so embedded design panels zoom even when the internal React Flow wheel handler does not.
- `semio/js/sketchpad.test.ts`: Extend the existing Playwright coverage with a regression test that verifies wheel zoom updates persisted Design app scale state.
- `semio/js/sketchpad.test.ts`: Reworked the zoom regression to verify the user-facing behavior that zoom survives the existing diagram fullscreen toggle.

## Log
- Confirmed the Design diagram already restores `savedDiagramScale` on mount, but never writes updated zoom back after user interaction.
- Narrowed the bug to `DesignDiagram.onMoveEnd`, which only persisted `diagramCenter` and left `setDiagramScale` unused.
- A direct actor-state assertion was unreliable in this harness, so the regression was moved to the visible fullscreen remount behavior the user can reproduce.
- Added `onMove` plumbing through the shared diagram wrapper because `onMoveEnd` alone was not sufficient for wheel-based zoom persistence.
- Focused Playwright validation passed with `npx playwright test sketchpad.test.ts --grep "Design Zoom Persists Across Fullscreen Toggle" --reporter=line` after rerunning outside the sandbox to let Chromium launch.
- User follow-up clarified the remaining defect: the embedded design diagram still could not zoom in regular use.
- Added a wrapper-level `onWheelCapture` fallback that computes the next viewport around the cursor and updates the React Flow instance directly before forwarding the new viewport through the existing persistence path.
- Revalidated with the same focused Playwright zoom test after the wheel fallback and the embedded design panel still zooms and retains scale across the fullscreen remount path.

## Todos
- [x] Persist zoom changes on viewport move end.
- [x] Extend the existing sketchpad test file to cover zoom state persistence.
- [x] Run focused validation for the updated wheel interaction path.
- [x] Close the ticket with the touched files.

## Plan
- Update the existing viewport move-end persistence path in `semio/js/sketchpad/Design.tsx` to save both center and scale from the active React Flow viewport.
- Add a focused design-app regression in `semio/js/sketchpad.test.ts` that zooms the diagram and asserts the stored Design app `diagramScale` tracks the viewport scale.
- Execute the focused Playwright test and record the result before closing the ticket.
