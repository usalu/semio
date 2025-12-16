---
slug: STATE-MANAGEMENT-REFINEMENT
summary: 'Refine state management: granular Yjs subscriptions for selectors'
---

# Previously

The state management used `useSyncDeep` which subscribes to the entire Y.Map with `observeDeep`, causing unnecessary re-renders when ANY nested field changes, even if the selected data hasn't changed.

# Plan

1. Add granular subscription methods to Store class (`onFieldChanged`, `onFieldsChanged`)
2. Create `createFieldObserver` and `createFieldsObserver` functions that only trigger when specific Y.Map keys change
3. Add `useSyncField` and `useSyncFields` hooks for granular subscriptions
4. Update Type.tsx and Design.tsx hooks to use granular subscriptions

# Changes

## Sketchpad.tsx

- Added `onFieldChanged(key, subscribe, deep)` method to Store class
- Added `onFieldsChanged(keys, subscribe, deep)` method to Store class
- Added `createFieldObserver` function that:
  - Subscribes to parent Y.Map for key creation/deletion
  - Optionally subscribes deeply to nested Y.Map/Y.Array structures
  - Only triggers callback when the specific key changes
- Added `createFieldsObserver` function for subscribing to multiple keys
- Added `useSyncField` hook that uses `onFieldChanged` for granular subscriptions
- Added `useSyncFields` hook that uses `onFieldsChanged` for multiple keys

## Type.tsx

Updated hooks to use granular subscriptions:

- `useTypeAppSelection` - subscribes to "selection" field
- `useTypeAppPanelVisibility` - subscribes to "panelVisibility" field
- `useTypeAppCamera` - subscribes to "camera" field (shallow)
- `useTypeAppFocusedPortGuid` - subscribes to "focusedPortGuid" field (shallow)
- `useTypeAppHover` - subscribes to "hover" field
- `useTypeAppActiveTool` - subscribes to "activeTool" field (shallow)
- `useTypeAppIsPortSelected` - subscribes to "selection" field
- `useTypeAppIsPortHovered` - subscribes to "hover" field
- `useTypeAppSelectedModelGuid` - subscribes to "selectedModelGuid" field (shallow)
- `useTypeAppSelectedModelTags` - subscribes to "selectedModelTags" field

## Design.tsx

Updated hooks to use granular subscriptions:

- `useDesignAppSelection` - subscribes to "selection" field
- `useDesignAppFullscreen` - subscribes to "fullscreenWindow" field (shallow)
- `useDesignAppCamera` - subscribes to "camera" field (shallow)
- `useDesignAppDiagramCenter` - subscribes to "diagramCenter" field (shallow)
- `useDesignAppDiagramScale` - subscribes to "diagramScale" field (shallow)
- `useDesignAppFocusedPieceGuid` - subscribes to "focusedPieceGuid" field (shallow)
- `useDesignAppSelectedModelTags` - subscribes to "selectedModelTags" field
- `useDesignAppHover` - subscribes to "hover" field
- `useDesignAppIsPieceHovered` - subscribes to "hover" field
- `useDesignAppIsPieceSelected` - subscribes to "selection" field
- `useDesignAppIsConnectionHovered` - subscribes to "hover" field
- `useDesignAppIsConnectionSelected` - subscribes to "selection" field
