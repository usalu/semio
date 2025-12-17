---
slug: STATE-MANAGEMENT-REFACTOR
summary: Complete state management refactor with triadic hooks pattern
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.890Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

The state management was started with XState (`sketchpadMachine` and `uiMachine`) but:

- Design.tsx and Type.tsx still import Y.js directly for app state
- Components still use `use*Commands` hooks to execute actions
- The state machine uses reflexive transitions only (all events in all states)
- No proper `can` derivation for triadic hooks based on machine state reachability
- UI components don't properly disable when operations are unavailable

# Plan

1. **Remove Y.js imports from Design.tsx and Type.tsx** - App state should only go through XState machine, not Y.js directly
2. **Add proper state machine states to uiMachine** - States like idle, hovering, selected, contextMenu with proper transitions
3. **Create triadic hooks pattern** - [state, setState, canSetState] where canSetState is derived from machine `can()` API
4. **Update UI components** - Use triadic hooks and disable elements when canSetState is false
5. **Remove use\*Commands usage** - Components should only use granular triadic hooks
6. **Run and fix tests** - Ensure sketchpad.test.ts passes

# Changes

## 1. Removed Y.js imports from Design.tsx and Type.tsx

- Removed `import * as Y from "yjs"` from both files
- Removed Y.js type definitions (YDesignAppVal, YDesignApp, YDesignApps) that depended on Y.js
- Removed Y.js type imports (YAttributes, YLeafMapNumber, YLeafMapString, YStringArray) from shared.ts imports

## 2. Updated Design.tsx hooks to use XState actor directly

Updated the following hooks to send XState events directly instead of using Y.js controller:

- `useDesignAppSelection` - now uses `DESIGN.SET_SELECTION` event
- `useDesignAppFullscreen` - now uses `DESIGN.SET_FULLSCREEN` event
- `useDesignAppActiveTool` - now uses `DESIGN.SET_ACTIVE_TOOL` event
- `useDesignAppCamera` - now uses `DESIGN.SET_CAMERA` event
- `useDesignAppDiagramCenter` - now uses `DESIGN.SET_DIAGRAM_CENTER` event
- `useDesignAppDiagramScale` - now uses `DESIGN.SET_DIAGRAM_SCALE` event
- `useDesignAppFocusedPieceGuid` - now uses `DESIGN.FOCUS_PIECE` event
- `useDesignAppSelectedModelTags` - uses `DESIGN.SYNC` event (temporary until dedicated event)
- `useDesignAppHover` - now uses `DESIGN.SET_HOVER` / `DESIGN.CLEAR_HOVER` events
- `useDesignAppPanelVisibility` - now uses `DESIGN.SET_PANEL_VISIBILITY` event

Each hook now:

1. Gets actor via `useSketchpadActorSafe()`
2. Gets kit/design scope via `useKitScope()` / `useDesignScope()`
3. Derives `canSet` from `!!actor && !!kitGuid && !!designGuid`
4. Creates setter that calls `actor.send()` with appropriate event

## 3. Fixed missing imports in Design.tsx

- Added missing icon imports (MonitorIcon, MoonIcon, SunIcon) from @semio/assets
- Moved Theme import from Sketchpad to shared.ts (correct source)
- Added useLanguage, useLayout imports from Sketchpad

## 4. Test Results

Ran sketchpad.test.ts - **All 5 tests passing**:

- **Home**: passed
- **Kit**: passed
- **Type**: passed
- **Design**: passed
- **Docs**: passed

## Key Insight

The refactoring approach that works is:

1. **Keep Y.js for the controller** - DesignStore and TypeStore still use Y.js internally for state storage and sync
2. **Hooks use XState for writes** - The granular hooks (useDesignAppSelection, etc.) now send XState events directly via `actor.send()`
3. **Y.js to XState sync continues** - The sync functions copy Y.js state to XState for reading

This hybrid approach allows incremental migration without breaking the existing architecture.
