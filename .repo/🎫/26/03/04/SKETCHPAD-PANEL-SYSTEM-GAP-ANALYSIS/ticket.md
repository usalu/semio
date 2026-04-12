---
goal: SKETCHPAD/PANELSYSTEM
---

# Ticket

## Summary

Completed comprehensive gap analysis between old (origin/ueli/latest) and current (HEAD) sketchpad panel/workbench system. Documented all differences in PanelSizes, PanelVisibility, PanelKind, panelKindConfigs, getPanels, defaults, navbar toggles, and layout rendering.
## Plan

Research-only ticket: compare old vs current panel system across all dimensions.

## Todos

- [x] Compare PanelSizes interface
- [x] Compare PanelVisibility interface
- [x] Compare PanelKind enum
- [x] Compare panelKindConfigs
- [x] Compare EMPTY_PANEL_VISIBILITY
- [x] Compare getPanels for all apps
- [x] Compare panelVisibility defaults for all apps
- [x] Compare navbar PanelToggles
- [x] Compare LayoutComponent leftPanel rendering
- [x] Write findings report

## Changes

No code changes (research-only).

## Log

### Findings

---

## 1. PanelSizes Interface

### OLD (origin/ueli/latest)
```ts
interface PanelSizes {
  toolbarHeight: number;
  workbenchWidth: number;   // <-- REMOVED in HEAD
  toolsWidth: number;
  hudWidth: number;
  statsWidth: number;
  detailsWidth: number;
  chatWidth: number;        // order different
  settingsWidth: number;    // order different
  consoleHeight: number;
  leftSidePanelWidth: number;
  rightSidePanelWidth: number;
  hudPanelWidth: number;    // <-- REMOVED in HEAD
}
```

### CURRENT (HEAD)
```ts
interface PanelSizes {
  toolbarHeight: number;
  toolsWidth: number;
  hudWidth: number;
  statsWidth: number;
  detailsWidth: number;
  consoleHeight: number;
  leftSidePanelWidth: number;
  rightSidePanelWidth: number;
  chatWidth: number;
  settingsWidth: number;
}
```

### Gaps
- **MISSING `workbenchWidth`**: Old code had separate `workbenchWidth: 230` for the workbench panel. Current code removed it. The old LayoutComponent used `panelSizes.workbenchWidth` when `panelVisibility.workbench` was true. Now workbench content goes into leftSidePanel tabs, so `leftSidePanelWidth` covers it. This is a valid simplification IF the leftSidePanel properly contains workbench content.
- **MISSING `hudPanelWidth`**: Old code had `hudPanelWidth: 400` for the floating HUD panel. Current code removed the entire HUD panel concept. The old HUD panel was a separate overlay panel with tabs (like a floating inspector). No replacement exists.

---

## 2. PanelVisibility Interface

### OLD
```ts
interface PanelVisibility {
  toolbar?: boolean;
  leftSidePanel?: boolean;
  rightSidePanel?: boolean;
  hudPanel?: boolean;       // <-- REMOVED
  workbench?: boolean;      // <-- REMOVED
  tools?: boolean;
  hud?: boolean;
  stats?: boolean;
  details?: boolean;
  chat?: boolean;
  settings?: boolean;
  params?: boolean;
  console?: boolean;
}
```

### CURRENT
```ts
interface PanelVisibility {
  toolbar?: boolean;
  leftSidePanel?: boolean;
  rightSidePanel?: boolean;
  tools?: boolean;
  hud?: boolean;
  stats?: boolean;
  details?: boolean;
  params?: boolean;
  console?: boolean;
  chat?: boolean;
  settings?: boolean;
}
```

### Gaps
- **MISSING `hudPanel`**: Old code had `hudPanel` for controlling the floating HUD overlay. Entirely removed.
- **MISSING `workbench`**: Old code had `workbench` for workbench panel visibility. Now absorbed into `leftSidePanel`.

---

## 3. PanelKind Enum

### OLD
```ts
enum PanelKind {
  WORKBENCH, TOOLS, TOOLBAR, HUD, STATS, DETAILS, CHAT, SETTINGS, PARAMS, CONSOLE
}
```

### CURRENT
```ts
enum PanelKind {
  WORKBENCH, TOOLS, TOOLBAR, STATS, DETAILS, PARAMS, CONSOLE
}
```

