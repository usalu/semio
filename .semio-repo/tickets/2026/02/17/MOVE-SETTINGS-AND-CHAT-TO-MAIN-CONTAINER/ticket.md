---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Moved Settings and Chat panels from the right side panel tabs into the main canvas container as GoldenLayout windows. Settings and Chat are now rendered as tab components within existing stacks alongside primary content windows (e.g., Table, Scene, Diagram, Page). Removed `CHAT` and `SETTINGS` from `PanelKind` enum and all associated panel infrastructure (PanelVisibility, PanelSizes, PanelSections, panelKindConfigs). Added `SETTINGS` and `CHAT` to `WindowKind` enum. Each app now defines Settings/Chat as `AppWindowKind` entries with proper GoldenLayout single-stack tab layouts.

## Changes

- `semio/js/sketchpad/shared.ts`: Added SETTINGS/CHAT to WindowKind, removed from PanelKind, PanelVisibility, PanelSizes, PanelSections, PanelKey, panelKindConfigs, EMPTY_PANEL_VISIBILITY.
- `semio/js/sketchpad/Sketchpad.tsx`: Removed chat/settings from usePanelSections, sectionsByKind, useEffect deps, default PanelVisibility/PanelSizes/PanelSections states.
- `semio/js/sketchpad/Home.tsx`: Added Settings/Chat to HomeAppWindowKind, windowKinds, and defaultLayout as tabs in a single stack. Removed panel addSection/removeSection for settings/chat.
- `semio/js/sketchpad/Kit.tsx`: Added Settings/Chat to KitAppWindowKind, windowKinds, and defaultLayout as tabs in a single stack. Removed panel addSection/removeSection for settings.
- `semio/js/sketchpad/Type.tsx`: Added Settings/Chat to TypeAppWindowKind, windowKinds, and manual single-stack defaultLayout. Removed panel addSection/removeSection for settings. Removed createDefaultLayout import.
- `semio/js/sketchpad/Design.tsx`: Added Settings/Chat to DesignAppWindowKind, windowKinds, and defaultLayout as tabs alongside Diagram in first stack. Removed panel addSection/removeSection for settings.
- `semio/js/sketchpad/Quality.tsx`: Added Settings/Chat to QualityAppWindowKind, windowKinds, and manual two-stack defaultLayout (Formula 20%, Diagram+Settings+Chat tabs 80%). Removed createDefaultLayout import.
- `semio/js/sketchpad/Docs.tsx`: Added Settings/Chat to DocsAppWindowKind, windowKinds, and manual single-stack defaultLayout. Removed panel addSection/removeSection for settings. Removed createDefaultLayout import.
- `semio/js/sketchpad.test.ts`: Removed chat/settings from PANEL_GROUPS, allPanels, openSettingsPanel and getSettingsSections helpers. Restored hudPanel/hud/stats to PANEL_GROUPS. Fixed intersect/lasso/HUD assertions for Kit, Type, Design tests.

## Log

- Phase 1: Modified shared.ts - enum/interface/constant changes
- Phase 2: Modified Sketchpad.tsx - panel system cleanup
- Phase 3: Modified all app files (Home, Kit, Type, Design, Quality, Docs) - added Settings/Chat as GoldenLayout windows
- Phase 4: Fixed TypeScript imports (Tree, TreeStateProvider)
- Phase 5: Cleaned up PanelVisibility references across all files
- Phase 6: TypeScript compilation clean
- Phase 7: Updated test file (removed panel references, fixed assertions)
- Phase 8-11: Multiple rounds of e2e test fixes (assertion corrections for intersect/lasso/HUD)
- Phase 12: Fixed layout patterns - replaced createDefaultLayout with manual single-stack layouts for Docs, Type, Quality
- Phase 13: All 7 e2e tests passing (Panels failure was transient ERR_CONNECTION_REFUSED)

## Todos

- [x] Modify shared.ts (enums, interfaces, constants)
- [x] Modify Sketchpad.tsx (panel system cleanup)
- [x] Modify Home.tsx (add Settings/Chat as GoldenLayout windows)
- [x] Modify Kit.tsx (add Settings/Chat as GoldenLayout windows)
- [x] Modify Type.tsx (add Settings/Chat as GoldenLayout windows)
- [x] Modify Design.tsx (add Settings/Chat as GoldenLayout windows)
- [x] Modify Quality.tsx (add Settings/Chat as GoldenLayout windows)
- [x] Modify Docs.tsx (add Settings/Chat as GoldenLayout windows)
- [x] Fix TypeScript compilation errors
- [x] Update e2e tests
- [x] Fix layout patterns (single-stack tabs instead of createDefaultLayout)
- [x] Verify all tests pass

## Plan

1. Remove CHAT and SETTINGS from PanelKind and related infrastructure in shared.ts
2. Add SETTINGS and CHAT to WindowKind enum in shared.ts
3. Clean up panel system references in Sketchpad.tsx
4. For each app (Home, Kit, Type, Design, Quality, Docs):
   a. Add Settings/Chat to AppWindowKind enum
   b. Add window definitions to windowKinds array
   c. Add to defaultLayout as tabs within the primary content stack
   d. Remove addSection/removeSection calls for settings/chat panels
   e. Remove PanelVisibility chat/settings references
5. Fix TypeScript compilation errors
6. Update e2e tests to remove panel references for settings/chat
7. Fix GoldenLayout default layouts to use single-stack tab pattern
8. Run full test suite and verify
