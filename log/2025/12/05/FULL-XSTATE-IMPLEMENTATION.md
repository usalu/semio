---
date: "2025-12-05T15:40:53.424Z"
slug: FULL-XSTATE-IMPLEMENTATION
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Implement full XState transition - no Yjs in apps
model: claude-sonnet-4.5
---

# Previously

See FULL-XSTATE-TRANSITION.md for detailed architecture plan.

Current state:

- Yjs-backed stores (HomeStore, KitAppStore, TypeAppStore, DesignAppStore) manage UI state
- XState machine exists but is minimally used, mainly mirroring Yjs state
- React hooks use useSyncDeep/useSyncField directly on Yjs stores
- DesignAppSyncComponent mirrors Yjs → XState (adhoc integration)

# Plan

## Key Requirements

1. **No Yjs in apps** - React components don't touch Yjs directly
2. **One Yjs document per kit** - Yjs only for collaborative kit data
3. **Clean XState machine** - Full XState features, not thin wrapper

## Architecture

```
┌─────────────────────────────┐
│         React UI            │
│  useSelector(actor, ...)    │
└──────────▲──────────────────┘
           │ XState events + state
┌──────────────────────────────┐
│      sketchpadMachine        │
│  - All app state in context  │
│  - Actions mutate context    │
│  - Yjs sync for kit data     │
└──────────▲───────────────────┘
           │ Kit data only
┌──────────────────────────────┐
│     Yjs (yDoc, yKit maps)    │
│  - Kit/Design/Type data      │
│  - NOT UI state              │
└──────────────────────────────┘
```

## Implementation Steps

1. **machines.ts** - Complete XState machine with:
   - Full app state in context (homeApp, kitApps, typeApps, designApps, qualityApps)
   - Typed events for all UI operations
   - Yjs sync actor for kit data only
   - Transaction/undo support via XState context

2. **xstate-hooks.ts** - All hooks use XState selectors:
   - useHomeApp, useHomeCommands
   - useKitApp, useKitAppCommands
   - useTypeApp, useTypeAppCommands
   - useDesignApp, useDesignAppCommands
   - useQualityApp, useQualityAppCommands

3. **Remove Yjs from apps**:
   - Delete HomeStore, KitAppStore, TypeAppStore, DesignAppStore classes
   - Keep KitStore for kit data only
   - Remove useSyncDeep/useSyncField from React hooks

4. **Update components**:
   - Replace store.execute() with actor.send()
   - Replace useSyncDeep with useSelector
   - Remove DesignAppSyncComponent etc.

# Changes

## machines.ts

- Updated header comment explaining architecture (XState as SSOT, Yjs for kit data only)
- Added complete type definitions:
  - `TypeAppHover`, `TypeAppFullscreenWindow` enums
  - Extended `TypeAppState` with `activeTool`, `fullscreenWindow`, `selectedModelGuid`, `windowLayout`
- Added comprehensive events for all apps:
  - Kit: `SELECT_TYPE`, `DESELECT_TYPE`, `SELECT_DESIGN`, `DESELECT_DESIGN`, `SET_SELECTION`, `CLEAR_SELECTION`, `SET_HOVER`, `CLEAR_HOVER`
  - Type: `SET_ACTIVE_TOOL`, `SET_SELECTION`, `CLEAR_SELECTION`, `SELECT_PORT`, `DESELECT_PORT`, `SET_HOVER`, `CLEAR_HOVER`, `SET_MODEL_TAGS`
  - Design: `SELECT_ALL`, `DELETE_SELECTED`
  - Transaction: `START`, `COMMIT`, `ABORT`, `UNDO`, `REDO`
- Added corresponding actions for all new events
- Added new selectors:
  - Type: `createTypeActiveToolSelector`, `createTypeHoverSelector`, `createTypeFullscreenWindowSelector`
  - Kit: `createKitAppSelector`, `createKitPanelVisibilitySelector`, `createKitSelectionSelector`, `createKitHoverSelector`, `createKitFilterSearchSelector`, `createKitExpandedRowsSelector`
- Added actor type exports: `SketchpadActorRef`, `SketchpadSnapshot`, `SketchpadState$`

## xstate-hooks.ts (NEW)

Created new file with clean React hooks that use XState selectors:

