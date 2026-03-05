---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed the navbar chat hook-order crash by moving side-panel tab callbacks back into component render boundaries and adding regression coverage for the Design chat toggle.
## Plan

1. Isolate the hook-order violation in the shared side-panel rendering path used by navbar chat.
2. Refactor shared tab rendering so tab callbacks render as React components instead of being invoked inline.
3. Extend the existing panel coverage to exercise the Design navbar chat toggle and catch hook-order/runtime errors.
4. Run the affected validation checks and record environment blockers.

## Todos

- [x] Isolate the `SidePanel` hook-order failure path triggered by the navbar chat toggle
- [x] Refactor shared side-panel rendering to preserve hook ownership inside tab content
- [x] Remove direct nested utility-tab callback invocation in the shared sketchpad shell
- [x] Extend the existing `sketchpad.test.ts` `Panels` coverage for the Design navbar chat toggle
- [x] Run `npx tsc -p semio/js/tsconfig.json --noEmit`
- [x] Attempt `npx playwright test sketchpad.test.ts --grep "Panels" --reporter=line`

## Changes
- Updated `semio/js/sketchpad/elements.tsx`:
  - Added `SidePanelTabContent` and changed `SidePanel` to render function tab content as a React component instead of calling `activeTab.content()` inline.
- Updated `semio/js/sketchpad/Sketchpad.tsx`:
  - Added `SidePanelTabSlot` to safely render registered side-panel tabs or fallback content.
  - Added memoized lookup for utility `chat` and `settings` side-panel tabs.
  - Replaced direct `chatTab.content()` and `settingsTab.content()` invocation in the navbar utility panel path with `SidePanelTabSlot`.
- Updated `semio/js/sketchpad.test.ts`:
  - Extended `initConsole` to capture `pageerror` events into the existing error collection.
  - Added a Design `Panels` assertion path that clicks the navbar chat toggle and fails on hook-order runtime errors.

## Log
- Reopened on 2026-02-27 to finish the migration: `Docs`, `Kit`, `Type`, and `Quality` still exposed `settings` and `chat` as window tabs, and `Design` still carried legacy tab-migration references.
- Reopened again on 2026-02-27 to remove chat/settings from the details panel only.
- Reopened again on 2026-02-27 to fix the navbar chat runtime crash caused by inline side-panel tab callback invocation in the shared renderer.
- Validation:
  - `npx tsc -p semio/js/tsconfig.json --noEmit` passed.
  - `npx playwright test semio/js/sketchpad.test.ts --grep "Panels" --reporter=line` failed before assertions because `page.goto("/")` had no valid base URL from the repo root.
  - `npx playwright test sketchpad.test.ts --grep "Panels" --reporter=line` from `semio/js` failed with the same existing `page.goto("/")` invalid URL issue before the new Design chat assertions ran.