### Gaps
- **MISSING `HUD`**: Old had `PanelKind.HUD = "hud"`. Removed. Apps used to register HUD panels.
- **MISSING `CHAT`**: Old had `PanelKind.CHAT = "chat"`. Removed from enum. Chat handling is now hardcoded in the rightSidePanel of LayoutComponent.
- **MISSING `SETTINGS`**: Old had `PanelKind.SETTINGS = "settings"`. Removed from enum. Settings handling is now hardcoded in the rightSidePanel of LayoutComponent.

---

## 4. panelKindConfigs

### OLD
```ts
panelKindConfigs = {
  WORKBENCH:  { icon: WorkbenchIcon, position: LEFT,   group: "workbench", groupable, hotkey: "ctrl+j" },
  TOOLS:      { icon: ToolsIcon,    position: LEFT,   group: "workbench", groupable, hotkey: "ctrl+j" },
  TOOLBAR:    { icon: ToolbarIcon,  position: BOTTOM },
  HUD:        { icon: HudIcon,      position: MIDDLE, group: "hud", groupable, transparent, hotkey: "ctrl+k" },
  STATS:      { icon: StatsIcon,    position: MIDDLE, group: "hud", groupable, transparent, hotkey: "ctrl+k" },
  DETAILS:    { icon: DetailsIcon,  position: RIGHT,  group: "right", groupable, hotkey: "ctrl+l" },
  CHAT:       { icon: ChatIcon,     position: RIGHT,  group: "right", groupable, hotkey: "ctrl+l" },
  SETTINGS:   { icon: SettingsIcon, position: RIGHT,  group: "right", groupable, hotkey: "ctrl+l" },
  PARAMS:     { icon: SettingsIcon, position: RIGHT,  group: "right", groupable, hotkey: "ctrl+l" },
  CONSOLE:    { icon: CodeIcon,     position: BOTTOM, hotkey: "ctrl+k" },
}
```

### CURRENT
```ts
panelKindConfigs = {
  WORKBENCH:  { icon: WorkbenchIcon, position: LEFT,   group: "left", groupable },       // group changed from "workbench" to "left", hotkey removed
  TOOLS:      { icon: ToolsIcon,    position: LEFT,   group: "left", groupable, hotkey: "ctrl+j" },  // group changed from "workbench" to "left"
  TOOLBAR:    { icon: ToolbarIcon,  position: BOTTOM },
  STATS:      { icon: StatsIcon,    position: MIDDLE, group: "hud", groupable, transparent, hotkey: "ctrl+k" },
  DETAILS:    { icon: DetailsIcon,  position: RIGHT,  group: "right", groupable, hotkey: "ctrl+l" },
  PARAMS:     { icon: SettingsIcon, position: RIGHT,  group: "right", groupable, hotkey: "ctrl+l" },
  CONSOLE:    { icon: CodeIcon,     position: BOTTOM, hotkey: "ctrl+k" },
}
```

### Gaps
- **MISSING HUD config**: No `PanelKind.HUD` config. Old had HUD as a MIDDLE panel with transparency.
- **MISSING CHAT config**: No `PanelKind.CHAT` config. Chat is hardcoded in Sketchpad.tsx rightSidePanel.
- **MISSING SETTINGS config**: No `PanelKind.SETTINGS` config. Settings is hardcoded in Sketchpad.tsx rightSidePanel.
- **Group changed**: WORKBENCH and TOOLS changed from group `"workbench"` to `"left"`. This means they now share the leftSidePanel tab group instead of being a separate "workbench" group.
- **Hotkey removed from WORKBENCH**: Old had `hotkey: "ctrl+j"` on WORKBENCH, current has none.

---

## 5. EMPTY_PANEL_VISIBILITY

### OLD
```ts
EMPTY_PANEL_VISIBILITY = { toolbar: true, workbench: false, details: false, chat: false, settings: false }
```

### CURRENT
```ts
EMPTY_PANEL_VISIBILITY = { toolbar: true, details: false, chat: false, settings: false }
```

### Gap
- **MISSING `workbench: false`**: Old explicitly set `workbench: false`. Current omits it since `workbench` is no longer in PanelVisibility.

---

## 6. getPanels per App (Panel Definitions)

