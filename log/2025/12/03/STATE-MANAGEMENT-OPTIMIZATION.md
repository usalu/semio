---
date: "2025-12-03T08:40:30.014Z"
slug: STATE-MANAGEMENT-OPTIMIZATION
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Optimize Yjs state management to prevent overfetching and infinite rerenders
model: claude-sonnet-4.5
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

1. Create stable selectors as module-level constants for TypeApp hooks
2. Fix `useSyncField` to use stable selector callback pattern
3. Ensure hooks use memoized selectors to prevent infinite loops

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
