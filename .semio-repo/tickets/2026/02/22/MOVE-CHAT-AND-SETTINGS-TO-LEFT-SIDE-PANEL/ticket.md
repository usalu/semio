---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Property panel no longer exposes tab controls for single-tab views; chat/settings are reachable only via navbar toggles on right side and remain out of scene tabs.
## Changes
- Updated `semio/js/sketchpad/Design.tsx`:
- Removed `settings` and `chat` components from Design window layout defaults and window kind registrations.
- Added layout sanitization that removes legacy `settings/chat` window nodes from stored layouts and persists migrated layout.
- Registered Design `settings` and `chat` as right side panel tabs via `useAddSidePanelTab`.
- Updated `semio/js/sketchpad/Sketchpad.tsx`:
- Changed navbar Chat/Settings toggle handlers to open right side panel and activate matching right tab.
- Chat/Settings render in the right panel slot; left panel handles only left-side tabs.
- Right panel resolves Chat/Settings tab content from side tab registry and renders fallback placeholders when absent.
- Filtered Chat/Settings out of normal right property panel tabs so they cannot be opened from property-panel tab area.
- Updated `semio/js/sketchpad/elements.tsx`:
- Side panel tab strip is now hidden when only one tab exists, so property panel renders without tab controls.
- Validation:
- `npm --prefix semio/js run test:unit` passed (12/12).
- `npx tsc -p semio/js/tsconfig.json --noEmit` passed.
- Re-ran both checks after final side-panel tab registration edits: still passing.
- Re-ran checks after property-panel tab filtering and single-tab strip removal: still passing.
- `npm --prefix semio/js run test:e2e` failed: Playwright webServer startup failure.
- `npm --prefix semio/js run build` failed due existing Storybook indexing issue in `.storybook/stories/elements/aggregation/Resizable.stories.tsx` (`Duplicate declaration "Panel"`), unrelated to this ticket.

## Log
- Opened ticket for left-panel Chat/Settings migration.
- Refactored Design window config and persisted migration behavior for old stored layouts.
- Refactored Sketchpad panel routing so Chat/Settings render at left side panel position.
- Reopened after requirement correction and moved Chat/Settings placement to right side panel position.
- Reopened again to remove property-panel tab access for Chat/Settings and remove tab strip from single-tab property panels.
- Ran tests and compile checks; recorded unrelated existing failures.

## Todos
- Verify UI manually in sketchpad Design route: property panel has no tabs, and Chat/Settings open only from navbar toggles.
- Add/update e2e assertions once Playwright webServer startup issue is fixed.

## Plan
- Remove Chat/Settings from Design scene window layout.
- Register Chat/Settings as right side panel tabs in Design app.
- Route navbar Chat/Settings toggles to right side panel presentation path.
- Remove Chat/Settings from property-panel tab list and hide single-tab side panel tab strip.
- Run unit/compile validations and record results.
