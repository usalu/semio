# Ticket

## Todos
- [x] Refresh toolbar tree context with `repo tree toolbar`.
- [x] Reopen existing toolbar subtools ticket for this request.
- [x] Refactor toolbar subtool handling to infrastructure-level group-agnostic logic.
- [x] Preserve existing visible behavior for current apps.
- [x] Add i18n keys for future Settings subtools (`App Settings`, `Command`, `Tools`).
- [x] Update sketchpad specs documentation for the new mechanism.
- [x] Run targeted verification commands.

## Changes
- Updated `compose/js/sketchpad/Sketchpad.tsx` to make toolbar subtool mechanics group-agnostic while preserving current UI behavior:
  - Added infrastructure-level toolbar group ordering support including future `settings` group.
  - Added generic `groupSubTools` and `groupHasDropdown` derivation from section metadata.
  - Generalized toolbar button rendering so any group can become a dropdown when configured.
  - Preserved current behavior by keeping legacy `selection` dropdown rule (`selection` dropdown remains as before) and requiring multiple subtools for other groups.
  - Generalized settings-toolbar section filtering by active subtool for whichever group is active.
  - Added `SettingsIcon` mapping for future `settings` parent tool.
- Updated `compose/js/sketchpad/locales/en.json`:
  - Added `compose.sketchpad.toolbar.parent.settings`.
  - Added `compose.sketchpad.toolbar.subtool.appSettings`, `command`, `tools`.
- Updated `compose/js/sketchpad/locales/de.json`:
  - Added toolbar parent settings label and matching subtool labels for `appSettings`, `command`, `tools`.
- Updated `compose/js/sketchpad/README.md` Specs -> Toolbar:
  - Documented that subtool dropdown infrastructure is group-agnostic while preserving current selection behavior unless additional groups define multiple subtools.

## Log
### Key Decisions
- No app section registrations were added for `settings` yet, so no new visible toolbar items are introduced by this change.
- Infrastructure now supports future per-tool subtools uniformly without forcing current non-selection groups into dropdown UI.
- Active subtool filtering in the right toolbar is now generic and no longer hardcoded to `selection`.

### Verification
- `node -e "JSON.parse(...)"` validation passed for `compose/js/sketchpad/locales/en.json` and `compose/js/sketchpad/locales/de.json`.
- `npx playwright test --config compose/js/playwright.config.ts sketchpad.test.ts --grep "Toolbar"` failed before test execution because `config.webServer` could not start in this environment.
- `npx tsc -p compose/js/tsconfig.json --noEmit` reports existing unrelated baseline errors in `Design.tsx`, `Feedback.tsx`, `Type.tsx`, and `elements.tsx`; no additional parse/runtime errors were introduced by locale JSON edits.

## Summary

Implemented group-agnostic toolbar subtool infrastructure without changing current UI, and added Settings subtool i18n scaffolding for App Settings, Command, and Tools.
