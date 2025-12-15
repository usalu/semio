---
slug: STORE-OVERFETCH-FIX
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix store overfetching and overrendering in design app
model: claude-opus-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---

# Previously

The design app was experiencing performance issues during operations like panning and hovering. Analysis revealed that:

1. The Store class's `snapshot()` method called `buildSnapshot()` on every invocation, even when Y.js data hadn't changed
2. Many hooks used `useSync` with full state subscriptions when they only needed specific fields
3. The Design test was measuring 244ms for pan operations when <100ms was expected

# Plan

1. Add dirty-tracking to the Store base class to avoid unnecessary snapshot rebuilds
2. Create granular hooks (e.g., `useDesignAppActiveTool`) using `useSyncField` instead of full state subscriptions
3. Refactor hooks that check transaction stack to use field-level subscriptions

# Changes

## Sketchpad.tsx - Store class optimization

Added dirty-tracking mechanism to the Store base class:

- Added `dirty: boolean = true` flag
- Added `internalObserverDisposer` for cleanup
- Added `setupDirtyTracking()` method that sets up a Y.js deep observer to mark cache as dirty when data changes
- Modified `snapshot()` to return cached snapshot immediately if `dirty === false`

This prevents unnecessary `buildSnapshot()` calls when multiple hooks call `snapshot()` in the same render cycle and Y.js data hasn't changed.

## Design.tsx - Granular hooks (staged changes)

The staged changes include:

- `useDesignAppActiveTool` - Uses `useSyncField` with "activeTool" key
- `useDesignAppOthers` - Changed from `useSyncDeep` to `useSyncField`
- `useIsDesignPieceChangedInTransaction` - Uses `useSyncField` with "currentTransactionStack" key
- `useDesignAppPieceStatus` - Uses `useSyncField` with "currentTransactionStack" key
- `useDesignAppConnectionStatus` - Uses `useSyncField` with "currentTransactionStack" key
- Helper functions for checking piece/connection status from transaction stack

## Note on test failures

The Design test was already failing before these changes were applied. The failure is related to kit import/loading issues, not the performance optimizations. The test shows "Kit not found" in the error context, indicating a pre-existing issue with the test environment or kit loading logic.
