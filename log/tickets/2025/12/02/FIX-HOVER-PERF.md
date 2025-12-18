---
slug: FIX-HOVER-PERF
summary: Fix hover and selection state overfetching for performance
prompt: Fix hover and selection state overfetching for performance
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-16T17:06:07.810Z'
commit: '0000000000000000000000000000000000000000'
iterations: []
---

# Previously

Design app had severe performance issues with hovering/selection. With 180 pieces in Nakagin Capsule Tower, hover operations took 3-5 seconds due to cascade re-renders.

Root causes identified:

1. `useIsPieceHovered()` called `useDesignAppHover()` which returns the FULL hover object - any hover change caused ALL pieces to re-render
2. `useDesignAppIsPieceTransitiveHovered()` used `useSync` subscribing to ALL store changes instead of just hover field
3. `PortHandle` component used `useDesignAppHover()` directly - every port re-rendered on any hover change
4. `PieceNodeComponent` and `DesignNodeComponent` used `useDesignAppSelection()` for checking isSelected - all nodes re-rendered on any selection change

# Plan

1. ✅ Analyze hover state subscription patterns to identify overfetching
2. ✅ Fix `useIsPieceHovered` to use granular subscription instead of full hover object
3. ✅ Fix `useDesignAppIsPieceTransitiveHovered` to use `useSyncField` with "hover" key
4. ✅ Fix `useIsPieceTransitiveHovered` and `useIsConnectionHovered` similarly
5. ✅ Add granular hooks for port hover and port selection
6. ✅ Fix `PieceNodeComponent` and `DesignNodeComponent` to use granular hooks
7. ✅ Add hover performance test to verify 100ms hover/unhover timing

# Changes

## Design.tsx

- Added `useDesignAppIsPortHovered(id, pieceId, portId)` - granular hook for port hover state
- Added `useDesignAppSelectedPort(id)` - granular hook for selected port only
- Fixed `useDesignAppIsPieceTransitiveHovered` to use `useSyncField("hover", ...)` instead of `useSync`
- Fixed `useDesignAppIsTypeTransitiveHovered` similarly
- Fixed `PortHandle` to use `useDesignAppIsPortHovered` instead of full hover object
- Fixed `PieceNodeComponent` to use:
  - `useDesignAppIsPieceSelected(undefined, guid)` for isSelected
  - `useDesignAppSelectedPort()` instead of full selection
- Fixed `DesignNodeComponent` similarly
- Updated `PieceNodeInnerProps` and `DesignNodeInnerProps` to use `selectedPort` instead of `selection`

## Sketchpad.tsx

- Added granular hooks to `getDesignAppHooks()`:
  - `useDesignAppIsPieceHovered`
  - `useDesignAppIsConnectionHovered`
  - `useDesignAppIsPieceSelected`
  - `useDesignAppIsConnectionSelected`
- Fixed `useIsPieceHovered()` to use `useDesignAppIsPieceHovered`
- Fixed `useIsConnectionHovered()` to use `useDesignAppIsConnectionHovered`
- Fixed `useIsPieceSelected()` to use `useDesignAppIsPieceSelected`
- Fixed `useIsConnectionSelected()` to use `useDesignAppIsConnectionSelected`

## sketchpad.test.ts

- Added hover performance test:
  - Measures hover (mouse enter) time
  - Measures unhover (mouse leave) time
  - Runs 3 hover cycles to verify consistency
  - Asserts all operations complete under 100ms
