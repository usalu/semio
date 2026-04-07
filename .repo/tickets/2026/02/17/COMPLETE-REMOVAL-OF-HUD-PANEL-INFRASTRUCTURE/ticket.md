---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Completely removing all HUD Panel infrastructure from the codebase. While a previous ticket (2026/02/16/REMOVE-HUD-PANEL-FROM-SKETCHPAD-INTERFACE) removed PanelKind.HUD and some panel definitions, many HUD Panel components, types, hooks, tests, and stories remain in the code. This ticket will remove all remaining HUD Panel references.

## Changes

- Remove HudPanel component and HudPanelTabConfig interface from elements.tsx
- Remove HudPanel-related types from shared.ts (HudPanelTab, HudPanelVisibility, HudPanelTabs, etc.)
- Remove "hudPanel" from PanelKey type in shared.ts
- Remove hudPanel from PanelSections interface in shared.ts
- Remove hudPanel from PanelVisibility interface in shared.ts
- Remove hudPanelWidth from PanelSizing interface in shared.ts
- Remove HudPanelTabsState interface from Sketchpad.tsx
- Remove hudPanelTabs state and related context from SidePanelTabProvider
- Remove HudPanel hooks (useHudPanelTabs, useAddHudPanelTab, useRemoveHudPanelTab) from Sketchpad.tsx
- Remove HudPanel tests from sketchpad.test.ts
- Remove HudPanel stories from Layout.stories.tsx
- Remove hudPanel prop from LayoutProps interface in elements.tsx
- Remove hudPanel rendering logic from Layout component in elements.tsx
- Remove hudPanel from EMPTY_PANEL_VISIBILITY in Sketchpad.tsx

## Log

Starting complete removal of HUD Panel infrastructure...

## Todos

- [x] Update ticket with plan and todos
- [ ] Remove HudPanel component from elements.tsx
- [ ] Remove HudPanel types from shared.ts
- [ ] Remove HudPanel context and hooks from Sketchpad.tsx
- [ ] Remove HudPanel tests from sketchpad.test.ts
- [ ] Remove HudPanel stories from Layout.stories.tsx
- [ ] Remove hudPanel from PanelKey type
- [ ] Remove hudPanel from Layout props
- [ ] Verify and fix any compilation errors
- [ ] Close ticket

## Plan

1. Remove HudPanel component section from elements.tsx (including HudPanelTabConfig and HudPanelProps interfaces)
2. Remove HudPanel from Layout component rendering logic in elements.tsx
3. Remove hudPanel prop from LayoutProps interface in elements.tsx
4. Remove HudPanelTab interface from shared.ts
5. Remove HudPanelVisibility interface from shared.ts
6. Remove HudPanelTabs interface from shared.ts
7. Remove "hudPanel" from PanelKey type in shared.ts
8. Remove hudPanel from PanelSections interface in shared.ts
9. Remove hudPanel from PanelVisibility interface in shared.ts
10. Remove hudPanelWidth from PanelSizing interface in shared.ts
11. Remove HudPanelTabsState interface from Sketchpad.tsx
12. Remove hudPanelTabs state from SidePanelTabProvider in Sketchpad.tsx
13. Remove addHudPanelTab and removeHudPanelTab functions from SidePanelTabProvider
14. Remove activeHudTabId state and setter from SidePanelTabProvider
15. Remove HudPanel-related hooks (useHudPanelTabs, useAddHudPanelTab, useRemoveHudPanelTab)
16. Remove HudPanel tests from sketchpad.test.ts (PANEL_GROUPS mappings, toggle verification, panel visibility checks)
17. Remove HudPanel story from Layout.stories.tsx
18. Remove hudPanel from EMPTY_PANEL_VISIBILITY constant in Sketchpad.tsx
19. Run TypeScript compilation checks
20. Close ticket
