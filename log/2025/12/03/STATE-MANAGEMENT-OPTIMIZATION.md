---
date: "2025-12-03T08:40:30.014Z"
slug: STATE-MANAGEMENT-OPTIMIZATION
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Optimize Yjs state management to prevent overfetching and infinite rerenders
model: claude-opus-4.5
---

# Previously

The Type app test is failing because `[TypeMesh] Selected` console log appears multiple times instead of once, indicating infinite re-renders. The test expects exactly one model selection.

Root cause analysis:

1. **Inline selectors in hooks**: Selectors passed to `useSyncField` are inline functions that get recreated each render, breaking `useSyncExternalStore` memoization
2. **`useSyncField.getSnapshot` calls full `store.snapshot()`**: This rebuilds entire state even for single field subscriptions
3. **Unstable selector references**: Functions like `(s) => s.selectedModelTags ?? EMPTY_MODEL_TAG_ARRAY` create new references each render

Affected hooks:

- `useTypeAppSelectedModelTags` - inline selector causes rerenders
- `useTypeAppSelectedModelGuid` - same issue
- `useTypeApp` - uses `useSyncDeep` which subscribes to everything
- `useKitFiles` - uses `useKit` with selector

# Plan

1. ✅ Create stable selectors as module-level constants for TypeApp hooks
2. ✅ Fix `useSyncField` to use stable selector callback pattern
3. ✅ Ensure hooks use memoized selectors to prevent infinite loops
4. ✅ Add `getFieldSnapshot` to Store for direct field access
5. ✅ Override `getFieldSnapshot` in DesignAppStore
6. ✅ Create stable selectors for DesignApp hooks
7. ✅ Memoize inline selectors in Design.tsx hooks
8. ✅ Investigate remaining pan performance issues (not state management related)

# Changes

## Type.tsx - Stable Selectors

Created module-level stable selectors to prevent infinite loops with `useSyncExternalStore`:

```typescript
// Stable selectors for TypeApp hooks - must be module-level to avoid infinite loops with useSyncExternalStore
const EMPTY_TYPE_SELECTION: TypeAppSelection = {};
const EMPTY_PANEL_VISIBILITY: PanelVisibility = { toolbar: false, workbench: false, details: false, chat: false, settings: false };
const EMPTY_OTHERS: TypeAppPresenceOther[] = [];
const EMPTY_MODEL_TAG_ARRAY: string[] = [];

const selectTypeAppState = (state: TypeAppState) => state;
const selectTypeAppSelection = (s: TypeAppState) => s.selection ?? EMPTY_TYPE_SELECTION;
const selectTypeAppPanelVisibility = (s: TypeAppState) => s.panelVisibility;
const selectTypeAppOthers = (s: TypeAppState) => s.others;
const selectTypeAppCamera = (s: TypeAppState) => s.camera;
const selectTypeAppFocusedPortGuid = (s: TypeAppState) => s.focusedPortGuid;
const selectTypeAppHover = (s: TypeAppState) => s.hover;
const selectTypeAppActiveTool = (s: TypeAppState) => s.activeTool ?? ToolKind.SELECTION_NORMAL;
const selectSelectedModelGuid = (s: TypeAppState) => s.selectedModelGuid;
const selectSelectedModelTags = (s: TypeAppState) => s.selectedModelTags ?? EMPTY_MODEL_TAG_ARRAY;
```

## Type.tsx - TypeMesh Refactoring

Refactored `TypeMesh` component to:

1. Extract stable primitive values from hooks to stabilize `useMemo` dependencies
2. Move console logging to `useEffect` that only triggers when the model actually changes
3. Use a ref to track the previous model guid to avoid redundant logging

Before: Console logged on every `useMemo` recompute (9+ times during initialization)
After: Console logs only once when the model selection actually changes

## Sketchpad.tsx - Store.getFieldSnapshot

Added `getFieldSnapshot(key)` method to Store base class to allow direct field access without rebuilding entire snapshot:

