# Restore Sketchpad Workbench Functionalities

## Goal

SKETCHPAD-IMPROVEMENTS

## Status

CLOSED

## Plan

Restore all missing sketchpad workbench functionalities. The migration from direct panels to tabbed side panels removed several features:

1. **PanelVisibility missing side panel flags** - `leftSidePanel: true` and `rightSidePanel: true` never set in INIT events across all apps.
2. **Type.tsx missing WORKBENCH panel definition** - `PanelKind.WORKBENCH` was removed from Type app's `getPanels`.
3. **Kit.tsx panelVisibility Y.js getter incomplete** - Only deserialized `toolbar` and `details`, dropping `leftSidePanel`, `rightSidePanel`, `chat`, `settings`.
4. **Core defaults incomplete** - `defaultPanelVisibility` and `EMPTY_PANEL_VISIBILITY` lacked side panel flags.

## TODOs

- [x] Add `leftSidePanel: true` to Design app panelVisibility defaults (3 locations)
- [x] Add `leftSidePanel: true` to Quality app panelVisibility defaults (2 locations)
- [x] Add `leftSidePanel: true` to Docs app panelVisibility defaults (2 locations)
- [x] Add `rightSidePanel: true` to Kit app panelVisibility defaults (2 locations)
- [x] Fix Kit.tsx panelVisibility Y.js getter to read all fields
- [x] Restore PanelKind.WORKBENCH to Type.tsx getPanels
- [x] Fix Type.tsx panelVisibility defaults (2 locations)
- [x] Update defaultPanelVisibility in Sketchpad.tsx
- [x] Update EMPTY_PANEL_VISIBILITY in shared.ts
- [x] Verify TypeScript compilation (build succeeded)
- [x] Run tests (14/14 unit tests pass, 42/42 engine tests pass)

## Changes

- `compose/js/sketchpad/Design.tsx`: Added `leftSidePanel: true` to panelVisibility in constructor, createDefaultState, and INIT event
- `compose/js/sketchpad/Type.tsx`: Restored `PanelKind.WORKBENCH` to getPanels, updated panelVisibility in createDefaultState and INIT
- `compose/js/sketchpad/Quality.tsx`: Updated panelVisibility in constructor and createDefaultState
- `compose/js/sketchpad/Docs.tsx`: Updated panelVisibility in constructor and createDefaultState
- `compose/js/sketchpad/Kit.tsx`: Updated panelVisibility in createDefaultState and useKitApp default, fixed Y.js getter to read all panelVisibility fields
- `compose/js/sketchpad/Sketchpad.tsx`: Updated defaultPanelVisibility to include side panel flags
- `compose/js/sketchpad/shared.ts`: Updated EMPTY_PANEL_VISIBILITY to include side panel flags

## Summary

Restored all missing sketchpad workbench functionalities by enabling side panel visibility flags across all 7 apps. The migration from direct panels to tabbed side panels had left `leftSidePanel` and `rightSidePanel` flags unset, hiding the workbench/tools tabs. Also restored `PanelKind.WORKBENCH` to Type app and fixed Kit app's Y.js panelVisibility getter. Build compiles cleanly, all 14 unit tests and 42 engine tests pass.