| App | OLD getPanels | CURRENT getPanels | Missing |
|-----|-------------|-----------------|---------|
| **Design** | WORKBENCH, TOOLS, TOOLBAR, HUD, STATS, DETAILS, CHAT, SETTINGS | WORKBENCH, TOOLS, TOOLBAR, STATS, DETAILS | **HUD, CHAT, SETTINGS** |
| **Kit** | TOOLBAR, DETAILS, CHAT, SETTINGS | TOOLBAR, DETAILS | **CHAT, SETTINGS** |
| **Type** | WORKBENCH, TOOLS, TOOLBAR, HUD, STATS, DETAILS, CHAT, SETTINGS | TOOLS, TOOLBAR, STATS, DETAILS | **WORKBENCH, HUD, CHAT, SETTINGS** |
| **Quality** | WORKBENCH, TOOLS, TOOLBAR, HUD, STATS, DETAILS, CHAT, SETTINGS | WORKBENCH, TOOLS, TOOLBAR, STATS, DETAILS | **HUD, CHAT, SETTINGS** |
| **Docs** | WORKBENCH, DETAILS, SETTINGS | WORKBENCH, DETAILS | **SETTINGS** |
| **Home** | TOOLBAR, DETAILS, CHAT, SETTINGS | TOOLBAR, DETAILS | **CHAT, SETTINGS** |
| **Feedback** | TOOLBAR | TOOLBAR | (none) |

### Key Observations
- **HUD removed everywhere**: No app registers HUD panels anymore.
- **CHAT removed everywhere**: Chat/Settings are no longer registered as app panels. Instead they're hardcoded as dedicated buttons in PanelToggles navbar and rendered directly in the rightSidePanel of LayoutComponent.
- **SETTINGS removed everywhere**: Same as CHAT.
- **Type lost WORKBENCH**: Type used to have WORKBENCH but current only has TOOLS. This means Type's left panel won't show the workbench navigation tab.

---

## 7. panelVisibility Defaults per App

| App | OLD Default | CURRENT Default | Difference |
|-----|------------|----------------|------------|
| **Design INIT** | `{ toolbar: true, workbench: false, details: true, chat: false, settings: false }` | `{ toolbar: true, details: true, rightSidePanel: true }` | Added `rightSidePanel: true`, removed `workbench`, `chat`, `settings` |
| **Design DEFAULT_PANEL_VISIBILITY** | N/A | `{ toolbar: false, details: true, rightSidePanel: true }` | New constant, not in old |
| **Kit** | `{ toolbar: true, workbench: false, details: false, chat: false, settings: false }` | `{ toolbar: true, details: false }` | Removed `workbench`, `chat`, `settings` |
| **Type** | N/A (uses EMPTY) | Uses `EMPTY_PANEL_VISIBILITY` + INIT: `{ toolbar: true, details: false }` | Same approach |
| **Quality** | `{ toolbar: false, workbench: false, details: false, chat: false, settings: false }` (initial) / `{ toolbar: true, workbench: false, details: false, chat: false, settings: false }` (registered) | `{ toolbar: false, details: false }` / `{ toolbar: true, details: false }` | Removed `workbench`, `chat`, `settings` |
| **Docs** | `{ toolbar: false, workbench: false, details: false, chat: false, settings: false }` | `{ toolbar: false, details: false }` | Removed `workbench`, `chat`, `settings` |
| **Home** | `{ ...EMPTY_PANEL_VISIBILITY }` | `{ ...EMPTY_PANEL_VISIBILITY }` | Same (but EMPTY changed) |
| **Feedback** | `{ ...EMPTY_PANEL_VISIBILITY }` | `{ ...EMPTY_PANEL_VISIBILITY }` | Same (but EMPTY changed) |

### Key Observations
- All apps removed `workbench`, `chat`, `settings` from their defaults since those fields are gone from PanelVisibility or handled globally.
- Design added `rightSidePanel: true` to show the right panel by default (replaces the old `details: true` auto-showing mechanism).
- No apps set `leftSidePanel` in their defaults, meaning the left side panel starts hidden everywhere.

---

## 8. Navbar PanelToggles

### OLD
- Three toggle buttons: LeftSidePanel, HudPanel, RightSidePanel
- Each toggles the corresponding `panelVisibility` flag
- HudPanel had its own toggle
- rightSidePanel was a single toggle (no separate chat/settings)

### CURRENT
- Four toggle buttons: LeftSidePanel, RightSidePanel, Settings, Chat
- Left toggle works the same as old
- Right side has three mutually-exclusive modes: rightSidePanel (details tabs), settings, chat
- When activating one right mode, the others are deactivated
- **HUD toggle completely removed from navbar**

### Gaps
- **No HUD toggle**: The floating HUD panel toggle is gone from the navbar.
- **New chat/settings toggles**: These replace the old rightSidePanel-only toggle. They're separate buttons that mutually exclude each other and the generic right panel.

---

## 9. LayoutComponent Rendering

### OLD leftPanel Logic
```
leftPanel = leftSidePanel || workbench || tools
  → size: tools ? toolsWidth : workbenchWidth
  → sections: tools ? toolsSections : workbenchSections
  → panelKey: tools ? "tools" : "workbench"
```

