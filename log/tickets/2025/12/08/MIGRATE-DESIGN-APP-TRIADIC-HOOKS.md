---
slug: MIGRATE-DESIGN-APP-TRIADIC-HOOKS
summary: >-
  Migrate Design.tsx UI components from useDesignAppCommands to triadic action
  hooks
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.887Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

The codebase used `useDesignAppCommands()` hook to access all design app commands. UI components would destructure specific commands and call them with origin strings. This mixed app state operations (hover, selection, focus) with kit mutations (addPiece, updatePiece, addConnection).

# Plan

1. Create granular action hooks for app state operations following the `[action, canAct]` pattern:
   - Hover: `useDesignAppHoverPiece`, `useDesignAppHoverConnection`, `useDesignAppClearHover`
   - Selection: `useDesignAppSelectPiece`, `useDesignAppSelectPieces`, `useDesignAppAddPieceToSelection`, `useDesignAppRemovePieceFromSelection`
   - Focus: `useDesignAppFocusPiece`, `useDesignAppClearFocus`
   - Fullscreen: `useDesignAppToggleDiagramFullscreen`, `useDesignAppToggleAccesslFullscreen`
   - Model tags: `useDesignAppAddModelTagForAllTypes`, `useDesignAppRemoveModelTagFromAllTypes`

2. Keep kit mutation commands in `useDesignAppCommands` (addPiece, updatePiece, addConnection, etc.) as they require transaction handling

3. Migrate each UI component to use action hooks instead of destructuring from `useDesignAppCommands`

4. Update callback signatures to not require origin strings (action hooks handle this internally)

# Changes

- Added 20+ action hooks in Design.tsx Store region:
  - `useDesignAppSelectPiece`, `useDesignAppSelectPieces`, `useDesignAppAddPieceToSelection`, `useDesignAppRemovePieceFromSelection`
  - `useDesignAppSelectConnection`, `useDesignAppAddConnectionToSelection`, `useDesignAppRemoveConnectionFromSelection`
  - `useDesignAppSelectPiecePort`, `useDesignAppDeselectPiecePort`
  - `useDesignAppHoverPiece`, `useDesignAppHoverConnection`, `useDesignAppHoverTypes`, `useDesignAppHoverDesigns`, `useDesignAppClearHover`
  - `useDesignAppFocusPiece`, `useDesignAppClearFocus`
  - `useDesignAppSelectAll`, `useDesignAppDeselectAll`
  - `useDesignAppToggleDiagramFullscreen`, `useDesignAppToggleAccesslFullscreen`, `useDesignAppTogglePanel`
  - `useDesignAppAddModelTagForAllTypes`, `useDesignAppRemoveModelTagFromAllTypes`

- Migrated components:
  - `PortHandle`: Uses `useDesignAppHoverPort` instead of `commands.hoverPort`
  - `ConnectionEdgeInner`: Uses `useDesignAppHoverConnection`, `useDesignAppClearHover`
  - `ModelPiece`: Uses selection and hover action hooks
  - `DesignNodeComponent`: Uses selection, hover, and port selection hooks
  - `DesignDiagram`: Uses all diagram-related action hooks
  - `ModelDesign`: Uses `useDesignAppSelectPieces`
  - `DesignAppScene`: Uses fullscreen, camera, focus, and deselect hooks
  - `App`: Uses selection, fullscreen, hover, and tool action hooks
  - `DesignAppFooter`: Uses model tag action hooks

- Action hooks return `[action, canAct]` tuple where `canAct` indicates if the action is available (app context exists)

- Kit mutation commands remain in `useDesignAppCommands` for transaction handling

## Final State

- **11 usages of `useDesignAppCommands` in Design.tsx**: 1 definition + 10 usages for kit mutations
- **10+ usages in Sketchpad.tsx**: Property hooks (`usePieceCenterU`, `useConnectionGap`, etc.) that call `updatePiece`/`updateConnection`
- All **app state operations** now use triadic action hooks
- All **kit mutations** correctly use commands with origin strings for undo/redo

