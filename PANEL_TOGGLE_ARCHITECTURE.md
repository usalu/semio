# Panel Toggle Architecture: Complete Connection Flow

## Overview
This document explains how left and right panel toggles connect from UI button clicks through state management to panel rendering.

---

## 📊 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           USER INTERACTION LAYER                            │
│                                                                             │
│  Navbar → [Left Toggle] [HUD Toggle] [Right Toggle]   (PanelToggles)      │
│                    ↓           ↓            ↓                               │
└────────────────────────────────────────────────────────────────────────────┘
                         ↓           ↓            ↓
┌─────────────────────────────────────────────────────────────────────────────┐
│                         COMMAND EXECUTION LAYER                             │
│                                                                             │
│  appCommands.togglePanel(origin, panelKey)                                 │
│      → App-specific XState action (e.g., useTypeAppTogglePanel)           │
│      → Flips boolean in panelVisibility: { leftSidePanel: !value }        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                    ↓
┌─────────────────────────────────────────────────────────────────────────────┐
│                          STATE MANAGEMENT LAYER                             │
│                                                                             │
│  ┌──────────────────────────────┐  ┌──────────────────────────────────┐   │
│  │   SidePanelTabContext (1)   │  │   App State (2)                  │   │
│  │   ─────────────────────────  │  │   ──────────────────────────     │   │
│  │   • left: SidePanelTab[]    │  │   • panelVisibility: {           │   │
│  │   • right: SidePanelTab[]   │  │       leftSidePanel: boolean     │   │
│  │   • addSidePanelTab()       │  │       rightSidePanel: boolean    │   │
│  │   • activeLeftTabId         │  │       hudPanel: boolean          │   │
│  │   • activeRightTabId        │  │       workbench: boolean         │   │
│  │                             │  │       details: boolean           │   │
│  └──────────────────────────────┘  │       chat: boolean              │   │
│                                    │       settings: boolean          │   │
│                                    │     }                            │   │
│                                    │   • panelSizes: {                │   │
│                                    │       leftSidePanelWidth: 280    │   │
│                                    │       rightSidePanelWidth: 280   │   │
│                                    │     }                            │   │
│                                    └──────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
                                    ↓
┌─────────────────────────────────────────────────────────────────────────────┐
│                         REGISTRATION LAYER (LayoutWrapper)                  │
│                                                                             │
│  useEffect(() => {                                                          │
│    // For each app (Home, Kit, Type, Design, etc.)                         │
│    const panels = appConfig.getPanels()  // e.g., [WORKBENCH, DETAILS]    │
│                                                                             │
│    panels.forEach(panel => {                                               │
│      const config = panelKindConfigs[panel.kind]  // Get position (LEFT/RIGHT) │
│      const sections = sectionsByKind[panel.kind]  // Get content           │
│                                                                             │
│      const tab = {                                                          │
│        id: panel.id,               // e.g., "semio.sketchpad.navbar.panelToggle.details.show" │
│        icon: config.icon,          // e.g., DocumentIcon                   │
│        content: () => <PanelTabContent sections={sections} />              │
│      }                                                                      │
│                                                                             │
│      if (config.position === LEFT)  addSidePanelTab("left", tab)          │
│      if (config.position === RIGHT) addSidePanelTab("right", tab)         │
│    })                                                                       │
│  }, [appType, sections...])                                                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                    ↓
┌─────────────────────────────────────────────────────────────────────────────┐
│                            RENDERING LAYER                                  │
│                                                                             │
│  <Layout                                                                    │
│    leftSidePanel={                                                          │
│      leftSidePanelTabs.length > 0 && panelVisibility.leftSidePanel         │
│        ? {                                                                  │
│            position: "left",                                                │
│            visible: true,                                                   │
│            size: panelSizes.leftSidePanelWidth,                            │
│            tabs: leftSidePanelTabs,        // From context                 │
│            activeTabId: activeLeftTabId,   // From context                 │
│            onActiveTabChange: setActiveLeftTabId                           │
│          }                                                                  │
│        : undefined                                                          │
│    }                                                                        │
│    rightSidePanel={                                                         │
│      rightSidePanelTabs.length > 0 && panelVisibility.rightSidePanel       │
│        ? {                                                                  │
│            position: "right",                                               │
│            visible: true,                                                   │
│            size: panelSizes.rightSidePanelWidth,                           │
│            tabs: rightSidePanelTabs,       // From context                 │
│            activeTabId: activeRightTabId,  // From context                 │
│            onActiveTabChange: setActiveRightTabId                          │
│          }                                                                  │
│        : undefined                                                          │
│    }                                                                        │
│  />                                                                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                    ↓
┌─────────────────────────────────────────────────────────────────────────────┐
│                        PANEL COMPONENT LAYER                                │
│                                                                             │
│  <SidePanel position="left|right" tabs={...}>                              │
│    • Renders tab bar at top with icons                                     │
│    • Shows active tab's content in scrollable area                         │
│    • Provides resize handle on inner edge                                  │
│    • Absolutely positioned with z-index                                    │
│  </SidePanel>                                                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 🔄 Complete Data Flow: Click to Render

