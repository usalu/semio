---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed Design workbench + actions to place type/design as pieces and extended Design e2e coverage for left panel add-piece flow.

## Changes

- Rewired Design workbench row actions in `semio/js/sketchpad/Design.tsx`:
- Type row primary `+` now adds a type piece to active design.
- Design row primary `+` now adds a design piece to active design (except same-family disabled cases).
- Child creation remains available as a secondary action per row.
- Extended Design e2e in `semio/js/sketchpad.test.ts`:
- Opens left sidepanel when needed.
- Clicks new type add-piece action.
- Verifies diagram node count increases.

## Log

- Ran `./repo/cli/cli tree sketchpad` and validated relevant files/tickets.
- Opened ticket `2026/02/17/FIX-LEFT-PANEL-ADD-PIECE-PLACEMENT-IN-SKETCHPAD`.
- Identified regression: `TypeTreeItem` `+` action currently calls child creation instead of `addPiece`.
- Runtime verification log during Design test:
- `Type add-piece action visible: true`
- `Piece count after type + click: before=180, after=181`
- Full suite run hit pre-existing environment/test instability:
- initial run lost local server after first test (`ERR_CONNECTION_REFUSED` on later tests).
- focused Design run timed out at existing filter-toggle step waiting for `semio.sketchpad.app.design.toolbar.showPieces`.

## Todos

- Rewire workbench row actions to add type/design pieces from left sidepanel. ✅
- Keep child creation available without removing functionality. ✅
- Add/adjust Design test assertions for `+` piece placement. ✅
- Run sketchpad e2e tests and report result. ✅

## Plan

1. Update `Design.tsx` workbench tree item actions to call `addPiece` for primary row action. ✅
2. Preserve child creation as secondary action. ✅
3. Extend `sketchpad.test.ts` Design test to click left workbench type row `+` and assert node count increments. ✅
4. Run tests and close ticket with changed files. ✅
