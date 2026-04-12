---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Revalidated the restored Type toolbar tool-sync fix and confirmed the matching Type Playwright coverage passes against a live sketchpad dev server.
## Changes

- `semio/js/sketchpad/Type.tsx`: selecting the type-app selection toolbar group now forces `selection-normal` when the current tool is not a selection mode, and the create toolbar group now forces `connector` while it is active.
- `semio/js/sketchpad.test.ts`: extended the existing `Type` Playwright flow to assert the type-app `activeTool` switches to `selection-normal` for the selection group and `connector` for the create group.
- Restored the `semio/js/sketchpad/Type.tsx` runtime logic after it disappeared while the Playwright assertions remained in place.

## Log

- Confirmed the regression is in the type-app toolbar settings flow: selecting the selection toolbar group only opened settings and did not move `activeTool` away from connector mode.
- Applied the toolbar-driven tool sync directly in the type-app settings components instead of relying on the nested toggle state alone.
- Ran `npx playwright test sketchpad.test.ts --grep "Type" --timeout 240000 --workers=1 --max-failures=1 --reporter=list`; the run failed before reaching the new assertions because of an existing unrelated timing assertion (`pan1Duration < 150ms`, observed `187ms`).
- Reopened the ticket after the `Type.tsx` behavior was overwritten: the test assertions were still present, but the selection/connector mount effects had been removed.
- Revalidated the current worktree and confirmed the `Type.tsx` toolbar mount effects are restored while the `Type` Playwright assertions remain in place.
- Ran `PLAYWRIGHT_SKIP_WEBSERVER=1 npx playwright test sketchpad.test.ts --grep "Type" --timeout 240000 --workers=1 --max-failures=1 --reporter=list` against a live `npm run dev:sketchpad` server on March 4, 2026; both matching tests passed, including the selection-group `selection-normal` and create-group `connector` assertions.

## Todos

- None.

## Plan

- Inspect the type-app toolbar settings components and current toolbar group behavior.
- Patch the selection and connector settings mount behavior so toolbar group changes drive the intended tool.
- Add a regression assertion in the existing Playwright file and run the targeted test coverage that is feasible here.
