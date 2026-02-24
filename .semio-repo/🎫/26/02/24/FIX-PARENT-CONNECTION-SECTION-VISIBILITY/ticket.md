# Fix Parent Connection Section Visibility

## Status: OPEN

## Goal: SKETCHPAD/DETAIL-PANEL

## Description
Fix sketchpad app crash when selecting connections. Runtime throws `Rendered fewer hooks than expected` in `PiecesSectionForm` due to conditional hook calls during selection-state transitions.

## Plan
1. Identify the hook-order violation in `PiecesSectionForm` triggered by selection changes.
2. Refactor label resolution in `PiecesSectionForm` to avoid conditional hook invocation.
3. Remove temporary debug logs from the parent-connection lookup path.
4. Extend existing sketchpad design e2e coverage for hook-order/PiecesSectionForm runtime errors.
5. Run Playwright Design test and record result.

## TODOs
- [x] Identify and fix root cause in `PiecesSectionForm`
- [x] Remove temporary DEBUG logs in `PiecesSectionForm`
- [x] Extend existing `sketchpad.test.ts` coverage for hook-order crash regressions
- [x] Run e2e test command for Design flow
- [ ] Get a full green Design e2e pass (current run times out in pre-existing workbench loop)
- [ ] Close ticket

## Changes
- Updated `semio/js/sketchpad/Design.tsx`:
  - Replaced conditional `useLabel(...)` usage inside `PiecesSectionForm` render branches with precomputed `t(...)` labels, keeping hook execution order stable across selection changes.
  - Removed temporary `[DEBUG] PiecesSectionForm ...` logs in parent connection lookup.
- Updated `semio/js/sketchpad.test.ts`:
  - Added regression assertions in Design flow to fail if browser console errors include `Rendered fewer hooks than expected` or `<PiecesSectionForm>`.
- Validation:
  - Executed: `cd semio/js && npx playwright test sketchpad.test.ts --grep "Design" --timeout 240000 --workers=1 --max-failures=1 --reporter=list`
  - Result: failed by timeout at `sketchpad.test.ts:2139` in existing workbench expansion loop, before reaching final Design assertions.

## Summary
Hook-order crash in `PiecesSectionForm` was addressed by removing conditional hook calls from render branches and replacing them with stable precomputed labels. Added e2e regression assertions for hook-order/PiecesSectionForm errors. Design e2e run currently times out in a pre-existing loop.
