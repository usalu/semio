---
goal: R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS/RUNNING-DESIGN-APP
---

# Ticket

## Plan
1. Reproduce the Design crash path for two-piece selection and isolate the failing hook order in `PiecesSectionForm`.
2. Refactor `PiecesSectionForm` so all `useLabel` hooks are called unconditionally at top-level.
3. Extend existing Design Playwright coverage in `semio/js/sketchpad.test.ts` to multi-select two pieces and assert no hook-order runtime error.
4. Run validation (`npx tsc --noEmit` and Design Playwright flow).

## Todos
- [x] Isolate the hook-order failure path in `PiecesSectionForm`
- [x] Replace conditional/inline `useLabel(...)` usage in `PiecesSectionForm` with top-level resolved labels
- [x] Remove temporary `PiecesSectionForm` parent-connection debug logs
- [x] Extend existing `sketchpad.test.ts` Design test to perform multi-selection and assert no `Rendered fewer hooks than expected` error
- [x] Run `cd semio/js && npx tsc --noEmit`
- [ ] Run full `Design` Playwright test to completion in this environment

## Summary

Finalized PiecesSectionForm hook-order fix and Design multi-selection regression assertions; compile check passes.
## Changes
- `semio/js/sketchpad/Design.tsx`
  - Removed `useTranslation` from `PiecesSectionForm`.
  - Added top-level, unconditional label resolution for:
    - `mixedSelectionMessageLabel`
    - `mixedValuesLabel`
    - `selectTypeLabel`
    - `selectVariantLabel`
    - `pieceAttributeLabel`
    - `connectedPieceInfoLabel`
    - `fixPieceLabel`
  - Replaced all inline/conditional `useLabel(...)` usages in `PiecesSectionForm` render paths with the resolved variables.
  - Removed temporary parent-connection debug logs in `PiecesSectionForm`.
- `semio/js/sketchpad.test.ts`
  - Extended `initConsole` to collect `pageerror` messages.
  - In `Design` test, added two-piece multi-selection (shift-click) and assertions ensuring no `Rendered fewer hooks than expected` in console errors or page errors.

## Validation
- `cd semio/js && npx tsc --noEmit` ✅
- `cd semio/js && npx playwright test sketchpad.test.ts --grep "Design" --timeout 240000 --workers=1 --max-failures=1 --reporter=list` ⚠️
  - Initially blocked because Playwright web server could not start in sandbox (`tsx` IPC pipe permission).
  - Re-run with external dev server succeeded and reached Design flow, including piece selection.
  - The long existing Design flow did not complete in a reasonable window in this environment and was stopped manually; no hook-order error was observed before stop.

## Log
- Confirmed `useLabel` is a hook (`semio/js/i18n.ts`) and was called inline in conditional JSX branches in `PiecesSectionForm`.
- Refactored to fixed-count top-level hook calls to preserve hook order across single/multi/mixed selection branches.
- Added explicit multi-selection hook-order regression assertions in the existing Design test.