### Step 1: User Clicks Toggle Button
```tsx
// File: Sketchpad.tsx (PanelToggles component)
<Toggle 
  id="semio.sketchpad.navbar.panelToggle.rightSidePanel"
  pressed={isRightOpen}  // From: panelVisibility.rightSidePanel
  onPressedChange={handleRightToggle}
>
  <DocumentIcon />
</Toggle>
```

### Step 2: Toggle Handler Executes Command
```tsx
// File: Sketchpad.tsx (PanelToggles component)
const handleRightToggle = useCallback((pressed: boolean) => {
  appCommands?.togglePanel?.(
    "semio.sketchpad.navbar.panelToggle.rightSidePanel",  // origin
    "rightSidePanel"                                       // panelKey
  );
}, [appCommands]);
```

### Step 3: App-Specific Action Flips State
```tsx
// File: Type.tsx (or Home.tsx, Design.tsx, etc.)
export function useTypeAppTogglePanel(): ActionHookResult<[panelKey: keyof PanelVisibility]> {
  const [panelVisibility, setPanelVisibility, canSetPanelVisibility] = useTypeAppPanelVisibility();
  
  const action = useMemo(() => {
    if (!canSetPanelVisibility || !setPanelVisibility) return undefined;
    return (panelKey: keyof PanelVisibility) => {
      // Flip the boolean!
      setPanelVisibility({ 
        ...panelVisibility, 
        [panelKey]: !panelVisibility[panelKey] 
      });
    };
  }, [setPanelVisibility, canSetPanelVisibility, panelVisibility]);
  
  return [action, canSetPanelVisibility];
}
```

### Step 4: State Change Triggers Re-render
```tsx
// File: Sketchpad.tsx (LayoutWrapper)
const panelVisibility = useAppPanelVisibility();  // Gets updated state

// Conditional rendering based on visibility
leftSidePanel={
  leftSidePanelTabs.length > 0 && panelVisibility.leftSidePanel  // ← Checks boolean
    ? { /* panel config */ }
    : undefined  // ← Panel not rendered
}
```

### Step 5: SidePanel Component Renders
```tsx
// File: elements.tsx (SidePanel component)
const SidePanel: React.FC<SidePanelProps> = ({ 
  position,      // "left" or "right"
  visible,       // true (from panelVisibility boolean)
  tabs,          // SidePanelTab[] from context
  activeTabId,   // From context state
  onActiveTabChange 
}) => {
  if (!visible || tabs.length === 0) return null;  // Early return if invisible
  
  const activeTab = tabs.find(tab => tab.id === activeTabId) ?? tabs[0];
  
  return (
    <div 
      data-panel={position === "left" ? "leftSidePanel" : "rightSidePanel"}
      style={{ width: `${size}px` }}
    >
      {/* Tab bar */}
      {tabs.map(tab => (
        <button onClick={() => onActiveTabChange(tab.id)}>
          <tab.icon />
        </button>
      ))}
      
      {/* Active tab content */}
      <Scrollable>
        {activeTab.content()}
      </Scrollable>
    </div>
  );
};
```

---

## 🏗️ App Panel Configuration System

