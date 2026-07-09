---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Reset lazy input state on field identity changes so creating a new Type cannot inherit a previously focused reference value, and added a passing Playwright regression.

## Changes

- Updated `compose/js/sketchpad/elements.tsx` so lazy `Input` fields immediately reset `isEditing` and resync their local value when the input `id` changes.
- Extended `compose/js/sketchpad.test.ts` with a regression that keeps a lazy resource-reference input focused, creates a new Type, and verifies the new Type name input shows the created name instead of the previously focused value.

## Log

- Traced Type creation paths in `Sketchpad.tsx` and `Kit.tsx`; both already use the default Type name label.
- Narrowed the regression to the shared lazy `Input` control in `elements.tsx`, which preserves local edit state across reconciled field identities.
- Planned a regression test in `compose/js/sketchpad.test.ts` that keeps a model URL input focused, creates a new Type through the store, and verifies the new Type name stays on the generated default.
- Ran `npx playwright test sketchpad.test.ts --grep "Type Create Keeps New Name Instead Of Focused Model Value" --timeout 240000 --workers=1 --max-failures=1 --reporter=list` in `compose/js`; the new regression passed.

## Todos

- [x] Reset lazy input editing state when the input id changes.
- [x] Add Playwright coverage in `compose/js/sketchpad.test.ts`.
- [x] Run the targeted regression test.

## Plan

1. Patch `compose/js/sketchpad/elements.tsx` so a lazy input immediately resynchronizes when React reuses the component for a different field id.
2. Extend `compose/js/sketchpad.test.ts` with a focused regression that reproduces the stale-value carryover path.
3. Run only the new regression test and capture the result here before closing the ticket.
