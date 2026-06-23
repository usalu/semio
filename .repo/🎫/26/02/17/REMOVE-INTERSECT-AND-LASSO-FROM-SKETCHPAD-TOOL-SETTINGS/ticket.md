---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Removed intersect and lasso controls from sketchpad tool settings across Design, Kit, Type, and Quality apps; removed Type lasso toolbar group; updated sketchpad Playwright expectations; verified sketchpad e2e suite passes (7/7).
## Changes
- Removed `intersect` and freeform `lasso` toggles from Design selection settings in `compose/js/sketchpad/Design.tsx`.
- Removed `intersect` and freeform `lasso` toggles from Kit selection settings in `compose/js/sketchpad/Kit.tsx`.
- Removed `intersect` toggle from Type selection settings and removed Type lasso toolbar group registration in `compose/js/sketchpad/Type.tsx`.
- Removed `intersect` toggle from Quality selection settings in `compose/js/sketchpad/Quality.tsx`.
- Updated Playwright assertions in `compose/js/sketchpad.test.ts` to verify removed controls are absent and retained controls still render.

## Log
- Opened ticket and mapped sketchpad toolbar configuration points in `Design.tsx`, `Kit.tsx`, `Type.tsx`, `Quality.tsx`, and `sketchpad.test.ts`.
- Implemented UI removals for intersect/lasso settings controls across Sketchpad apps.
- Updated test expectations for hidden controls and retained tool settings controls.
- Executed `npm run test:e2e -- sketchpad.test.ts` in `compose/js`; result: `7 passed`.

## Todos
- [x] Remove `intersect` toggle from selection settings where present.
- [x] Remove `lasso` toggle(s)/group from tool settings bar where present.
- [x] Update Playwright coverage in `compose/js/sketchpad.test.ts`.
- [x] Run sketchpad tests.
- [x] Close ticket with updated summary and touched files.

## Plan
1. Patch toolbar setting components in `Design.tsx`, `Kit.tsx`, `Type.tsx`, and `Quality.tsx` to remove `intersect` and `lasso` settings entries.
2. Remove the Type app lasso toolbar section registration so no lasso tool group appears in the toolbar.
3. Update `compose/js/sketchpad.test.ts` to assert these controls are absent and preserve existing checks for retained controls.
4. Run the relevant sketchpad test command and record outcome.
