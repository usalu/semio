# Fix Parent Connection Section Visibility

## Status: OPEN

## Goal: SKETCHPAD/DETAIL-PANEL

## Description
Fix the parent connection section not appearing in the piece detail panel when a child piece is selected. The bidirectional connection lookup was already implemented but the React component is not rendering the parentConnection TreeItem.

## Plan
1. Debug why PiecesSectionForm's `parentConnection` variable is null at render time despite metadata having parentPieceId
2. Check if console DEBUG logs fire from Design.tsx when piece is selected
3. Identify root cause (likely: metadata key mismatch OR usePiecesMetadataMap reactivity issue)
4. Fix the root cause
5. Verify parent connection section appears in browser
6. Run e2e tests

## TODOs
- [ ] Check browser console for DEBUG logs from PiecesSectionForm
- [ ] Verify metadata key format matches getPieceId output
- [ ] Identify and fix root cause
- [ ] Verify fix in browser
- [ ] Run e2e tests
- [ ] Clean up DEBUG logs
- [ ] Close ticket

## Changes
(tracking changes as they happen)

## Summary
(to be filled on close)
