---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Filtered utility tabs out of the shared right panel across sketchpad apps while preserving navbar toggle access; extended the existing Panels Playwright spec with cross-app assertions.
## Changes
- Updated `compose/js/sketchpad/Design.tsx`:
- Removed `settings` and `chat` components from Design window layout defaults and window kind registrations.
- Added layout sanitization that removes legacy `settings/chat` window nodes from stored layouts and persists migrated layout.
- Registered Design `settings` and `chat` as right side panel tabs via `useAddSidePanelTab`.
- Updated `compose/js/sketchpad/Sketchpad.tsx`:
- Changed navbar Chat/Settings toggle handlers to open right side panel and activate matching right tab.
- Chat/Settings render in the right panel slot; left panel handles only left-side tabs.
- Right panel resolves Chat/Settings tab content from side tab registry and renders fallback placeholders when absent.
- Filtered Chat/Settings out of normal right property panel tabs so they cannot be opened from property-panel tab area.
- Updated `compose/js/sketchpad/elements.tsx`:
- Side panel tab strip is now hidden when only one tab exists, so property panel renders without tab controls.
- Updated `compose/js/sketchpad/Sketchpad.tsx`:
- Split right-side tabs into workspace tabs and utility tabs in the shared shell.
- Normal right-panel rendering now receives only workspace tabs in every app.
- Navbar Chat/Settings toggles still read utility content from the utility tab subset, so behavior stays intact without exposing those tabs in the right-panel strip.
- Updated `compose/js/sketchpad.test.ts`:
- Extended the existing `Panels` Playwright coverage to assert that the open right panel does not show `.settings` or `.chat` tabs in Home, Kit, Design, and Type flows.
- Validation:
- `npm --prefix compose/js run test:unit` passed (12/12).
- `npx tsc -p compose/js/tsconfig.json --noEmit` passed.
- Re-ran `npx tsc -p compose/js/tsconfig.json --noEmit` after the shared-shell refactor: passed.
- Ran `npx playwright test sketchpad.test.ts --grep "Panels" --reporter=line` from `compose/js`: failed before assertions because `page.goto("/")` had no valid base URL in the current test invocation (`Protocol error (Page.navigate): Cannot navigate to invalid URL`).
- Re-ran both checks after final side-panel tab registration edits: still passing.
- Re-ran checks after property-panel tab filtering and single-tab strip removal: still passing.
- `npm --prefix compose/js run test:e2e` failed: Playwright webServer startup failure.
- `npm --prefix compose/js run build` failed due existing Storybook indexing issue in `.storybook/stories/elements/aggregation/Resizable.stories.tsx` (`Duplicate declaration "Panel"`), unrelated to this ticket.

## Log
- Opened ticket for left-panel Chat/Settings migration.
- Refactored Design window config and persisted migration behavior for old stored layouts.
- Refactored Sketchpad panel routing so Chat/Settings render at left side panel position.
- Reopened after requirement correction and moved Chat/Settings placement to right side panel position.
- Reopened again to remove property-panel tab access for Chat/Settings and remove tab strip from single-tab property panels.
- Reopened again to complete the cross-app removal by filtering utility tabs from the shared right-panel tab list instead of relying on per-app behavior.
- Ran tests and compile checks; recorded unrelated existing failures.

## Todos
- Verify UI manually in sketchpad Design route: property panel has no tabs, and Chat/Settings open only from navbar toggles.
- Add/update e2e assertions once Playwright webServer startup issue is fixed.
- Run the updated cross-app `Panels` Playwright coverage and confirm the new assertions pass.

## Plan
- Remove Chat/Settings from Design scene window layout.
- Register Chat/Settings as right side panel tabs in Design app.
- Route navbar Chat/Settings toggles to right side panel presentation path.
- Remove Chat/Settings from property-panel tab list and hide single-tab side panel tab strip.
- Finish the shared right-panel filtering so the removal applies uniformly to Home, Kit, Design, Type, and other apps using the same shell.
- Run unit/compile validations and record results.