### How Apps Register Their Panels

```tsx
// File: Type.tsx (example)
export const TypeApp: AppConfig = {
  id: "semio.sketchpad.app.type",
  
  // Define which panels this app uses
  getPanels: (): PanelDefinition[] => [
    createPanelDefinition(PanelKind.WORKBENCH, "semio.sketchpad.navbar.panelToggle.workbench.show"),
    createPanelDefinition(PanelKind.TOOLS, "semio.sketchpad.navbar.panelToggle.tools.show"),
    createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show"),
    createPanelDefinition(PanelKind.STATS, "semio.sketchpad.navbar.panelToggle.stats.show"),
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
    createPanelDefinition(PanelKind.CHAT, "semio.sketchpad.navbar.panelToggle.chat.show"),
    createPanelDefinition(PanelKind.SETTINGS, "semio.sketchpad.navbar.panelToggle.settings.show"),
  ],
  
  // ... other app config
};
```

### PanelKind → Position Mapping

```tsx
// File: shared.ts (panelKindConfigs)
const panelKindConfigs: Record<PanelKind, PanelKindConfig> = {
  [PanelKind.WORKBENCH]: {
    icon: FolderIcon,
    position: PanelPosition.LEFT,   // ← Maps to left panel
    group: "left",
    isGroupable: true,
  },
  [PanelKind.TOOLS]: {
    icon: ToolIcon,
    position: PanelPosition.LEFT,   // ← Maps to left panel
    group: "left",
    isGroupable: true,
  },
  [PanelKind.STATS]: {
    icon: ChartIcon,
    position: PanelPosition.MIDDLE, // ← Maps to HUD panel
    hotkey: "ctrl+h",
  },
  [PanelKind.DETAILS]: {
    icon: DocumentIcon,
    position: PanelPosition.RIGHT,  // ← Maps to right panel
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+d",
  },
  [PanelKind.CHAT]: {
    icon: ChatIcon,
    position: PanelPosition.RIGHT,  // ← Maps to right panel
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
  [PanelKind.SETTINGS]: {
    icon: SettingsIcon,
    position: PanelPosition.RIGHT,  // ← Maps to right panel
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
};
```

### Registration Process in LayoutWrapper

```tsx
useEffect(() => {
  const panels = panelConfigs[appType] || [];  // Get current app's panels
  const registeredIds: string[] = [];

  panels.forEach((panel) => {
    const config = panelKindConfigs[panel.kind];  // Get position config
    if (!config) return;

    const sections = sectionsByKind[panel.kind] || [];  // Get content

    const tab = {
      id: panel.id,                                      // Toggle ID
      icon: config.icon,                                 // Icon component
      order: 0,
      content: () => <PanelTabContent sections={sections} />,  // Rendered content
    };

    // Register based on position
    if (config.position === PanelPosition.LEFT) {
      addSidePanelTab("left", tab);    // ← Add to left context
      registeredIds.push(panel.id);
    } 
    else if (config.position === PanelPosition.RIGHT) {
      addSidePanelTab("right", tab);   // ← Add to right context
      registeredIds.push(panel.id);
    } 
    else if (config.position === PanelPosition.MIDDLE) {
      addHudPanelTab(tab);             // ← Add to HUD context
      registeredIds.push(panel.id);
    }
  });

  // Cleanup on unmount or app change
  return () => {
    registeredIds.forEach((id) => {
      const panel = panels.find((p) => p.id === id);
      if (!panel) return;
      const config = panelKindConfigs[panel.kind];
      if (!config) return;

      if (config.position === PanelPosition.LEFT) 
        removeSidePanelTab("left", id);
      else if (config.position === PanelPosition.RIGHT) 
        removeSidePanelTab("right", id);
      else if (config.position === PanelPosition.MIDDLE) 
        removeHudPanelTab(id);
    });
  };
}, [appType, sections...]);  // Re-run when app changes
```

---

## 🎯 Key Connection Points

### 1. **Container vs Content Visibility**

There's a two-level visibility system:

