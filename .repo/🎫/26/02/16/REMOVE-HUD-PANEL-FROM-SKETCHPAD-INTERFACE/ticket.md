---
goal: SKETCHPAD/LAYOUT
---

# Ticket

## Summary

Removing the HUD panel from the Sketchpad interface as it will be implemented in the future.

## Changes

- Removed HUD panel definition from Design.tsx getPanels method
- Removed HUD panel definition from Type.tsx getPanels method  
- Removed HUD panel definition from Quality.tsx getPanels method
- Removed PanelKind.HUD from shared.ts enum
- Removed HUD config from panelKindConfigs in shared.ts
- Removed "hud" and "hudPanel" from PanelKey type in shared.ts
- Removed HudPanelKey type definition from shared.ts
- Removed HudIcon import from shared.ts
- Removed HUD panel sections from Design.tsx (hud pieces section)
- Updated test file PANEL_GROUPS to remove HUD panel mappings
- Updated test file to remove HUD panel toggle verification
- Updated README.md documentation to remove HUD references

## Log

Starting removal of HUD panel from Sketchpad interface.

1. Removed PanelKind.HUD panel definitions from Design.tsx, Type.tsx, and Quality.tsx getPanels methods
2. Removed PanelKind.HUD enum value from shared.ts
3. Removed HUD entry from panelKindConfigs in shared.ts
4. Removed "hud" and "hudPanel" from PanelKey type in shared.ts
5. Removed HudPanelKey type definition from shared.ts
6. Removed HudIcon import from shared.ts imports
7. Removed HUD panel sections from Design.tsx (removed hudPieces section with useEffect hook)
8. Updated PANEL_GROUPS in sketchpad.test.ts to remove HUD panel mappings
9. Updated test verification to remove HUD panel toggle checks
10. Updated README.md documentation to remove HUD references from PanelKind enum, PanelVisibility interface, and Panel Positioning section

All changes completed successfully with no TypeScript compilation errors.

## Todos

- [x] Update ticket plan
- [x] Remove HUD panel definitions from app files
- [x] Remove HUD from shared.ts types and configs
- [x] Remove HUD panel sections from Design.tsx
- [x] Update test file to remove HUD panel references
- [x] Update README.md documentation
- [x] Close ticket

## Plan

1. Remove HUD panel definition from Design.tsx getPanels method
2. Remove HUD panel definition from Type.tsx getPanels method
3. Remove HUD panel definition from Quality.tsx getPanels method
4. Remove PanelKind.HUD enum value from shared.ts
5. Remove HUD entry from panelKindConfigs in shared.ts
6. Remove "hud" and "hudPanel" from PanelKey type in shared.ts
7. Remove HudPanelKey type definition from shared.ts
