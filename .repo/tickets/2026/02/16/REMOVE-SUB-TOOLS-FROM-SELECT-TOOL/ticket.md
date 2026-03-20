---
goal: FEATURES/TYPE-APP
---

# Ticket

## Summary

Removed intersect and lasso from Sketchpad tool settings bars across apps and updated e2e coverage.
## Changes
- Updated selection settings UI in:
- `semio/js/sketchpad/Design.tsx`
- `semio/js/sketchpad/Kit.tsx`
- `semio/js/sketchpad/Type.tsx`
- `semio/js/sketchpad/Quality.tsx`
- Updated Playwright coverage in:
- `semio/js/sketchpad.test.ts`

## Log
- 2026-02-17: Located existing open ticket and continued work in this ticket.
- 2026-02-17: Removed `intersect` toggle from Design, Kit, Type, and Quality selection settings components.
- 2026-02-17: Removed `lasso` toggle from Design and Kit selection settings components.
- 2026-02-17: Updated e2e assertions to verify removed controls are no longer visible.
- 2026-02-17: Ran `npm run test:e2e -- sketchpad.test.ts` in `semio/js`; result: 7 passed.

## Todos
- [x] Locate all app-level selection settings bars in Sketchpad.
- [x] Remove `intersect` from tool settings bar everywhere exposed.
- [x] Remove `lasso` from tool settings bar everywhere exposed.
- [x] Update existing test file expectations.
- [x] Run tests and confirm pass/fail.

## Plan
- Scope: remove UI settings toggles only; keep tool kinds and behavior internals for later implementation.
- Validation: run `semio/js/sketchpad.test.ts` and confirm no assertion still expects removed toggles.
