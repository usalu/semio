---
goal: R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS/RUNNING-DESIGN-APP
---

# Ticket

## Plan

Root causes identified and fixed:
1. **onNodeDragStop blocking**: Synchronous processing of 180 nodes in `onNodeDragStop` blocked the main thread for >4 minutes, preventing Playwright mouse events from completing. Fixed by deferring all processing via `requestAnimationFrame`.
2. **onSelectionChange blocking**: `onSelectionChange` dispatched selection synchronously during `mousedown`, blocking before `onNodeDragStart` could set `isDraggingNodeRef`. Fixed by other agent via `setTimeout(0)` deferral.
3. **Design data loading race**: In sequential test runs, the design page's golden-layout + ReactFlow didn't render because the store hadn't finished populating pieces. Fixed by polling `__SEMIO_STORE__` in `initDesign` until pieces are loaded (up to 30s).
4. **Selection readback key mismatch**: Test read `designApps?.[designGuid]` but the actual key is `${kitGuid}:${designGuid}`. Fixed both readback locations.

## Todos
- [x] Fix onNodeDragStop → defer via requestAnimationFrame
- [x] Fix selection readback composite key in test
- [x] Fix design data loading race in initDesign
- [x] Verify all 6 playwright tests pass (2 consecutive runs: 3.0m, 3.2m)
- [x] Close ticket

## Summary

All 6 Playwright tests pass consistently. Fixed onNodeDragStop via rAF deferral, fixed selection readback composite key, and added store polling in initDesign for sequential test stability.
## Changes

- `semio/js/sketchpad/Design.tsx` - onNodeDragStop deferred via rAF, onSelectionChange deferred via setTimeout
- `semio/js/sketchpad.test.ts` - initDesign store polling, selection readback key fix
- `semio/play/globals.css` - CSS import fix (previous session)
- `semio/sketchpad/globals.css` - CSS import fix (previous session)
- `semio/play/package.json` - Removed duplicate "type" key (previous session)
- `semio/play/vite.config.ts` - Added @semio/assets alias (previous session)
- `semio/sketchpad/vite.config.ts` - Added @semio/assets alias (previous session)

## Log

- Identified onNodeDragStop as timeout source (180 nodes synchronous processing)
- Confirmed with NOOP handler test (58.6s pass vs 4.1min timeout)
- Applied requestAnimationFrame deferral for onNodeDragStop
- Discovered sequential test failure: design data not loaded (0 ReactFlow elements)
- Added store polling in initDesign (waits for pieces to populate)
- All 6 tests pass consistently (verified 2 runs)
