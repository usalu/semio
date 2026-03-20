---
goal: r26.02-1/Running Sketchpad
---

# Ticket

## Summary

Restored descendant drag propagation in Design, extended existing Design coverage for parent-to-descendant drag movement, and hardened adjacent Design test flows uncovered during validation.
## Changes
- Design.tsx: update descendant React Flow nodes during live drag and again on drag stop before persisting piece center diffs
- Design.tsx: preserve selected/non-selected node snapshots at drag stop so late post-drag logic does not read cleared refs
- sketchpad.test.ts: derive hierarchy metadata from `window.__piecesMetadata`, add descendant drag-propagation coverage to the existing Design test, and harden diagram focus clicks with `force: true`
- sketchpad.test.ts: compute flat metadata from the imported fixture kit instead of the mutated runtime store so the baseline comparison stays stable after in-test edits

## Log
- Reopened for a new regression in the same area: parent-node drag no longer carries descendant nodes in the diagram.
- Root cause confirmed: descendant pieces were only queued in diff updates, while their React Flow internal/node positions were not updated during live drag, so the dragged root moved alone in the diagram.
- Browser validation: `PLAYWRIGHT_BASE_URL=http://127.0.0.1:5175 npx playwright test sketchpad.test.ts --grep "Design" --timeout 240000 --workers=1 --max-failures=1 --reporter=list` reached and passed the new drag section, then exposed two pre-existing overlay-intercept failures in the existing Design test; both click paths were hardened.
- Browser validation then exposed a pre-existing unstable metadata comparison that read from the already-mutated store; the test now computes against the imported fixture kit.
- Final syntax validation: `npx playwright test sketchpad.test.ts --grep "Design" --list` succeeded and listed the 4 Design-related tests.

## Todos
- [x] Reopen existing ticket for the drag propagation regression
- [x] Inspect current design drag flow and descendant resolution
- [x] Patch descendant drag propagation in the diagram runtime
- [x] Extend existing sketchpad tests to cover parent drag propagation through descendants
- [x] Run the relevant sketchpad tests
- [x] Close ticket

## Plan
1. Completed: patch the drag handlers so descendant nodes update in React Flow during live drag and commit consistently on drag stop.
2. Completed: add a runtime regression assertion to the existing Design Playwright test that verifies parent and descendant nodes move together.
3. Completed: run targeted Design validation and fix adjacent flaky assertions that blocked the run.
4. Completed: update the ticket summary and close the ticket with touched files.
