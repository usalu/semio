---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed Design navbar disappearance by restoring missing SettingsIcon and ChatIcon imports in Design.tsx and validating navbar utility toggles with Playwright.

## Changes

- Updated `compose/js/sketchpad/Design.tsx` asset imports to include `SettingsIcon` and `ChatIcon`
- Kept existing Design side-panel tab registration intact (`compose.sketchpad.app.design.settings` and `compose.sketchpad.app.design.chat`)
- Revalidated Design navbar rendering and utility toggles after import fix

## Log

- Reproduced failure in Design route: navbar selectors absent and Design tests failing
- Verified runtime error with browser instrumentation: `ReferenceError: SettingsIcon is not defined` from `Design.tsx`
- Applied import fix for `SettingsIcon`, then re-ran and surfaced the next runtime error: `ReferenceError: ChatIcon is not defined`
- Applied import fix for `ChatIcon`
- Re-ran runtime reproduction script and confirmed on Design route: `navCount=1`, `settingsToggle=1`, `chatToggle=1`
- Ran `npx playwright test sketchpad.test.ts -g "Design Navbar Settings Is Available" --reporter=line` and confirmed pass (`1 passed`)

## Todos

- [x] Reproduce navbar disappearance in Design app
- [x] Capture concrete runtime failure cause
- [x] Fix Design app runtime failure without changing panel behavior
- [x] Validate navbar visibility and utility toggles on Design route
- [x] Run focused Playwright regression

## Plan

1. Reproduce the navbar disappearance from Home -> Kit -> Design path
2. Capture runtime errors and map to exact source location
3. Patch missing runtime dependencies in Design app registration code
4. Validate with runtime probe and focused Playwright test
