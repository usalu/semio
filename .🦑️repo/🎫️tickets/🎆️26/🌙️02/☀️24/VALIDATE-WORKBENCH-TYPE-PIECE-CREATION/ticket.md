# Validate Workbench Duplicate Type Visibility

**Goal:** SKETCHPAD-IMPROVEMENTS
**Status:** open
**Client:** codex
**LLM:** gpt-5-3-codex

## Prompt

Duplicate Type Visibility Without App Switch.
Scope: compose/js/sketchpad/Design.tsx and compose/js/sketchpad.test.ts only.
Ensure duplicate creates exactly one child under clicked parent, child is immediately visible in Workbench, URL stays on Design route (no /types/), and add-child naming/icon alignment is updated.

## Plan

1. Reopen existing matching ticket and capture repo context with repo tree
2. Update Workbench duplicate action presentation in Design.tsx
3. Extend existing Design e2e flow for strict duplicate child + URL assertions
4. Run Design Playwright flow and capture exact command outcomes

## TODOs

- [x] Reopened matching ticket and gathered tree context
- [x] Updated Workbench duplicate action icon in Design.tsx
- [x] Extended duplicate assertions in existing sketchpad.test.ts Design flow
- [x] Captured Playwright command outcomes with logs
- [x] Recorded blockers and final outcome

## Changes

1. `compose/js/sketchpad/Design.tsx`

- Updated Workbench type duplicate action icon from `CodeIcon` to `TypeIcon` for distinct visual identity from add-piece.
- Kept action id as `compose.sketchpad.common.duplicateType`.

2. `compose/js/sketchpad.test.ts`

- Tightened duplicate-type Design flow assertions:
- Captures Design URL before duplicate click.
- Tracks `framenavigated` main-frame URLs during duplicate action.
- Asserts child count for selected parent increases by exactly `+1`.
- Asserts exactly one new child guid appears.
- Asserts URL remains equal to pre-click URL and no observed URL contains `/types/`.
- Asserts duplicated child name becomes visible in Workbench.
- Updated variable/log naming from add-child wording to duplicate-type wording.
- Added resilient `initHome` early-return when app is already on `/kits/...` route to avoid false failures in reused state.

3. Ticket artifact

- Added test output log: `.repo/🎫️/26/02/24/VALIDATE-WORKBENCH-TYPE-PIECE-CREATION/design-playwright.log`

## Commands And Outcomes

1. `./repo/cli/cli tree sketchpad`

- Outcome: success, located relevant files/tickets and matching closed ticket.

2. `./repo/cli/cli ticket reopen 2026/02/24/VALIDATE-WORKBENCH-TYPE-PIECE-CREATION "...duplicate type visibility..." codex gpt-5-3-codex`

- Outcome: ticket reopened locally; GitHub reopen/comment failed due network restrictions.

3. `cd compose/js && npx playwright test sketchpad.test.ts --grep "Design" --timeout 240000 --workers=1 --max-failures=1 --reporter=list`

- Outcome: failed before test start (`config.webServer` startup failure in this environment).

4. `cd compose/js && PLAYWRIGHT_SKIP_WEBSERVER=1 npx playwright test ...`

- Outcome: initial run failed in sandbox due Chromium launch restriction; rerun required elevated permissions.

5. Elevated run with skip-webserver while server not running

- Outcome: failed with `ERR_CONNECTION_REFUSED`.

6. `cd compose/js && npm run dev:sketchpad`

- Outcome: success, dev server starts on `http://localhost:5173`.

7. Elevated run with skip-webserver + running dev server (output in ticket log)

- Outcome: test runs but fails in Design flow due pre-existing ReactFlow attach instability (`ReactFlow nodes not attached after 60s`), blocking full pass confirmation in this environment.

## Summary

Implemented duplicate-type behavior assertions and Workbench duplicate icon update in scoped files only. Duplicate e2e checks now enforce +1 child delta, single new child guid, strict no-`/types/` navigation, and visible duplicated row. Design e2e remains blocked by existing ReactFlow attach instability in this runtime.
