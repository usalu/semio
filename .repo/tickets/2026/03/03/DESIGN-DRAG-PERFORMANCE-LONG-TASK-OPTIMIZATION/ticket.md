---
goal: PERF/DRAG-PERFORMANCE
---

# Ticket

## Summary

Fixed hover state propagation in PieceRenderData subscription store. The syncPieceRenderData function was subscribing only to designStore (PlainAppStore), but hover state lives exclusively in XState context. Added XState actor subscription alongside designStore subscription so hover and selection changes from XState properly propagate to the PieceRenderData store. All 11 Playwright tests pass. Design Drag Performance: 7 long tasks, max 128ms (budget: 150ms).

## Changes

- `semio/js/sketchpad/Design.tsx`:
  - Modified `syncPieceRenderData` to accept `hover` and `selection` parameters instead of reading from `designStore.snapshot()`
  - Updated useEffect in DesignDiagram to subscribe to both `designStore` and XState `actorForSync`
  - Actor subscription reads hover/selection from `actorForSync.getSnapshot().context.designApps[key]`
  - Added `useSketchpadActorSafe`, `useKitScope`, `useDesignScope` hooks in DesignDiagram for XState access

## Log

- Traced hover state flow: actor.send(DESIGN.SET_HOVER) → XState wildcard → dispatchAppEvent → executeEventHandler → registered via registerKeyedAppEventHandlers → updates XState context.designApps[key].hover
- Confirmed PlainAppStore (DesignStore) is never notified for hover changes — hover only lives in XState context
- Fixed syncPieceRenderData to read hover/selection from XState actor instead of PlainAppStore
- test-equiv: 5 tasks, max 94ms (improved from prior 111ms)
- phase-trace: ZOOM_IN 63ms, ZOOM_OUT 63ms, MOUSE_MOVE 120ms+97ms, DRAG_START 52ms+98ms
- Playwright test: 7 tasks, max 128ms, PASS (budget: 150ms)
- Full suite: 11/11 passed

## Todos

- [x] Fix syncPieceRenderData hover subscription
- [x] Test current state with Vite restart
- [x] Run phase-trace to measure improvements
- [x] Run actual Playwright test
- [x] Run full test suite validation
- [x] Update ticket and close

## Plan

1. Identified that hover state is only stored in XState context (via registerKeyedAppEventHandlers), not in PlainAppStore
2. Modified syncPieceRenderData to accept hover/selection from caller instead of reading from designStore.snapshot()
3. Updated useEffect to subscribe to XState actor alongside designStore
4. Verified all tests pass
