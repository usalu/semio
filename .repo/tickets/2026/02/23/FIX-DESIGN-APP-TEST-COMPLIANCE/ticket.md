---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed 2 TS compilation errors caused by invalid `id_` property in Piece object literals across 4 drag-drop handlers. Ensured dev server on port 5173 is running for Playwright test infrastructure. All 7 e2e tests pass, 13 unit tests pass, 0 TS errors.

## Changes

- `semio/js/sketchpad/Design.tsx`: Removed invalid `id_` property from 4 piece creation sites in drag-drop handlers (diagram drop, workbench drop, scene drop)

## Log

- Design test initially failed due to missing dev server on port 5173 (webServer config)
- Started dev:sketchpad on port 5173, test passed
- Found 2 TS errors: `id_` property not in Piece type schema but used in 4 addPiece calls
- Removed `id_: pieceGuid` from all 4 occurrences where Piece objects are created for addPiece
- Verified: 0 TS errors, 7/7 e2e pass, 13/13 unit tests pass

## Todos

- [x] Run Design test to identify failures
- [x] Fix dev server availability on port 5173
- [x] Fix TS compilation errors (id_ property)
- [x] Run full test suite verification
- [x] Close ticket

## Plan

1. Run Design test to see current failures
2. Fix issues found
3. Verify all tests pass
