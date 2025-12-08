---
date: "2025-12-08T15:43:43.165Z"
slug: STATE-MACHINE-REFACTOR
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Refactor sketchpad state machine for proper state transitions
model: claude-opus-4.5
---

# Previously

The codebase has a dual state management system:

1. **Y.js stores** - For persistence and synchronization of kit data
2. **XState machine** - For app state (selection, hover, panel visibility, etc.)

Current issues:

- XState machine only uses reflexive transitions (all events just trigger actions, no state transitions)
- Some hooks don't follow the triadic pattern `[state, setState, canSetState]`
- App stores still use Y.js for app state which should be pure XState

Triadic hook pattern already exists in:

- `useTypeAppSelection(): GranularHookResult<TypeAppSelection>`
- `usePieceCenterU(): GranularHookResult<number>`
- `useConnectionGap(): GranularHookResult<number>`

All playwright tests pass (5/5).

# Plan

1. [x] Analyze current state management structure
2. [x] Ensure all sketchpad hooks follow triadic pattern
3. [ ] Verify XState machine handles all app state correctly
4. [ ] Remove Y.js usage from app state (keep only for kit data sync)
5. [x] Run tests to verify nothing breaks

## Current State (Dec 8, 2025)

### Analysis

Current architecture has these layers:

1. **Y.js stores (DesignStore, TypeStore, etc.)** - Store app state in Y.js maps
2. **XState sketchpadMachine** - Has reflexive transitions only (no hierarchical states)
3. **Commands pattern** - `useDesignAppCommands()` returns methods that call `store.execute()`

### Problems

1. App state is in Y.js but should be pure XState
2. State machine has no guards based on state (e.g., can only select after hover)
3. Hooks use `store.execute()` but should use `actor.send()`
4. `canSetState` is hardcoded, not derived from machine's `can()` method

### Target Architecture

1. **Pure XState for app state** - Selection, hover, focus, panel visibility, tools
2. **Y.js only for Kit data** - Types, designs, pieces, connections (collaborative sync)
3. **Hierarchical states** - idle → hovered → selected → contextMenu
4. **Triadic hooks** - `[value, setValue, canSetValue]` where `canSetValue` uses `actor.can()`
5. **No useXCommands** - Components use granular triadic hooks instead

# Changes

## Converted sketchpad hooks to triadic pattern

Updated the following hooks in `Sketchpad.tsx` to return `GranularHookResult<T>` (triadic tuple `[value, setter, canSet]`):

- `useTheme()` → `GranularHookResult<Theme>`
- `useLanguage()` → `GranularHookResult<string>`
- `useLayout()` → `GranularHookResult<Layout>`
- `useMode()` → `GranularHookResult<Mode>`
- `useExpertise()` → `GranularHookResult<Expertise>`
- `useIsFullscreen()` → `GranularHookResult<boolean>`

Each triadic hook:

1. Gets the value from Y.js store via `useSketchpad`
2. Uses `useOrigin()` to get the current origin from context
3. Creates a setter that sends events to both XState actor and Y.js store with the origin
4. Returns `[value, setter, canSet]` as a readonly tuple

## Added OriginProvider/Context pattern

Created in `Sketchpad.tsx`:

- `OriginContext` - React context for tracking command origin
- `OriginProvider` - Wraps children with specified origin id
- `useOrigin()` - Returns current origin from context (defaults to "semio.sketchpad.unknown")
- `useOriginSafe()` - Returns origin or null if not in provider

Components with an `id` should wrap their children in `<OriginProvider id={...}>` to provide origin for triadic hooks.

## Updated all consumers to use destructuring and OriginProvider

Updated all files using these hooks:

- `Home.tsx` - Extracted `ThemeToggle`, `LanguageSelect`, `LayoutToggle`, `ExpertiseToggle`, `ModeToggle` components wrapped with `OriginProvider`
- `Design.tsx` - Added `OriginProvider` import and settings helper components
- `Type.tsx` - Added `OriginProvider` import and settings helper components
- `Kit.tsx` - Added `OriginProvider` import

## Migrated Type.tsx triadic hooks to XState

All Type app triadic hooks now use `actor.send()` instead of Y.js store:

- `useTypeAppSelection()` - Uses `TYPE.SET_SELECTION` event
- `useTypeAppPanelVisibility()` - Uses `TYPE.TOGGLE_PANEL` event
- `useTypeAppCamera()` - Uses `TYPE.SET_CAMERA` event
- `useTypeAppFocusedPortGuid()` - Uses `TYPE.FOCUS_PORT` / `TYPE.CLEAR_FOCUS` events
- `useTypeAppHover()` - Uses `TYPE.HOVER_PORT` / `TYPE.HOVER_MODEL` / `TYPE.CLEAR_HOVER` events
- `useTypeAppActiveTool()` - Uses `TYPE.SET_ACTIVE_TOOL` event
- `useTypeAppFullscreen()` - Uses `TYPE.SET_FULLSCREEN` event
- `useTypeAppSelectedModelTags()` - Uses `TYPE.ADD_MODEL_TAG` / `TYPE.REMOVE_MODEL_TAG` events

Replaced `useTypeAppYjsToXStateSync` with simpler `useTypeAppInitialize` that sends `TYPE.INIT` event with default state.

## Migrated Design.tsx triadic hooks to XState

All Design app triadic hooks now use `actor.send()` instead of Y.js store:

- `useDesignAppSelection()` - Uses `DESIGN.SET_SELECTION` event
- `useDesignAppFullscreen()` - Uses `DESIGN.SET_FULLSCREEN` event
- `useDesignAppActiveTool()` - Uses `DESIGN.SET_ACTIVE_TOOL` event
- `useDesignAppCamera()` - Uses `DESIGN.SET_CAMERA` event
- `useDesignAppDiagramCenter()` - Uses `DESIGN.SET_DIAGRAM_CENTER` event
- `useDesignAppDiagramScale()` - Uses `DESIGN.SET_DIAGRAM_SCALE` event
- `useDesignAppFocusedPieceGuid()` - Uses `DESIGN.FOCUS_PIECE` event
- `useDesignAppSelectedModelTags()` - Uses `DESIGN.SYNC` event (for bulk update)
- `useDesignAppHover()` - Uses `DESIGN.SET_HOVER` / `DESIGN.CLEAR_HOVER` events
- `useDesignAppPanelVisibility()` - Uses `DESIGN.SET_PANEL_VISIBILITY` event

Replaced `useDesignAppYjsToXStateSyncInternal` with simpler `useDesignAppInitialize` that sends `DESIGN.INIT` event with default state.

Removed `useSyncDeep` import from Design.tsx.

## Current Architecture

- **XState**: Source of truth for app state (selection, hover, panels, tools, fullscreen, camera)
- **Y.js (Controllers)**: Persistence layer for undo/redo stacks and kit mutation transactions
- **Triadic hooks**: Read from XState, write via actor.send() events

All 5 playwright tests pass (Home, Kit, Type, Design, Docs).

- `Sketchpad.tsx` - `LayoutWrapper` destructures triadic hooks

All 5 playwright tests pass after the refactor.
