---
date: "2025-12-03T14:13:54.989Z"
slug: DESIGN-APP-PERFORMANCE
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Design App Pan Performance Investigation
model: claude-opus-4.5
---

# Previously

Investigating Design app performance issues. The test `sketchpad.test.ts` expects pan operations with 180 pieces (Metabolism design) to complete in <100ms.

# Plan

1. Identify performance bottlenecks in Design app state management
2. Implement granular subscriptions to Y.js store
3. Optimize drag/hover/selection hooks to avoid cascade re-renders
4. Test performance with optimized code

# Changes

## Investigation Summary

### Optimizations Applied

1. **Transaction State Context** - Created `TransactionPiecesProvider` with single Y.js subscription to compute affected pieces once, instead of 360 individual subscriptions
2. **Hover State Context** - Created `HoverPiecesProvider` with single subscription for transitive hover state
3. **Selection State Refs** - Changed selection usage in callbacks to use refs instead of state to prevent callback recreation
4. **Drag State Refs** - Changed drag position tracking to use refs instead of state to avoid re-renders during drag
5. **Deferred Y.js Updates** - Moved `updatePieces` call from `onNodeDrag` to `onNodeDragStop` to avoid cascade re-renders during drag
6. **Transaction Stack Caching** - Added caching in `AppStore.currentTransactionStack` getter to avoid repeated `toArray()` calls
7. **Skip Expensive Calculations** - Made `onNodeDrag` return early when alt key is not pressed

### Root Cause Analysis

**Key Finding**: Even with ALL drag callbacks completely disabled (not passed to ReactFlow), pan operations still take ~2200ms.

This proves the bottleneck is NOT in our callback code. The ~2200ms overhead comes from:

- ReactFlow's internal rendering/event handling during pan/drag
- Browser's rendering of 180 DOM nodes
- Possibly other React components re-rendering

### Test Results

| Configuration                     | Pan Duration |
| --------------------------------- | ------------ |
| Original code                     | ~2500ms      |
| With all optimizations            | ~2300ms      |
| With gutted callbacks             | ~2250ms      |
| With callbacks completely removed | ~2200ms      |

### Conclusion

The test expectation of <100ms for operations with 180 nodes appears unrealistic without implementing node virtualization in ReactFlow. The performance overhead is fundamental to rendering 180 React components in the DOM.

### Recommended Next Steps

1. Implement ReactFlow node virtualization to only render visible nodes
2. Or adjust test expectations to be more realistic for large designs
3. Consider using web workers for expensive calculations
