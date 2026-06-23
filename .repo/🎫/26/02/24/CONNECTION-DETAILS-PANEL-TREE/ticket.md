# Connection Details Panel Tree Structure

## Status: OPEN

## Goal: SKETCHPAD/DESIGN-APP

## Prompt
Implement connection details panel in sketchpad design app showing connecting/connected/plane/diagram tree when a connection is selected.

## Plan
1. Read current Design.tsx Details panel implementation
2. Read Connection data model from compose.ts and Sketchpad.tsx
3. Implement connection details tree structure:
   - Connecting (piece id, port id)
   - Connected (piece id, port id)
   - Plane: Translation (Gap, Shift, Rise), Orientation (Rotation, Inversion)
   - Diagram (Gap, Shift, Rise, X Offset, Y Offset)
4. Add i18n translations
5. Verify runtime behavior

## TODOs
- [ ] Gather context on current details panel
- [ ] Read connection model
- [ ] Implement connection tree in details panel
- [ ] Add i18n translations
- [ ] Verify implementation

## Changes
(tracked below)

## Summary
(filled on close)
