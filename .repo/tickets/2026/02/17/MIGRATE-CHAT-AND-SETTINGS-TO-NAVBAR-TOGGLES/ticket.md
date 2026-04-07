---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Restored functional Design navbar utility settings/chat panels by adding Design utility tab registration and shell-level utility fallbacks; added focused Playwright coverage for navbar settings availability.
## Plan

1. Reproduce the navbar settings unavailability in Design and trace the utility tab resolution path.
2. Restore Design-side utility registration for settings/chat tabs.
3. Add shell-level utility fallback rendering for settings/chat so navbar utility toggles remain functional even when app-specific tabs are missing.
4. Extend the existing sketchpad Playwright coverage for navbar settings availability.
5. Validate with TypeScript and focused Playwright coverage.

## Todos

- [x] Reproduce Design navbar settings unavailable state
- [x] Add Design right-side utility tabs for `settings` and `chat`
- [x] Add shell-level utility fallback content for `settings` and `chat` in `Sketchpad.tsx`
- [x] Add Design utility settings content with panel visibility controls
- [x] Extend existing Playwright spec to validate Design navbar settings utility path
- [x] Run `npx tsc -p semio/js/tsconfig.json --noEmit`
- [x] Run `npx playwright test semio/js/sketchpad.test.ts --grep "Design Navbar Settings Is Available" --reporter=line`

## Changes
- Updated `semio/js/sketchpad/Design.tsx`:
  - Added Design-specific right-side utility tab registration for `semio.sketchpad.app.design.settings` and `semio.sketchpad.app.design.chat`.
  - Added `DesignSettingsContent` with toolbar/workbench/windows/details visibility toggles.
  - Added required imports for chat/settings icons, utility tab hooks, chat panel, and tree wrappers.
- Updated `semio/js/sketchpad/Sketchpad.tsx`:
  - Added `UtilityFallbackSettingsContent` for navbar utility settings when no app-provided settings tab exists.
  - Added navbar chat fallback rendering with `BasicChatPanel` when no app-provided chat tab exists.
  - Removed hardcoded “Settings not available” / “Chat not available” fallback messages.
- Updated `semio/js/sketchpad.test.ts`:
  - Replaced the Design utility removal assertion with `Design Navbar Settings Is Available`.
  - Added assertions for navbar settings availability and no fallback utility-unavailable text when toggling settings/chat.

## Log
- Reopened on 2026-02-27 to finish the migration: `Docs`, `Kit`, `Type`, and `Quality` still exposed `settings` and `chat` as window tabs, and `Design` still carried legacy tab-migration references.
- Reopened again on 2026-02-27 to remove chat/settings from the details panel only.
- Reopened again on 2026-02-27 to fix the navbar chat runtime crash caused by inline side-panel tab callback invocation in the shared renderer.
- Reopened on 2026-03-11 for regression: Sketchpad navbar settings toggle showed unavailable fallback content in Design.
- Validation:
  - `npx tsc -p semio/js/tsconfig.json --noEmit` passed.
  - `npx playwright test semio/js/sketchpad.test.ts --grep "Design Navbar Settings Is Available" --reporter=line` passed with local `vite` server on `127.0.0.1:5173`.
