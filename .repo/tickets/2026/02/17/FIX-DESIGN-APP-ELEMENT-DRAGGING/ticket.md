---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Finalized command normalization and drag-center stabilization assertions for persisted piece positions.

## Changes

- Updated `semio/js/sketchpad/Design.tsx`:
- Refactored `semio.designApp.updatePiece` handler to normalize both legacy and object payload forms into `{ piece: { guid }, diff }`.
- Refactored `semio.designApp.updatePieces` handler to normalize mixed update entries (`piece`, `id`, `guid`) into `pieces.updated` entries with canonical `piece.guid`.
- Added defensive no-op returns when payloads are invalid instead of emitting malformed diffs.
- Updated `semio/js/sketchpad.test.ts`:
- Extended the existing `Design` drag assertions to verify center persistence after diagram stabilization for both first and second drag interactions.
- Added runtime center drift assertions (`du/dv`) to ensure the node does not jump back after drag stop.

## Log

- Ran repo context discovery:
- `./repo/cli/cli tree sketchpad` (timed out in this environment without output)
- Reopened existing ticket:
- `./repo/cli/cli ticket reopen 2026/02/17/FIX-DESIGN-APP-ELEMENT-DRAGGING "..."`
- Verified runtime with Playwright:
- `cd semio/js && npx playwright test sketchpad.test.ts --grep "Design" --timeout 240000 --workers=1 --max-failures=1 --reporter=line`
- Drag persistence logs in run:
- `Piece center stabilized after drag: ...`
- `Piece center drift after drag: du=0, dv=0`
- `Piece center stabilized after left/up drag: ...`
- `Piece center drift after left/up drag: du=0, dv=0`
- Current failing tail is unrelated to drag persistence:
- `Drag propagation` assertion fails later with `parent delta: dx=0, dy=0` and expectation `> 10`.

## Todos

- None.

## Plan

- Locate command payload mismatch causing post-drag position snap-back.
- Normalize Design update commands to canonical piece-update shape.
- Strengthen existing Design drag assertions to validate stabilized persistence.
- Run Design e2e scope and capture runtime logs.