- Home: `useHomeApp`, `useHomePanelVisibility`, `useHomeSelection`, `useHomeHover`, `useHomeSortColumn`, `useHomeSortDirection`, `useHomeLoadingKits`, `useHomeCommands`
- Kit: `useKitApp`, `useKitPanelVisibility`, `useKitSelection`, `useKitHover`, `useKitFilterSearch`, `useKitExpandedRows`, `useKitAppCommands`
- Type: `useTypeApp`, `useTypePanelVisibility`, `useTypeSelection`, `useTypeHover`, `useTypeFocusedPort`, `useTypeSelectedModelTags`, `useTypeCamera`, `useTypeActiveTool`, `useTypeFullscreenWindow`, `useTypeAppCommands`
- Design: `useDesignApp`, `useDesignPanelVisibility`, `useDesignSelection`, `useDesignHover`, `useDesignFocusedPiece`, `useDesignSelectedModelTags`, `useDesignDiagramCenter`, `useDesignDiagramScale`, `useDesignCamera`, `useDesignActiveTool`, `useDesignFullscreenWindow`, `useDesignAppCommands`
- Utilities: `useIsPieceSelected`, `useIsPieceHovered`, `useIsConnectionSelected`, `useIsConnectionHovered`, `useIsPortSelected`, `useIsPortHovered`
- Exports `SketchpadActorContext` for use by Sketchpad.tsx

## Sketchpad.tsx

- Imported `SketchpadActorRef` from machines.ts
- Imported `SketchpadActorContext` from xstate-hooks.ts
- Removed local `SketchpadActorContext` definition (now using shared context)
- `SketchpadScopeProvider` provides the XState actor through the shared context

## Home.tsx

- Added imports from xstate-hooks (aliased as `*XState`)
- Updated `useHome()` to use `useHomeAppXState()` from XState
- Updated `useHomePanelVisibility()` to use `useHomePanelVisibilityXState()`
- Updated `useHomeCommands()` to use `useHomeCommandsXState()` (wrapped for backwards compatibility with origin parameter)
- Kept `HomeStore` class for legacy command execution

## Kit.tsx

- Added imports from xstate-hooks (aliased as `*XState`)
- Updated `useKitApp()` to use `useKitAppXState()` from XState
- Kept `KitAppStore` class and legacy hooks for complex functionality

## Type.tsx

- Added imports from xstate-hooks (aliased as `*XState`)
- Updated `useTypeApp()` to use `useTypeAppXState()` from XState
- Kept `TypeAppStore` class and legacy hooks for complex functionality

## Design.tsx

- Added imports from xstate-hooks (aliased as `*XState`)
- Updated `useDesignApp()` to use `useDesignAppXState()` from XState
- Kept `DesignAppStore` class and legacy hooks for complex functionality

## Migration Log

### Phase 1: XState Machine and Hooks

- Updated machines.ts with complete XState machine and type definitions
- Created new xstate-hooks.ts file with clean React hooks using XState selectors
- Updated Sketchpad.tsx to use shared SketchpadActorContext

### Phase 2: Full Migration (No Backwards Compatibility)

- **Home.tsx**: Removed HomeStore class entirely, replaced with XState hooks
  - Re-exported hooks from xstate-hooks: `useHomeApp as useHome`, `useHomePanelVisibility`, `useHomeSelection`, etc.
  - HomeDropZone uses local React state for loading kits instead of Yjs
- **Kit.tsx**: Updated useKitApp hook to use XState via `useKitAppXState`
  - Import added: `import { useKitApp as useKitAppXState } from "./xstate-hooks";`
- **Type.tsx**: Updated useTypeApp hook to use XState via `useTypeAppXState`
  - Import added: `import { useTypeApp as useTypeAppXState } from "./xstate-hooks";`
- **Design.tsx**: Updated useDesignApp hook to use XState via `useDesignAppXState`
  - Import added: `import { useDesignApp as useDesignAppXState } from "./xstate-hooks";`

### Phase 3: Remove "store" from App Files (In Progress)

**Completed:**

- **Home.tsx**: All "store" references removed
  - Renamed `KitStoreKind` to `KitKind`
  - Removed `useSketchpadStore` import
  - Added `getKitSnapshot` and `storeKitFileBlobs` commands to useSketchpadCommands
  - Updated all usages to use new commands

**Remaining:**

- **Kit.tsx**: Contains ~700 lines of KitAppStore class + hooks - needs migration to Sketchpad.tsx
- **Type.tsx**: Contains ~500 lines of TypeAppStore class + hooks - needs migration to Sketchpad.tsx
- **Design.tsx**: Contains ~600 lines of DesignAppStore class + hooks - needs migration to Sketchpad.tsx

### Migration Path for Remaining Files

1. Move \*AppStore classes to Sketchpad.tsx (the orchestrator)
2. Export types/interfaces from Sketchpad.tsx
3. Update xstate-hooks.ts with equivalent hooks using XState selectors
4. Update app files to import from xstate-hooks.ts and Sketchpad.tsx
5. Remove all "store" references from variable names

### Architecture

- XState machine is independent from React - pure logic in machines.ts
- xstate-hooks.ts provides React bindings using @xstate/react's useSelector
- UI state (selection, hover, panel visibility, etc.) is managed by XState
- Kit data (types, designs, etc.) is still managed by Yjs via KitStore
