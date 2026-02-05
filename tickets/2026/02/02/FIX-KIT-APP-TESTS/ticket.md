# Ticket: Fix Kit App Tests

## Status
Finished

## Prompt
get the kit app test to comply. dont remove any funtionality from the test. Change/refactor/extend whatever is necessary to get it working. Even if it seems unrelated to you. The goal is clear.

## Todos
- [x] Fix `test-selection-tools-simple.spec.ts` `require` usage.
- [x] Diagnose browser launch issue.
- [x] Verify fix.
- [x] Fix click event propagation in `elements.tsx`.
- [x] Stabilize `selection-tools.spec.ts` with correct selectors.

## Log
- [2026-02-02] Created plan.
- [2026-02-02] Identified navigation timeout caused by click event bubbling in Toggle component.
- [2026-02-02] Fixed `js/semio/sketchpad/elements.tsx` by adding `stopPropagation` to Action button wrapper.
- [2026-02-02] Updated `js/semio/playwright/kit/selection-tools.spec.ts` to use ID attribute selectors to avoid escaping issues and allow sufficient wait time.
- [2026-02-02] Verified all tests in `playwright/kit` pass.

## Summary
Fixed navigation timeout in kit selection tests by resolving an event bubbling issue in the Toggle component's Action button. Verified that clicking the Create button now properly triggers navigation instead of toggling the parent. Updated tests to use robust selectors and confirmed all kit app tests pass.