### Kit Mutations (remain using useDesignAppCommands)

- `addPiece`, `updatePiece`, `updatePieces`, `deletePiece`
- `addConnection`, `addConnections`, `updateConnection`, `updateConnections`
- `startTransaction`, `finalizeTransaction`, `abortTransaction`
- `undo`, `redo`, `deleteSelected`, `execute`

### App State Operations (migrated to action hooks)

- Selection: `selectPiece`, `selectPieces`, `addPieceToSelection`, `removePieceFromSelection`, `selectConnection`, `addConnectionToSelection`, `removeConnectionFromSelection`, `selectAll`, `deselectAll`
- Hover: `hoverPiece`, `hoverConnection`, `hoverPort`, `hoverTypes`, `hoverDesigns`, `clearHover`
- Focus: `focusPiece`, `clearFocus`
- UI: `toggleDiagramFullscreen`, `toggleAccesslFullscreen`, `togglePanel`, `setCamera`, `setDiagramCenter`, `setDiagramScale`, `setActiveTool`
- Model Tags: `addModelTagForAllTypes`, `removeModelTagFromAllTypes`

## Type.tsx Migration (Completed)

### Action Hooks Created

- Selection: `useTypeAppSelectPort`, `useTypeAppDeselectPort`, `useTypeAppDeselectAll`, `useTypeAppSelectModel`, `useTypeAppDeselectModel`
- Hover: `useTypeAppHoverPort`, `useTypeAppHoverModel`, `useTypeAppClearHover`
- Focus: `useTypeAppFocusPort`, `useTypeAppClearFocus`
- UI: `useTypeAppSetActiveTool`, `useTypeAppSetCamera`, `useTypeAppTogglePanel`
- Model Tags: `useTypeAppAddModelTag`, `useTypeAppRemoveModelTag`, `useTypeAppSetSelectedModel`

### UI Components Migrated

- `SceneContent`: selectPort, deselectPort, hoverPort, clearHover, focusPort
- `Scene`: setCamera, deselectAll, clearFocus
- `ModelsSectionForm`: selectModel, deselectModel, hoverModel, clearHover
- `PortsListSectionForm`: selectPort, deselectPort, hoverPort, clearHover
- `ToolsToggleGroup`: setActiveTool
- `App`: setActiveTool (keyboard handling via useHotkeys)
- `TypeAppFooter`: addModelTag, removeModelTag

### Remaining Commands (kit mutations)

- `addPort`, `updatePort`, `deletePort`
- `addModel`, `updateModel`, `deleteModel`
- `undo`, `redo`

## Quality.tsx Migration (Completed)

### Action Hooks Created

- Selection: `useQualityAppSelectFormulaNode`, `useQualityAppDeselectAll`
- Hover: `useQualityAppHoverFormulaNode`, `useQualityAppClearHover`
- UI: `useQualityAppTogglePanel`, `useQualityAppToggleFormulaFullscreen`, `useQualityAppToggleDiagramFullscreen`

### UI Components Migrated

- `QualityDiagram`: selectFormulaNode, hoverFormulaNode, clearHover
- `App`: deselectAll, togglePanel, toggleFormulaFullscreen, toggleDiagramFullscreen

### Remaining Commands (kit mutations)

- `connectNodes`, `updateFormula`
- `undo`, `redo`

## Remaining Work

### Kit.tsx

Extensive selection operations still using `useKitAppCommands`:

- `selectDesign`, `selectType`, `deselectAll`
- `addToSelection`, `removeFromSelection`
- `toggleExpandedRow`
- `setCamera`

### Home.tsx

Selection and sorting operations still using `useHomeCommands`:

- `selectKit`, `deselectAll`
- `setSortColumn`, `setSortDirection`, `toggleSort`

### Sketchpad-level

Settings commands used across apps:

- `setTheme`, `setLanguage`, `setLayout`, `setExpertise`, `setMode`

## Test Results

- All 9 tests pass
- No new TypeScript errors in Type.tsx or Quality.tsx (pre-existing Design.tsx errors remain)