### CURRENT leftSidePanel Logic
```
leftSidePanel = leftSidePanelTabs.length > 0 && panelVisibility.leftSidePanel
  → tabs-based (from useSidePanelTabs("left"))
  → size: leftSidePanelWidth (single width for all left tabs)
```

### Key Differences
- **Old was sections-based, current is tabs-based**: Old rendered either workbench or tools sections directly. Current uses side panel tabs populated by the useEffect that maps panelConfigs.
- **Old had separate workbenchWidth/toolsWidth sizing**: Old dynamically chose between workbenchWidth and toolsWidth. Current uses single leftSidePanelWidth for all left panel content.
- **Old directly checked `panelVisibility.workbench || panelVisibility.tools`**: The old code OR'd workbench/tools/leftSidePanel. Current only checks `panelVisibility.leftSidePanel`. This means:
  - If an app toggles `workbench` it won't show the left panel in current code (workbench is not in PanelVisibility)
  - The leftSidePanel visibility must be explicitly toggled

### OLD middlePanel Logic
```
middlePanel = hudPanel || hud || stats
  → size: stats ? statsWidth : hudWidth
  → sections: stats ? statsSections : hudSections
```

### CURRENT
- **No middlePanel at all in LayoutComponent rendering**. The old had a middlePanel for HUD/stats. Current code has no equivalent. Stats sections exist but are never rendered in a middle panel.

### OLD hudPanel Logic
```
hudPanel = hudPanelTabs.length > 0 && panelVisibility.hudPanel
  → tabs-based floating panel
  → size: hudPanelWidth
```

### CURRENT
- **No hudPanel at all**. Completely removed.

### OLD rightPanel Logic
```
rightPanel = rightSidePanel || details || chat || settings
  → priority: chat > settings > details
  → size: chat ? chatWidth : settings ? settingsWidth : detailsWidth
  → sections-based
```

### CURRENT rightSidePanel Logic
```
rightSidePanel = chat ? {chat tabs} : settings ? {settings tabs} : rightSidePanel ? {right tabs} : undefined
  → priority: chat > settings > rightSidePanel
  → tabs-based
```
- Similar logic but tabs-based instead of sections-based.

---

## 10. Overall Summary of Functional Gaps

### Removed Features (No Replacement)
1. **HUD Panel (PanelKind.HUD)**: Entire floating HUD overlay with tabs removed. No middle panel rendering. No navbar toggle.
2. **HUD Panel Width (hudPanelWidth)**: Gone from PanelSizes.
3. **Workbench Width (workbenchWidth)**: Gone from PanelSizes. Merged into leftSidePanelWidth.

### Removed but Refactored (Chat/Settings)
4. **PanelKind.CHAT & PanelKind.SETTINGS**: No longer registered as app panels. Instead hardcoded in Sketchpad.tsx. Apps don't register them in getPanels. Chat/Settings toggles are in navbar and render in rightSidePanel.

### Architectural Changes
5. **Sections → Tabs**: Old used sections-based panels (workbench/tools switching). Current uses tabs-based side panels where multiple panel kinds share the same side panel as tabs.
6. **workbench visibility → leftSidePanel visibility**: Old toggled `panelVisibility.workbench` directly. Current uses `panelVisibility.leftSidePanel` as the master toggle for leftSidePanel.
7. **Workbench group renamed**: panelKindConfigs changed group from `"workbench"` to `"left"`.

### Missing Panel Registrations per App
- **Type**: Missing WORKBENCH in getPanels (old had it).
- **All apps**: Missing HUD, CHAT, SETTINGS in getPanels (by design for chat/settings, but HUD is a functional gap).

### The "workbench" Toggle Translation
The old navbar had a separate workbench toggle button. The old code checked `panelVisibility.workbench || panelVisibility.tools || panelVisibility.leftSidePanel` to show the left panel. The current code only has a single leftSidePanel toggle in the navbar. When pressed, it sets `panelVisibility.leftSidePanel`. The workbench/tools content is registered as tabs in the leftSidePanel via the useEffect. This is functionally equivalent but:
- The old workbench toggle was tied to the `workbench` PanelKind visibility
- The current toggle is a generic leftSidePanel toggle
- The workbench content tab is always registered (if the app includes WORKBENCH in getPanels) but might not auto-select

### Design.tsx rightSidePanel Default
Old Design defaulted to `{ details: true, rightSidePanel: false }`. Current defaults to `{ details: true, rightSidePanel: true }`. This means the right side panel is visible by default in the new code for Design, which is a behavior change.
