---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Research on how each sketchpad app defines panels and windows.

## Changes

None (research only).

## Log

### Research: Panel and Window Definitions per App

#### apps/index.ts (semio/js/sketchpad/apps/index.ts, 23 lines)

Re-exports from shared.ts:
```ts
export { composePluginContributions, getAppPlugin, getAppPlugins, hasAppPlugin, registerAppPlugin } from "../shared";
export type { AppMachineContribution, AppPlugin } from "../shared";
```

No panel/window definitions here. Just re-exports the plugin registration API.

---

#### Home.tsx (semio/js/sketchpad/Home.tsx, 1791 lines)

**WindowKind enum** (L96-100):
```ts
export enum HomeAppWindowKind {
  Table = "table",
  Settings = "settings",
  Chat = "chat",
}
```

**windowConfig** (L1694-1725):
```ts
const windowConfig: AppWindowConfig = useMemo(
  () => ({
    windowKinds: [
      { id: HomeAppWindowKind.Table, label: "table", component: () => <TableWindow /> },
      { id: HomeAppWindowKind.Settings, label: "settings", component: () => <SettingsContent /> },
      { id: HomeAppWindowKind.Chat, label: "chat", component: () => <ChatPlaceholder /> },
    ],
    defaultLayout,
  }),
  [defaultLayout],
);
```

**defaultLayout** (L1657-1690):
Single stack with 100% width containing Table, Settings, Chat.

**getPanels** (L1779-1782):
```ts
getPanels: (): PanelDefinition[] => [
  createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show"),
  createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
],
```

**WORKBENCH references**: None.

**Panel sections registered**: toolbar (filters, create)

---

#### Kit.tsx (semio/js/sketchpad/Kit.tsx, 9117 lines)

**WindowKind enum** (L381-386):
```ts
export enum KitAppWindowKind {
  Table = "table",
  Diagram = "diagram",
  Settings = "settings",
  Chat = "chat",
}
```

**windowConfig** (L7941-7985):
```ts
const windowConfig: AppWindowConfig = useMemo(
  () => ({
    windowKinds: [
      { id: KitAppWindowKind.Table, label: "table", component: () => <TableWindow /> },
      { id: KitAppWindowKind.Diagram, label: "diagram", component: () => <DiagramWindow /> },
      { id: KitAppWindowKind.Settings, label: "settings", component: () => <KitEditorSettingsContent /><SketchpadSettingsContent /> },
      { id: KitAppWindowKind.Chat, label: "chat", component: () => <ChatPlaceholder /> },
    ],
    defaultLayout,
  }),
  [defaultLayout],
);
```

**defaultLayout** (L7893-7935):
Two stacks: 50% (Table, Settings, Chat) + 50% (Diagram).

**getPanels** (L9105-9108):
```ts
getPanels: (): PanelDefinition[] => [
  createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show"),
  createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
],
```

**WORKBENCH references**: panelVisibility defaults have `workbench: false` (L733, L739, L1186, L1352). KitStore uses workbench toggle (L3332).

**Panel sections registered**: toolbar (selection, filters, create, toolsGroup)

---

#### Type.tsx (semio/js/sketchpad/Type.tsx, 4065 lines)

**WindowKind enum** (L161-165):
```ts
export enum TypeAppWindowKind {
  Scene = "scene",
  Settings = "settings",
  Chat = "chat",
}
```

**windowConfig** (L3698-3735):
```ts
const windowConfig: AppWindowConfig = useMemo(() => ({
  windowKinds: [
    { id: TypeAppWindowKind.Scene, label: "Scene", component: (props) => <Scene isDragOver={isDragOver} /> },
    { id: TypeAppWindowKind.Settings, label: "settings", component: () => <TypeSettingsContent /> },
    { id: TypeAppWindowKind.Chat, label: "chat", component: () => <ChatPlaceholder /> },
  ],
  defaultLayout,
}), [defaultLayout, isDragOver]);
```

**defaultLayout** (L3647-3677):
Single stack 100% with Scene, Settings, Chat.

**getPanels** (L4050-4056):
```ts
getPanels: (): PanelDefinition[] => [
  createPanelDefinition(PanelKind.WORKBENCH, "semio.sketchpad.navbar.panelToggle.workbench.show"),
  createPanelDefinition(PanelKind.TOOLS, "semio.sketchpad.navbar.panelToggle.tools.show"),
  createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show"),
  createPanelDefinition(PanelKind.STATS, "semio.sketchpad.navbar.panelToggle.stats.show"),
  createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
],
```

