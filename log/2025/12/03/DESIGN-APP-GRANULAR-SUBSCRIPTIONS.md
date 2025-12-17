---
slug: DESIGN-APP-GRANULAR-SUBSCRIPTIONS
summary: Optimize state management with granular Y.js subscriptions
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.833Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

Store was overfetching and overrendering due to:

1. `useSyncField` subscribing to entire Y.Map fields, causing all components to re-render when any part of the field changed
2. Hooks like `useDesignAppIsPieceHovered`, `useDesignAppIsPieceSelected` using field-level subscriptions instead of item-level
3. `useDesignAppCommands` creating new object references on every render
4. No throttling on expensive `onNodeDrag` calculations with O(n²) complexity

# Plan

1. ✅ Create item-level Y.Array membership observers (`createNestedArrayItemMembershipObserver`)
2. ✅ Add `useSyncNestedArrayItemMembership` and `useSyncSelectionItemMembership` hooks
3. ✅ Convert `useDesignAppIsPieceHovered` to use item-level subscription
4. ✅ Convert `useDesignAppIsPieceSelected` to use item-level subscription
5. ✅ Convert `useDesignAppIsConnectionHovered/Selected` to use item-level subscription
6. ✅ Memoize `useDesignAppCommands` with `useMemo`
7. ✅ Add `isPanning` flag to skip hover updates during pan operations
8. ✅ Add throttling to `onNodeDrag` to reduce O(n²) helper line calculations
9. ⚠️ Performance still slow (~3s) for 180-piece designs - requires deeper architectural changes

# Changes

## Sketchpad.tsx

- Added `createArrayItemMembershipObserver` - observes specific item membership in Y.Array
- Added `createNestedArrayItemMembershipObserver` - observes nested array items (e.g., hover.pieces)
- Added `useSyncNestedArrayItemMembership` - React hook for granular array item subscription
- Added `useSyncSelectionItemMembership` - React hook specifically for selection arrays

## Design.tsx

- Updated `useDesignAppIsPieceHovered` to use `useSyncNestedArrayItemMembership`
- Updated `useDesignAppIsPieceSelected` to use `useSyncSelectionItemMembership`
- Updated `useDesignAppIsConnectionHovered/Selected` similarly
- Memoized `useDesignAppCommands` to prevent unnecessary re-renders
- Added `isPanning` flag and `onMoveStart`/`onMoveEnd` handlers to skip hover during pan
- Added throttling (50ms) to `onNodeDrag` for expensive helper line calculations
- Added `useDesignAppIsPiecePortSelected` granular hook for port selection

## elements.tsx

- Added `onMoveStart` prop to DiagramProps and ReactFlow component

## Additional Optimizations Made

### Deferred Y.js Updates

- `onNodeDragStart` no longer triggers Y.js updates (selection, transaction)
- All Y.js updates are deferred to `onNodeDragStop`
- Uses refs instead of state for drag position tracking

### DesignStore Dirty Tracking

- Added `dirty` flag to `DesignStore` and `PieceStore`
- `snapshot()` now returns cached value immediately when not dirty
- Avoids O(n) piece/connection snapshot() calls on every access

### Component State Cleanup

- Removed `isDragging` state - now uses `isDraggingRef`
- Removed `helperLines` state - disabled for performance
- Used refs for selection tracking in callbacks

### Full State Subscription Fixes

- Fixed `ToolsToggleGroup` subscribing to entire state with `useDesignApp((s) => s)`
- Fixed `App` component subscribing to entire state
- Both now use targeted hooks (`useDesignAppActiveTool`)

## Test Results

Despite all optimizations, pan operations still take ~2300ms vs expected <100ms.

## Root Cause Analysis

Even with all optimizations, performance remains poor because:

1. **180 node subscriptions**: Each `PieceNodeInner` calls multiple hooks:
   - `useDesignAppPieceColor` (4 internal hooks)
   - `useIsPieceHovered`
   - `useDiffedPiece` (calls `usePiece` which subscribes to PieceStore)
2. **useSyncExternalStore overhead**: React calls `getSnapshot` for ALL subscriptions when ANY subscription fires
3. **Development mode overhead**: Running in dev mode with React's extra checks

4. **ReactFlow internal overhead**: ReactFlow itself may be doing expensive operations during drag

## Next Steps

The user indicated "There is an alternative version that renders the same example without issues without any advanced strategy" - this suggests:

- The issue might be a fundamental misconfiguration rather than optimization
- Need to compare with working version to identify the actual difference
- Consider if there's a simpler hook pattern that avoids the subscription cascade