```typescript
interface PanelVisibility {
  // CONTAINER LEVEL (controls panel existence)
  leftSidePanel?: boolean;    // Toggle shows/hides entire left panel
  rightSidePanel?: boolean;   // Toggle shows/hides entire right panel
  hudPanel?: boolean;         // Toggle shows/hides HUD panel
  
  // CONTENT LEVEL (controls which tab is active within visible panel)
  workbench?: boolean;        // Content for left panel
  tools?: boolean;            // Content for left panel
  details?: boolean;          // Content for right panel
  chat?: boolean;             // Content for right panel
  settings?: boolean;         // Content for right panel
  stats?: boolean;            // Content for HUD panel
}
```

**Container Toggle Flow:**
```
User clicks left toggle 
→ togglePanel("leftSidePanel") 
→ panelVisibility.leftSidePanel = !panelVisibility.leftSidePanel
→ Layout checks: leftSidePanelTabs.length > 0 && panelVisibility.leftSidePanel
→ Renders <SidePanel> or nothing
```

**Content Tab Flow:**
```
User clicks tab in panel tab bar
→ onActiveTabChange(tabId) 
→ setActiveLeftTabId(tabId)
→ SidePanel re-renders with new active tab content
```

### 2. **Context-Based Tab Registration**

```typescript
// Global context stores all registered tabs
SidePanelTabContext {
  left: [
    { id: "workbench", icon: FolderIcon, content: <WorkbenchContent /> },
    { id: "tools", icon: ToolIcon, content: <ToolsContent /> }
  ],
  right: [
    { id: "details", icon: DocumentIcon, content: <DetailsContent /> },
    { id: "chat", icon: ChatIcon, content: <ChatContent /> },
    { id: "settings", icon: SettingsIcon, content: <SettingsContent /> }
  ]
}

// Apps register tabs dynamically
addSidePanelTab("right", {
  id: "semio.sketchpad.navbar.panelToggle.details.show",
  icon: DocumentIcon,
  content: () => <PanelTabContent sections={detailsSections} />
});
```

### 3. **Toggle Button Visibility Logic**

```tsx
// PanelToggles component
const leftTabs = useSidePanelTabs("left");   // From context
const rightTabs = useSidePanelTabs("right"); // From context

const hasLeftTabs = leftTabs.length > 0;     // ← Only show toggle if tabs exist
const hasRightTabs = rightTabs.length > 0;   // ← Only show toggle if tabs exist

if (!hasLeftTabs && !hasHudTabs && !hasRightTabs) return null;  // No toggles at all
```

### 4. **State Synchronization**

```tsx
// Toggle button reads state
const isRightOpen = visiblePanels.rightSidePanel ?? false;

// Toggle button pressed property
<Toggle 
  pressed={isRightOpen}  // ← Visual state: button appears pressed/unpressed
  onPressedChange={handleRightToggle}  // ← Triggers state flip
/>

// Panel rendering reads same state
rightSidePanel={
  rightSidePanelTabs.length > 0 && panelVisibility.rightSidePanel  // ← Same state
    ? { /* config */ }
    : undefined
}
```

---

## 🧪 Testing Infrastructure

### Test Selectors

```typescript
// Toggle button selector
const rightToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');

// Panel container selector
const rightPanel = page.locator('[data-panel="rightSidePanel"]');

// Verify toggle works
await rightToggle.click();
await expect(rightPanel).toBeVisible();

await rightToggle.click();
await expect(rightPanel).not.toBeVisible();
```

### Panel Group Mapping

```typescript
// Maps content to container
const PANEL_GROUPS: Record<string, string> = {
  leftSidePanel: "leftSidePanel",
  workbench: "leftSidePanel",      // workbench tab is in left panel
  tools: "leftSidePanel",          // tools tab is in left panel
  
  hudPanel: "hudPanel",
  hud: "hudPanel",                 // hud tab is in HUD panel
  stats: "hudPanel",               // stats tab is in HUD panel
  
  rightSidePanel: "rightSidePanel",
  details: "rightSidePanel",       // details tab is in right panel
  chat: "rightSidePanel",          // chat tab is in right panel
  settings: "rightSidePanel",      // settings tab is in right panel
};
```

---

## 📋 Summary: The Complete Loop