**WORKBENCH references**: panelVisibility default `workbench: false` (L3910). No workbench sections registered in Type.tsx (workbench only defined via getPanels).

**Panel sections registered**: toolbar (filters, selection, lasso, hand, connector)

---

#### Design.tsx (semio/js/sketchpad/Design.tsx, 9821 lines)

**WindowKind enum** (L306-311):
```ts
export enum DesignAppWindowKind {
  Diagram = "diagram",
  Scene = "scene",
  Settings = "settings",
  Chat = "chat",
}
```

**windowConfig** (L9055-9097):
```ts
const windowConfig: AppWindowConfig = useMemo(() => ({
  windowKinds: [
    { id: DesignAppWindowKind.Diagram, label: "diagram", component: (props) => <DiagramWindow /> },
    { id: DesignAppWindowKind.Scene, label: "scene", component: (props) => <SceneWindow /> },
    { id: DesignAppWindowKind.Settings, label: "settings", component: () => <DesignSettingsContent /> },
    { id: DesignAppWindowKind.Chat, label: "chat", component: () => <ChatPlaceholder /> },
  ],
  defaultLayout,
}), [defaultLayout, reactFlowInstanceRef]);
```

**defaultLayout** (L8968-9012):
Two stacks: 50% (Diagram, Settings, Chat) + 50% (Scene).

**getPanels** (L9804-9810):
```ts
getPanels: (): PanelDefinition[] => [
  createPanelDefinition(PanelKind.WORKBENCH, "semio.sketchpad.navbar.panelToggle.workbench.show"),
  createPanelDefinition(PanelKind.TOOLS, "semio.sketchpad.navbar.panelToggle.tools.show"),
  createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show"),
  createPanelDefinition(PanelKind.STATS, "semio.sketchpad.navbar.panelToggle.stats.show"),
  createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
],
```

**WORKBENCH references** (extensive):
- `workbenchTypes = useKitTypes()` (L8956)
- `workbenchDesigns = useKitDesigns()` (L8957)
- `PiecesWorkbenchContent` component (L9300) - main workbench UI with type/design trees, drag-and-drop
- `addSection("workbench", ...)` for pieces (L9584) and windows library (L9591)
- panelVisibility defaults have `workbench: false` (L1202, L1409, L1579, L1679)

**Panel sections registered**: toolbar (select, filters), workbench (pieces, windows), details (various)

---

#### Quality.tsx (semio/js/sketchpad/Quality.tsx, 2363 lines)

**WindowKind enum** (L148-153):
```ts
export enum QualityAppWindowKind {
  Formula = "formula",
  Diagram = "diagram",
  Settings = "settings",
  Chat = "chat",
}
```

**windowConfig** (L2259-2302):
```ts
const windowConfig: AppWindowConfig = useMemo(() => ({
  windowKinds: [
    { id: QualityAppWindowKind.Formula, label: "Formula", component: (props) => <FormulaWindow /> },
    { id: QualityAppWindowKind.Diagram, label: "Diagram", component: (props) => <DiagramWindow /> },
    { id: QualityAppWindowKind.Settings, label: "settings", component: () => <QualitySettingsContent /> },
    { id: QualityAppWindowKind.Chat, label: "chat", component: () => <ChatPlaceholder /> },
  ],
  defaultLayout,
}), [defaultLayout, reactFlowInstanceRef]);
```

**defaultLayout** (L2220-2258):
Two stacks: width:20 (Formula) + width:80 (Diagram, Settings, Chat). Note: uses `componentType` instead of `componentName`.

**getPanels** (L2348-2354):
```ts
getPanels: (): PanelDefinition[] => [
  createPanelDefinition(PanelKind.WORKBENCH, "semio.sketchpad.navbar.panelToggle.workbench.show"),
  createPanelDefinition(PanelKind.TOOLS, "semio.sketchpad.navbar.panelToggle.tools.show"),
  createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show"),
  createPanelDefinition(PanelKind.STATS, "semio.sketchpad.navbar.panelToggle.stats.show"),
  createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
],
```

**WORKBENCH references**:
- `QualityWorkbench` component (L1723) - lists formula function nodes by category
- `QualityWorkbenchQualities` component (L1837)
- `addSection("workbench", ...)` for functions (L2121) and qualities (L2128)
- panelVisibility defaults have `workbench: false` (L699, L864, L1121)

**Panel sections registered**: workbench (functions, qualities)

---

#### Docs.tsx (semio/js/sketchpad/Docs.tsx, ~1600 lines)

