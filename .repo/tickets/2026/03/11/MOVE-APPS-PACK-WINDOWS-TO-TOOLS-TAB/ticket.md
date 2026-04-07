---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Moved Design windows section from workbench to tools tab and extended existing Design Playwright assertions for tab-specific section placement; validated behavior via runtime logs with running dev server.

## Changes

- Updated `semio/js/sketchpad/Design.tsx` panel registration:
  - Kept `semio.sketchpad.app.kit.pieces` in `workbench`.
  - Moved `semio.sketchpad.app.design.windows` to `tools`.
  - Updated cleanup to `removeSection("tools", "semio.sketchpad.app.design.windows")`.
- Extended `semio/js/sketchpad.test.ts` inside the existing `Design` test:
  - Asserted workbench tab contains `semio.sketchpad.app.kit.pieces`.
  - Asserted workbench tab does not contain `semio.sketchpad.app.design.windows`.
  - Asserted tools tab contains `semio.sketchpad.app.design.windows`.
  - Asserted tools tab does not contain `semio.sketchpad.app.kit.pieces`.
  - Returned to workbench tab before existing drag assertions.

## Log

- Gathered sketchpad tree via repo CLI.
- Opened ticket `2026/03/11/MOVE-APPS-PACK-WINDOWS-TO-TOOLS-TAB`.
- Located section registration in `Design.tsx` and panel assertions in `sketchpad.test.ts`.
- Applied the panel section move and extended existing test coverage.
- Ran Playwright slice without dev server first; failed due to `ERR_CONNECTION_REFUSED` on `http://127.0.0.1:5173`.
- Started sketchpad dev server via `npm run dev:js:js:sketchpad`.
- Re-ran `npx playwright test semio/js/sketchpad.test.ts -g "Design|Panels"`.
- Observed new section-location assertions pass in logs:
  - `[Design] Workbench tab has pieces section: true`
  - `[Design] Workbench tab has windows section: false`
  - `[Design] Tools tab has windows section: true`
  - `[Design] Tools tab has pieces section: false`
- Test run still has unrelated existing failures in additive-toggle and long-task expectations.

## Todos

- Completed: Move windows section registration to tools tab.
- Completed: Extend existing test coverage for section placement.
- Completed: Run targeted Playwright validation and record outcomes.

## Plan

1. Completed: Update Design panel section registration and cleanup in `Design.tsx`.
2. Completed: Update existing panel test logic in `sketchpad.test.ts` for the new panel location.
3. Completed: Run targeted Playwright tests and document outcomes.
