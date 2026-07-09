---
goal: R26-02/RUNNING-SKETCHPAD
---

# Ticket

## Summary

Fixed flaky URL-toggle race conditions in sketchpad Playwright tests. Replaced fixed waitForTimeout(500) with waitForURL() predicates for Home temporary-kind toggle and Kit designs-kind toggle. All 6 tests (Home, Kit, Type, Design, Docs, Feedback) now pass consistently across 3 consecutive runs.

## Changes

1. **DONE** `compose/js/sketchpad.test.ts`: Replaced fixed `waitForTimeout(500)` with `waitForURL()` predicates for Home temporary-kind toggle and Kit designs-kind toggle, eliminating race conditions where URL hadn't updated before assertions.

## Log

- Initial analysis: All 6 tests were reported failing, but on first run 5/6 passed and Kit failed intermittently
- Root cause: Toggle click → URL update race condition. The 500ms fixed wait was insufficient when the browser was under load
- Fix: Used `page.waitForURL()` with regex/predicate to wait for URL to actually change before asserting
- Verified: 3 consecutive full runs, all 6 tests passing each time (~2.6m per run)

## Todos

- [x] Analyze test failures
- [x] Fix Home toggle race condition
- [x] Fix Kit toggle race condition
- [x] Verify all 6 tests pass (3 consecutive runs)

## Plan

1. For Home/Kit: Add click on `toolbar.group.filter` to expand before checking toggles
2. For Design: Fix the HUD tree items check - either add role="treeitem" to TreeItem component or update the selector
3. For Feedback: Debug send button visibility and fix
