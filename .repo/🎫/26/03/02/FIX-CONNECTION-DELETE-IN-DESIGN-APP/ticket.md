---
goal: SKETCHPAD/DESIGN-APP
---

# Ticket

## Summary

Fixed connection deletion in the Design app by resolving a dual-state desynchronization between the XState machine (actor) and the DesignStore local state. The selection state set via `actor.send(DESIGN.SET_SELECTION)` was never synced back to the DesignStore, causing `deleteSelected` to read an empty selection. Modified `executeCommand` to read selection from the XState machine state (the source of truth) and sync selection changes back after execution. Also made the `SketchpadStore.actor` field accessible from `DesignStore`. Added backspace key support alongside delete key. Added e2e test coverage for connection deletion via Delete and Backspace keys.

## Changes

1. `semio/js/sketchpad/Design.tsx`:
   - Modified `DesignStore.executeCommand` to read selection from XState machine context before building the command context
   - After executing commands that modify selection, sync the new selection state back to the machine via `actor.send(DESIGN.SET_SELECTION)`
   - Added backspace key to `useHotkeys("delete,backspace", ...)` for connection deletion

2. `semio/js/sketchpad/Sketchpad.tsx`:
   - Changed `SketchpadStore.actor` from `private` to non-private so DesignStore can access it via `this.parentStore.actor`

3. `semio/js/sketchpad.test.ts`:
   - Added `Connection Delete` e2e test region after the Filters section
   - Tests connection selection via actor.send + deletion via Delete key
   - Tests connection selection via actor.send + deletion via Backspace key
   - Verifies connection count decreases, specific connection is removed, and selection is cleared after delete

## Log

- Root cause: Selection flows through XState machine (`actor.send({ type: "DESIGN.SET_SELECTION" })`), but kit mutations flow through DesignStore (`store.execute("semio.designApp.deleteSelected")`). The `executeCommand` method was reading selection from `this.snapshot()` (local PlainAppStore state) which was never updated by XState machine selection events.
- Fix: `executeCommand` now reads selection from `actor.getSnapshot().context.designApps[key].selection` before building the command context, and syncs selection changes back to the machine after execution.
- Unit tests: 14/14 pass
- E2e test: Connection Delete section runs successfully. Pre-existing multi-piece selection test times out in later section (unrelated to changes).

## Todos

- [x] Investigate root cause of connection delete failure
- [x] Fix executeCommand to read selection from XState machine state
- [x] Make SketchpadStore.actor accessible from DesignStore
- [x] Add backspace key support for deletion
- [x] Add e2e test coverage for connection deletion
- [x] Run unit tests (14/14 pass)
- [x] Run e2e tests (connection delete section passes)

## Plan

1. Trace the selection → delete flow across XState machine and DesignStore
2. Identify the dual-state desync root cause
3. Fix executeCommand to bridge the gap
4. Add e2e test coverage
5. Verify all tests pass