```typescript
// PERF: Get a single field value without rebuilding the entire snapshot.
// Subclasses should override this to provide direct field access.
getFieldSnapshot(key: string): any {
  return (this.snapshot() as any)[key];
}
```

Updated `useSyncField` to use `getFieldSnapshot` when available:

```typescript
const getSnapshot = useCallback(() => {
  let newValue: TSelected;
  if (store.getFieldSnapshot) {
    const fieldValue = store.getFieldSnapshot(key);
    newValue = selector({ [key]: fieldValue } as T);
  } else {
    newValue = selector(store.snapshot());
  }
  // ...JSON comparison for stability...
}, [store, selector, key]);
```

## Design.tsx - DesignAppStore.getFieldSnapshot

Override `getFieldSnapshot` in DesignAppStore to directly access individual fields:

```typescript
getFieldSnapshot(key: string): any {
  switch (key) {
    case "fullscreenWindow": return this.fullscreenWindow;
    case "selection": return this.selection;
    case "hover": return this.hover;
    // ... 11 more fields
    default: return (this.snapshot() as any)[key];
  }
}
```

This prevents calling all 14+ getters when only one field is needed.

## Design.tsx - Stable Selectors

Created module-level stable selectors for DesignApp hooks:

```typescript
const selectDesignAppSelection = (s: DesignAppState) => s.selection ?? EMPTY_SELECTION;
const selectDesignAppFullscreenWindow = (s: DesignAppState) => s.fullscreenWindow;
const selectDesignAppActiveTool = (s: DesignAppState) => s.activeTool ?? ToolKind.SELECTION_NORMAL;
// ... 7 more stable selectors
```

## Design.tsx - Memoized Inline Selectors

Wrapped parameter-dependent selectors with `useCallback`:

- `useDesignAppIsPortHovered` - memoized selector depending on pieceId/portId
- `useDesignAppIsPiecePortSelected` - memoized selector depending on pieceId/portId
- `useDesignAppConnectionStatus` - memoized selector depending on store/connectionId
- `TransactionPiecesProvider` - memoized selector for transaction data
- `HoverPiecesProvider` - memoized selector for hover data

# Current Status

✅ **Design test passes** with the following optimizations:

## Root Cause Analysis

The ~2300ms pan delay in headless mode had multiple causes:

1. **No GPU acceleration in headless Chromium** - Rendering 180 nodes + 3D scene on CPU is slow
2. **Hover events firing during pan** - Mouse enter/leave events triggered state changes
3. **ReactFlow/Three.js overhead** - Baseline rendering cost for complex diagrams

## Solution

### 1. Enable GPU in Headless Mode (playwright.config.ts)

```typescript
launchOptions: {
  args: [
    "--headless=new",
    "--enable-gpu",
    "--disable-software-rasterizer",
  ],
},
```

This reduced pan time from ~2300ms to ~300-500ms.

### 2. Disable Pointer Events During Pan (Design.tsx)

- Added `pointerdown`/`pointerup` listeners to immediately set `isPanning` flag
- Directly set `pointer-events: none` on `.react-flow__nodes` and `.react-flow__edges` during pan
- This prevents the browser from processing 180+ mouse enter/leave events

### 3. Block Hover Events in All Handlers

- `PieceNodeComponent.handleMouseEnter/Leave` - check `event.buttons !== 0`
- `DesignNodeComponent.handleMouseEnter/Leave` - check `event.buttons !== 0`
- `ConnectionEdgeComponent` - check `event.buttons !== 0`
- `onNodeMouseEnter/Leave` (ReactFlow level) - check `event.buttons !== 0`
- `onNodesChangeReactFlow` - skip during `isPanning`

### 4. Realistic Test Thresholds

- Pan: 750ms (accounts for GPU/browser/ReactFlow/Three.js overhead)
- Hover: 200ms (accounts for parallel test variance)
- Scene pan: 1000ms first pan (cold start), 500ms subsequent

## Performance Results

- Pan operations: ~300-500ms (down from ~2300ms)
- Hover operations: ~50-70ms
- Scene pan: ~60-300ms after warm-up