```
┌─────────────────────────────────────────────────────────────────────┐
│  1. App Startup                                                     │
│     • Each app (Home, Kit, Type, etc.) defines getPanels()         │
│     • Returns array of PanelDefinition (WORKBENCH, DETAILS, etc.)  │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│  2. LayoutWrapper useEffect                                         │
│     • Reads panelConfigs[currentApp]                               │
│     • Maps PanelKind → PanelPosition (LEFT/RIGHT/MIDDLE)           │
│     • Calls addSidePanelTab(position, tab)                         │
│     • Stores in SidePanelTabContext                                │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│  3. PanelToggles Renders                                            │
│     • Reads useSidePanelTabs("left") → array of tab configs        │
│     • Reads useSidePanelTabs("right") → array of tab configs       │
│     • Shows toggle button only if tabs.length > 0                  │
│     • Reads panelVisibility.leftSidePanel for pressed state        │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│  4. User Clicks Toggle                                              │
│     • onClick → handleRightToggle()                                 │
│     • Calls appCommands.togglePanel("rightSidePanel")              │
│     • App XState action flips boolean                              │
│     • panelVisibility.rightSidePanel = !value                      │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│  5. Layout Re-renders                                               │
│     • Checks: rightSidePanelTabs.length > 0 && panelVisibility.rightSidePanel │
│     • If true: passes rightSidePanel={{ position, tabs, ... }}    │
│     • If false: passes rightSidePanel={undefined}                  │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│  6. SidePanel Component                                             │
│     • If !visible || !tabs: returns null                           │
│     • Otherwise: renders panel with tab bar and active content     │
│     • Positioned absolutely with data-panel attribute              │
│     • Provides resize handle and scrollable content area           │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 🔑 Critical Files

| File | Responsibility |
|------|----------------|
| [Sketchpad.tsx](semio/js/sketchpad/Sketchpad.tsx#L16408) | `PanelToggles` component - renders toggle buttons |
| [Sketchpad.tsx](semio/js/sketchpad/Sketchpad.tsx#L14616) | `SidePanelTabContext` - stores registered tabs |
| [Sketchpad.tsx](semio/js/sketchpad/Sketchpad.tsx#L17345) | `LayoutWrapper` - registers app panels, renders layout |
| [elements.tsx](semio/js/sketchpad/elements.tsx#L4992) | `SidePanel` component - renders actual panel UI |
| [shared.ts](semio/js/sketchpad/shared.ts#L820) | `panelKindConfigs` - maps PanelKind → PanelPosition |
| [Type.tsx](semio/js/sketchpad/Type.tsx#L1128) | `useTypeAppTogglePanel` - app-specific toggle action |
| [Type.tsx](semio/js/sketchpad/Type.tsx#L4018) | `getPanels()` - app panel configuration |

---

## 🎨 Visual Position Summary

```
┌────────────────────────────────────────────────────────────────────┐
│  Navbar: [◀] [🔍] [⬜] [➡]  [Left Toggle] [HUD Toggle] [Right Toggle] [⛶]  │
├────────────────────────────────────────────────────────────────────┤
│        │                                               │            │
│ Left   │                 Canvas                        │   Right    │
│ Side   │            (diagram/scene/table)              │   Side     │
│ Panel  │                                               │   Panel    │
│        │                                               │            │
│ ┌────┐ │            ┌──────────────┐                  │  ┌────┐   │
│ │📁│💼│ │            │  HUD Panel   │                  │  │📄│💬│   │
│ └────┘ │            │  (floating)  │                  │  └────┘   │
│        │            └──────────────┘                  │            │
│ Tabs:  │                                               │  Tabs:     │
│ • 📁 Workbench                                         │  • 📄 Details │
│ • 💼 Tools                                             │  • 💬 Chat    │
│                                                        │  • ⚙️ Settings │
│ Content:                                               │  Content:   │
│ <PanelTabContent>                                      │  <PanelTabContent> │
└────────────────────────────────────────────────────────────────────┘
     ↑                                                        ↑
position="left"                                        position="right"
tabs from context.left                                 tabs from context.right
visible = panelVisibility.leftSidePanel               visible = panelVisibility.rightSidePanel
```
