---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Migrated Chat and Settings to be accessible as separate toggle buttons in the navbar (next to the right panel toggle). When toggled, they render their content in the same position as the right panel, providing a clean separation between workspace panels (workbench, details) and global controls (settings, chat). **Only one of Settings, Chat, or Right Panel can be active at a time (mutually exclusive).**

## Plan

1. Add chat and settings to PanelVisibility interface ✅
2. Add chatWidth and settingsWidth to PanelSizes interface ✅
3. Add chat/settings visibility to EMPTY_PANEL_VISIBILITY ✅
4. Update default panel sizes to include chat/settings widths ✅
5. Enhance PanelToggles component to add Chat and Settings buttons ✅
6. Import ChatIcon and SettingsIcon in Sketchpad.tsx ✅
7. Modify rightSidePanel rendering logic to prioritize chat/settings when visible ✅
8. Implement mutual exclusivity - only one panel active at a time ✅
9. Keep existing side panel tab registrations from apps (Home, etc.) ✅
10. Verify TypeScript compilation ✅

## Changes

- `compose/js/sketchpad/shared.ts`: Added `chat` and `settings` to `PanelVisibility`, added `chatWidth` and `settingsWidth` to `PanelSizes`, updated `EMPTY_PANEL_VISIBILITY`
- `compose/js/sketchpad/Sketchpad.tsx`:
  - Added default sizes for `chatWidth` and `settingsWidth` (280px each)
  - Updated `PanelToggles` component to add Settings and Chat toggle buttons with handlers
  - **Implemented mutual exclusivity**: When one panel is opened, the other two are automatically closed
  - Imported `ChatIcon` and `SettingsIcon` from assets
  - Modified `rightSidePanel` rendering logic to show chat/settings panels when toggled (overriding normal side panel tabs)
- `compose/js/sketchpad/Home.tsx`:
  - Added `ChatIcon` and `SettingsIcon` imports
  - Added `useAddSidePanelTab` and `useRemoveSidePanelTab` imports
  - Registered settings/chat as side panel tabs (content providers for when buttons are toggled)

## Architecture

The solution implements a **mutually exclusive**, priority-based rendering for the right panel position:

1. **Chat toggled**: Shows chat content only (no tabs), closes Settings and Right Panel
2. **Settings toggled**: Shows settings content only (no tabs), closes Chat and Right Panel
3. **Right panel toggled**: Shows normal side panel with all registered tabs (workbench, details, etc.), closes Chat and Settings
4. **None toggled**: Right panel hidden

Each app registers its settings/chat content as side panel tabs, which are then pulled and rendered by the navbar toggles when activated.

## Mutual Exclusivity Implementation

Each toggle handler checks if the other panels are open and closes them before opening the target panel:

- `handleRightToggle`: Closes chat and settings when opening right panel
- `handleChatToggle`: Closes right panel and settings when opening chat
- `handleSettingsToggle`: Closes right panel and chat when opening settings

This ensures only one panel occupies the right position at any time.

## Log

- Phase 1: Added chat/settings to panel system interfaces and constants
- Phase 2: Updated default panel sizes
- Phase 3: Enhanced PanelToggles component with chat/settings buttons
- Phase 4: Added icon imports
- Phase 5: Implemented conditional rendering logic for rightSidePanel
- Phase 6: Fixed icon import (ChatIcon not MessageSquareIcon)
- Phase 7: Verified TypeScript compilation - clean build
- Phase 8: Implemented mutual exclusivity in toggle handlers
- Phase 9: Verified TypeScript compilation - clean build

## Todos

- [x] Add chat/settings to PanelVisibility interface
- [x] Add chatWidth/settingsWidth to PanelSizes
- [x] Update EMPTY_PANEL_VISIBILITY
- [x] Update default panel sizes
- [x] Add Settings/Chat toggle buttons to navbar
- [x] Import ChatIcon and SettingsIcon
- [x] Implement rightSidePanel rendering logic
- [x] Implement mutual exclusivity between panels
- [x] Verify TypeScript compilation
- [x] Update ticket documentation
