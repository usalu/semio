---
goal: PERFORMANCE/SKETCHPAD
---

# Ticket

## Summary

Updated the existing Design Drag Performance test with hard render, zoom, drag, and long-task budgets. Focused validation now fails on the explicit 50ms long-task ceiling, exposing current regressions.
## Changes
- `semio/js/sketchpad.test.ts`: Replaced the old synthetic drag-only timing with explicit hard budgets for initial render, zoom interaction, drag interaction, and browser long tasks.

## Log
- Reused the existing open ticket because it already covers the sketchpad performance test.
- Located `semio/js/sketchpad.test.ts` and targeted the existing `Design Drag Performance` case for in-place hardening.
- Added browser-side long-task observation with `PerformanceObserver` before navigation so interaction work is measured in-page.
- Added explicit ceilings for current runtime: initial render `< 45000ms`, zoom interaction `< 2000ms`, drag interaction `< 20000ms`, and long tasks `<= 50ms`.
- Replaced the flaky background pan gesture with a deterministic zoom-in plus zoom-out viewport interaction while keeping the combined interaction budget.
- Focused Playwright validation required escalated permissions because Chromium failed under the default sandbox with `sandbox_host_linux.cc` permission errors.
- Latest focused run reached every assertion and failed on the long-task ceiling: initial render `42614ms`, pan or zoom `1404ms`, drag `11299ms`, max long task `3973ms` across `74` long-task entries.

## Todos
- [x] Harden the existing performance test with explicit budgets.
- [x] Run the focused Playwright validation for the updated assertions.
- [x] Record the observed regressions in this ticket.
- [x] Close the ticket with the touched files.

## Plan
- Add reusable helpers in `semio/js/sketchpad.test.ts` for browser long-task capture and viewport transform reads.
- Extend `Design Drag Performance` to assert concrete budgets for initial render, pan or zoom, drag, and long tasks.
- Execute the focused test, then update this ticket with the result and close it.
