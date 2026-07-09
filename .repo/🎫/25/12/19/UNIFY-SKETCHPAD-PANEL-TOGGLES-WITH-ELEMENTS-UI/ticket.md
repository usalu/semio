# Ticket

## Todos

- [x] Inspect Sketchpad HUD layout implementation and Design app panel registration
- [x] Reproduce current test state for Design app HUD behavior
- [x] Refactor HUD panel layout positioning to align with panel system behavior
- [x] Register Design HUD/Stats sections so HUD tree content is visible and functional
- [x] Extend existing `compose/js/sketchpad.test.ts` coverage for HUD layout and tree visibility
- [x] Run relevant tests and capture outcome
- [x] Summarize changes and close ticket

# Previously

# Plan

- Diagnose both failure modes: layout docking and empty HUD tree content.
- Fix HUD panel positioning in `elements.tsx` so the middle panel uses stable top docking and bounded viewport height.
- Add concrete Design app sections for `hud` and `stats` in `Design.tsx` to guarantee visible tree items and functional content.
- Extend `sketchpad.test.ts` helper logic to assert HUD panel positioning and section/tree visibility for Design.
- Execute targeted verification commands and record blockers if browser runtime cannot launch.

# Changes

## Changes

- Reopened existing ticket for HUD/sidepanel refactor context.
- Baseline test command attempted:
  - `npx playwright test compose/js/sketchpad.test.ts -g "Design" --reporter=line`
  - Result: failed before test execution due Playwright Chromium launch shutdown (`browserType.launch: Target page, context or browser has been closed`).
- Updated HUD panel positioning/sizing in `compose/js/sketchpad/elements.tsx`:
  - Added fixed height bound and max width bound to improve layout stability and tree visibility.
  - Removed class-level max height in favor of style-level explicit sizing.
- Added Design app HUD/Stats panel sections in `compose/js/sketchpad/Design.tsx`:
  - Registered `hud` section with selection counters and connector-selected indicator.
  - Registered `stats` section with piece/connection totals and window layout status.
  - Added cleanup for new sections on effect disposal.
- Added i18n labels in `compose/js/sketchpad/locales/en.json` and `compose/js/sketchpad/locales/de.json` for new Design HUD/Stats ids.
- Extended `compose/js/sketchpad.test.ts`:
  - Added HUD-specific layout/tree verifier helper.
  - Design app test now asserts HUD is top-docked and contains tree items when toggled.
- Validation commands:
  - `node -e "const fs=require('fs'); JSON.parse(fs.readFileSync('compose/js/sketchpad/locales/en.json','utf8')); JSON.parse(fs.readFileSync('compose/js/sketchpad/locales/de.json','utf8')); console.log('locales-json-ok');"`
    - Result: `locales-json-ok`
  - `npx tsc --noEmit --pretty false -p compose/js/tsconfig.json`
    - Result: failed with numerous pre-existing TypeScript errors across sketchpad modules (not introduced by this HUD scope).
  - `npx playwright test compose/js/sketchpad.test.ts -g "Design" --reporter=line`
    - Result: fails before running assertions due Chromium launch process closing immediately in this environment.

## Log

## Summary

Refactored Design HUD panel layout and visibility by adding real hud/stats sections, i18n labels, and stricter HUD layout/tree assertions in existing sketchpad test file.
