---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Restored cross-root filter synchronization lost during concurrent edits. Added globalThis singleton filter store with useSyncExternalStore for cross-React-root state sharing. Wrapped DiagramWindow/SceneWindow with DesignFilterProvider. Changed diagram filtering from hidden:true to array removal. All 7 e2e tests pass, 13 unit tests pass, 0 TS errors. Piece detail panel fully renders (ID, Type, Variant, Connection, Plane, Diagram fields) when piece is selected.

## Changes

- `compose/js/sketchpad/Design.tsx`: Added global singleton filter store (globalThis + useSyncExternalStore), wrapped DiagramWindow/SceneWindow with DesignFilterProvider, changed diagram filtering from hidden:true to array removal, synced toolbar toggles to shared store

## Log

- Read DESIGN-APP-SKETCHPAD-FILTERS ticket (2026/02/17) for prior fix details
- Identified filter regression: useSyncExternalStore cross-root sync overwritten by concurrent edits
- DiagramWindow/SceneWindow not wrapped with DesignFilterProvider
- baseNodes used hidden:true instead of array filtering
- Implemented global singleton filter store with subscribe/getSnapshot/notify pattern
- Updated DesignFilterProvider to use useSyncExternalStore
- Synced toolbar toggles to shared store immediately (before setSearchParams)
- Added useEffect to sync on URL searchParams changes
- Wrapped DiagramWindow and SceneWindow with DesignFilterProvider
- Changed baseNodes/edges from hidden:true to empty array when filtered off
- Tied edge visibility to both showPieces AND showConnections
- Verified: 7/7 e2e tests pass, 13/13 unit tests pass, 0 TS errors

## Todos

- [x] Read DESIGN-APP-SKETCHPAD-FILTERS ticket for fix details
- [x] Reopen PIECE-DETAILS-PANEL ticket
- [x] Restore cross-root filter synchronization (globalThis + useSyncExternalStore)
- [x] Fix diagram node filtering (array removal instead of hidden:true)
- [x] Wrap DiagramWindow/SceneWindow with DesignFilterProvider
- [x] Run e2e Design test to verify filters
- [x] Runtime verify piece details panel (7/7 e2e pass including piece selection + details validation)
- [x] Close ticket with summary

## Plan

1. Add global singleton filter store on globalThis with subscribe/getSnapshot/notify pattern
2. Update DesignFilterProvider to use useSyncExternalStore for cross-root sync
3. Sync URL params → shared store and toolbar toggle → shared store
4. Wrap DiagramWindow and SceneWindow with DesignFilterProvider
5. Change baseNodes/edges from hidden:true to array .filter() removal
6. Tie edge visibility to both pieces and connections filter state
7. Run Design Playwright test → PASSED
8. Verify piece details panel renders at runtime → PASSED (all e2e pass)
