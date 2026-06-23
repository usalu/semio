---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Restored connection translation controls to steppers in the sketchpad detail panel and updated sketchpad Playwright coverage to validate the stepper UI with a default baseURL.
## Changes
- Design.tsx: Replace connection translation `Slider` controls with `Stepper` controls for `gap`, `shift`, and `rise` in single and multiple selection forms
- sketchpad.test.ts: Add a default Playwright `baseURL`, update the existing detail-panel coverage to assert translation steppers, and edit the gap value through the stepper input

## Log
- Reopened the existing detail-panel migration ticket for the connection translation control regression
- Verified the current implementation in `compose/js/sketchpad/Design.tsx` still renders `gap`, `shift`, and `rise` with `Slider`
- Verified the old implementation in `compose/js/sketchpad/Design.Details.tsx.old` renders `Gap`, `Shift`, and `Rise` with `Stepper`
- Reviewed the current `Stepper` implementation in `compose/js/sketchpad/elements.tsx` to confirm it supports numeric editing and labels by id
- Updated the existing Playwright sketchpad coverage to validate the translation controls as steppers instead of sliders
- Added a default `baseURL` in `sketchpad.test.ts` because the repo does not have a tracked Playwright config and `page.goto("/")` otherwise fails locally
- Started the sketchpad dev server and reran `npx playwright test sketchpad.test.ts --grep "Design" --timeout 240000 --workers=1 --max-failures=1 --reporter=list` with `PLAYWRIGHT_BASE_URL=http://127.0.0.1:5175`
- Verified the `Design` Playwright test passed and confirmed the gap batch edit still updates all selected connections to `5.5`

## Todos
- [x] Reopen the matching detail-panel migration ticket
- [x] Compare the current and old connection translation controls
- [x] Replace `gap`, `shift`, and `rise` sliders with steppers
- [x] Update the existing sketchpad test to match the stepper UI
- [x] Run the relevant test coverage

## Plan
1. Restore the old-build control kind for connection translation inputs in `Design.tsx`
2. Refactor the existing Playwright test to assert stepper rendering and stepper-based editing
3. Run the relevant sketchpad test coverage and record the result
