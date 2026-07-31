---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed second filter regression by restoring cross-root shared filter synchronization and strict off-state diagram filtering in Design app. Re-verified with passing Design Playwright test.

## Changes

- `compose/js/sketchpad/Design.tsx`: Fixed `toggleFilter()` to clear all filter params when all 3 kinds are re-enabled
- `compose/js/sketchpad.test.ts`: Fixed HUD panel toggle assertions, improved drag test, added filter coverage, fixed Panels test timeout

## Log

- Identified filter toggle bug: re-enabling all 3 filters didn't clear URL params
- Fixed `toggleFilter()` in `DesignToolbarFilters` component
- Fixed stale HUD panel toggle expectations (Design app now has STATS panel)
- Improved diagram drag test reliability
- All 7 sketchpad tests pass

## Todos

- [x] Check current test file state
- [x] Run tests and identify failures
- [x] Fix filter toggle logic bug
- [x] Run all tests and verify (7/7 passed)
- [x] Close ticket

## Plan

1. Read Design.tsx filter implementation
2. Run existing tests to identify failures
3. Fix filter toggle logic (clear params when all re-enabled)
4. Fix test assertions for HUD panel toggle
5. Fix drag test assertion reliability
6. Verify all 7 tests pass

## Reopen 2026-02-23

### Summary

Reopened to complete filter runtime verification for the Design app and extend existing test coverage in-place.

### Plan

1. Validate current Design filter runtime behavior in diagram/model views.
2. Fix Design filter behavior where runtime visibility does not track toggles.
3. Extend `compose/js/sketchpad.test.ts` filter checks to assert actual visible elements in diagram/model.
4. Re-run Design Playwright scope and confirm pass.

### Todos

- [x] Reopen existing ticket for matching scope
- [x] Gather code/test context and current behavior
- [x] Patch Design app filter runtime behavior
- [x] Extend existing Design test filter assertions
- [x] Run Design test and verify pass

### Changes

- Refactored Design filter state to a shared in-module snapshot store (`useSyncExternalStore`) to synchronize filter visibility across isolated Design window roots.
- Kept URL query persistence (`filter=`) as source for toolbar state while propagating updates into the shared filter snapshot.
- Wrapped `DiagramWindow` and `SceneWindow` with `DesignFilterProvider` so filter state applies in all Design windows.
- Strengthened existing Design filter test assertions to verify visible diagram nodes/edges/ports actually hide/show, not just toggle state and URL params.

### Verification

- `npx playwright test sketchpad.test.ts --grep Design --timeout 240000 --workers=1 --max-failures=1 --reporter=list`
- Result: `1 passed`

## Reopen 2026-02-23 (Second Regression)

### Summary

Current branch state regressed filter synchronization in `Design.tsx`; disabled filter toggles no longer consistently hide elements.

### Plan

1. Restore cross-root filter synchronization.
2. Restore strict node/edge removal for off-state filtering.
3. Re-run existing Design Playwright verification.

### Todos

- [x] Reopen ticket
- [x] Reproduce/confirm regression in current code
- [x] Re-apply filter synchronization and strict visibility behavior
- [x] Re-verify Design Playwright test

### Changes

- Re-introduced shared Design filter infrastructure in `compose/js/sketchpad/Design.tsx`:
  - global singleton filter store on `globalThis`
  - `useSyncExternalStore` subscription in `DesignFilterProvider`
  - URL-to-store and toolbar toggle-to-store synchronization
- Re-wired `DiagramWindow` and `SceneWindow` with `DesignFilterProvider`.
- Reinstated strict diagram filtering behavior:
  - nodes removed from render when pieces are disabled
  - edges removed when pieces or connections are disabled

### Verification

- `npx playwright test sketchpad.test.ts --grep Design --timeout 240000 --workers=1 --max-failures=1 --reporter=list`
- Result: `1 passed`

## Reopen 2026-02-23 (Visibility Mismatch Follow-Up)

### Summary

Fixed remaining mismatch where UI filter off-state did not consistently drive visibility in the active diagram window across isolated render roots.

### Plan

1. Reproduce mismatch in existing Design test scope.
2. Rework filter synchronization so all Design roots observe the same state.
3. Enforce strict off-state rendering behavior for filtered diagram entities.
4. Extend existing assertions in `sketchpad.test.ts` and re-verify.

### Todos

- [x] Reopen ticket for follow-up filter mismatch
- [x] Reproduce mismatch in Design test
- [x] Implement shared global filter store for cross-root sync
- [x] Apply strict diagram filtering when toggles are off
- [x] Extend existing Design test assertions
- [x] Verify with Design Playwright run

### Changes

- `compose/js/sketchpad/Design.tsx`
  - Added global singleton filter store on `globalThis` and `useSyncExternalStore` subscription.
  - Synced toolbar URL filters to shared state and propagated updates to all Design roots.
  - Wrapped `DiagramWindow` and `SceneWindow` with `DesignFilterProvider`.
  - Changed diagram filtering to remove nodes/edges from render arrays when off (instead of hidden flags), and tied edges visibility to pieces+connections.
- `compose/js/sketchpad.test.ts`
  - Scoped diagram locator to `#diagram .react-flow`.
  - Added assertion that edges are also hidden when pieces are toggled off.

### Verification

- `npx playwright test sketchpad.test.ts --grep Design --timeout 240000 --workers=1 --max-failures=1 --reporter=list`
- Result: `1 passed`
