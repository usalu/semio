---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Bulk close
## Changes
- Refactor `semio/js/sketchpad/elements.tsx` so `SidePanel` mounts function-valued tab content with JSX instead of invoking it directly during `SidePanel` render
- Extend the existing `semio/js/sketchpad.test.ts` panel-combination coverage to assert the design chat side panel content stays mounted when the navbar chat toggle is used
- Update the existing right-panel utility tab assertion in `semio/js/sketchpad.test.ts` to match the current embedded utility-tab UI instead of the retired zero-tab expectation

## Log
- Verified the crash source in `Design.tsx`: the design chat side panel registers a function-valued `content` callback that calls `useLabel`
- Verified `SidePanel` in `elements.tsx` currently executes `activeTab.content()` inline, causing hooks in tab content to be counted as `SidePanel` hooks
- Identified the existing Playwright regression block in `semio/js/sketchpad.test.ts` that already exercises the navbar chat toggle in the design app
- Ran `npx playwright test semio/js/sketchpad.test.ts --grep "Panels" --reporter=line` and hit an unrelated stale assertion expecting zero embedded utility tabs in the Home right panel even though the current UI renders two
- Updated the stale assertion, reran `npx playwright test semio/js/sketchpad.test.ts --grep "Panels" --reporter=line`, and verified the full `Panels` test passed in 4.6 minutes

## Todos
- [x] Isolate the crash path and identify the failing hook boundary
- [x] Refactor the shared side panel content renderer
- [x] Extend the existing sketchpad Playwright coverage
- [x] Align the stale right-panel utility tab assertion with the current UI
- [x] Run the affected regression coverage

## Plan
1. Refactor `SidePanel` to render function-valued tab content as a component boundary
2. Extend the existing design panel combination regression to assert the chat side panel remains visible after the navbar chat toggle
3. Run the relevant Playwright coverage and record the result