**WindowKind enum** (L1388-1392):
```ts
export enum DocsAppWindowKind {
  Page = "page",
  Settings = "settings",
  Chat = "chat",
}
```

**windowConfig** (L1490-1540):
```ts
const windowConfig: AppWindowConfig = useMemo(() => ({
  windowKinds: [
    { id: DocsAppWindowKind.Page, label: "page", component: () => <PageCanvas /> },
    { id: DocsAppWindowKind.Settings, label: "settings", component: () => <Settings /> },
    { id: DocsAppWindowKind.Chat, label: "chat", component: () => <ChatPlaceholder /> },
  ],
  defaultLayout,
}), [defaultLayout, error, loading, mdxModule]);
```

**defaultLayout** (L1400-1416):
Single stack 100% with Page, Settings, Chat.

**getPanels** (L1572-1581):
```ts
getPanels: (getLabelFn, getHotkeyFn) => [
  createPanelDefinition(PanelKind.WORKBENCH, "semio.sketchpad.navbar.panelToggle.workbench.show", getHotkeyFn("..."), { labelKey, manualPath }),
  createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show", getHotkeyFn("..."), { labelKey, manualPath }),
],
```

**WORKBENCH references**:
- `Workbench` component (L1293) - displays docs sections navigation
- `addSection("workbench", ...)` for docs (L1426) and overview (L1432)
- panelVisibility defaults have `workbench: false` (L847, L1014)

**Panel sections registered**: workbench (docs, overview), details (page), toolbar (empty placeholder)

---

#### Feedback.tsx (semio/js/sketchpad/Feedback.tsx, ~600 lines)

**WindowKind enum**: None. Feedback uses a direct `<FeedbackForm />` inside `<Canvas>` without LayoutCanvas.

**windowConfig**: None.

**getPanels** (L550):
```ts
getPanels: (): PanelDefinition[] => [createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show")],
```

**WORKBENCH references**: None.

**Panel sections registered**: toolbar (send action)

---

#### Tutorials.tsx (semio/js/sketchpad/Tutorials.tsx, 1232 lines)

No app config, no getPanels, no windowKinds, no WORKBENCH references. This is a utility/component file for tutorial overlays, not a sketchpad app.

---

### Summary Table

| App      | WindowKinds                          | Panels (getPanels)               | WORKBENCH panel? | Workbench sections? | Uses LayoutCanvas? |
|----------|--------------------------------------|----------------------------------|------------------|---------------------|-------------------|
| Home     | Table, Settings, Chat                | TOOLBAR, DETAILS                 | No               | No                  | Yes               |
| Kit      | Table, Diagram, Settings, Chat       | TOOLBAR, DETAILS                 | No               | No                  | Yes               |
| Type     | Scene, Settings, Chat                | WORKBENCH, TOOLS, TOOLBAR, STATS, DETAILS | Yes    | No (not registered) | Yes               |
| Design   | Diagram, Scene, Settings, Chat       | WORKBENCH, TOOLS, TOOLBAR, STATS, DETAILS | Yes    | Yes (pieces, windows) | Yes             |
| Quality  | Formula, Diagram, Settings, Chat     | WORKBENCH, TOOLS, TOOLBAR, STATS, DETAILS | Yes    | Yes (functions, qualities) | Yes        |
| Docs     | Page, Settings, Chat                 | WORKBENCH, DETAILS               | Yes              | Yes (docs, overview) | Yes              |
| Feedback | (none)                               | TOOLBAR                          | No               | No                  | No (direct Canvas)|
| Tutorials| (none)                               | (none)                           | No               | No                  | No (not an app)   |

### Key Shared Types (shared.ts)

- `PanelKind.WORKBENCH = "workbench"` - position: LEFT, group: "workbench", icon: WorkbenchIcon, hotkey: ctrl+j
- `PanelKind.TOOLS = "tools"` - position: LEFT, group: "workbench", hotkey: ctrl+j
- `PanelKind.TOOLBAR` - position: BOTTOM
- `PanelKind.STATS` - position: MIDDLE (HUD), group: "hud"
- `PanelKind.DETAILS` - position: RIGHT, group: "right", hotkey: ctrl+l
- `AppWindowConfig` = { windowKinds: WindowKindDefinition[], defaultLayout?: any }
- `WindowKindDefinition` = { id, label?, icon?, component, controls?, variants? }
- `createPanelDefinition(kind, id, hotkey?, tooltip?)` → `PanelDefinition`

## Todos

- [x] Research panel/window definitions across all app files

## Plan

Research complete. This data is needed to plan the workbench-to-main-container migration.
