# Workbench Manual

## Executive Summary (10 lines)

The **workbench** is the left sidebar panel in the semio sketchpad application — a single-page React app for designing architectural kits. It lives entirely in one 57k-line file: `semio/sketchpad/index.tsx`. The workbench displays a **tree of draggable items** (types, designs, qualities, doc sections) that users drag onto a central canvas to compose designs. It is one of several panel kinds (workbench, details, tools, stats, toolbar, console) rendered inside a `Layout` shell from `elements/ui/index.tsx`. Each "app" (Kit, Design, Type, Quality, Docs, Home) independently registers its own workbench sections via a `PanelSectionContext` using `addSection("workbench", ...)` / `removeSection("workbench", ...)` in `useEffect` hooks. The workbench maps to `PanelKind.WORKBENCH` → `PanelPosition.LEFT` and is rendered inside the `SidePanel` component as a tab in the left side panel. Visibility is toggled via `panelVisibility.leftSidePanel` in the per-app XState-managed state. State flows: `AppPlugin` → XState machine → `PanelSectionContext` → `SidePanel` → `Tree/TreeItem` components. Drag-and-drop uses `@dnd-kit` with `useDraggable` on workbench items and `useDroppable` on the canvas.

---

## 1. What the Workbench Is

**Purpose:** The workbench is the primary asset browser / entity palette for the sketchpad. It shows the available building blocks (types, designs, formula functions, doc sections) that belong to the currently open kit or context.

**Main responsibilities:**

- Display hierarchical tree of types/designs with avatars (`TypeTreeItem`, `DesignTreeItem`)
- Enable drag-and-drop of items from workbench onto canvas (diagram/scene)
- Provide inline actions (add piece, add child type, add child design, navigate-on-double-click)
- Show hover/highlight feedback on the canvas when hovering workbench items
- Optionally show formula function nodes (Quality app) or documentation sections (Docs app)

**User-facing behavior:**

- Toggled via navbar panel toggle (`semio.sketchpad.navbar.panelToggle.workbench.show`)
- Appears as a resizable panel on the left side of the layout
- Contains collapsible tree sections rendered via `Tree` / `TreeItem` from `@semio/ui`
- Items are draggable (`DraggableAvatar`) with avatar circles showing 2-letter initials

---

## 2. Architecture Map

### Entry Points

| Entry                         | File                        | Line   | Description                                                      |
| ----------------------------- | --------------------------- | ------ | ---------------------------------------------------------------- |
| `PanelKind.WORKBENCH`         | `semio/sketchpad/index.tsx` | ~922   | Enum value `"workbench"`                                         |
| `panelKindConfigs[WORKBENCH]` | `semio/sketchpad/index.tsx` | ~1081  | Config: `icon: WorkbenchIcon`, `position: LEFT`, `group: "left"` |
| `PanelSections.workbench`     | `semio/sketchpad/index.tsx` | ~1235  | Array of `PanelSection[]` — the dynamic section list             |
| `PanelSectionProvider`        | `semio/sketchpad/index.tsx` | ~22527 | React context that holds `{ workbench: PanelSection[], ... }`    |
| `LayoutWrapper`               | `semio/sketchpad/index.tsx` | ~25382 | Shell that wires `leftSidePanel` prop to `Layout`                |
| `Layout`                      | `elements/ui/index.tsx`     | ~10351 | Renders `<SidePanel position="left" .../>`                       |
| `SidePanel`                   | `elements/ui/index.tsx`     | ~17403 | Tabbed side panel with resize handle                             |

### Core Modules

```
┌─────────────────────────────────────────────────────────────────┐
│  semio/sketchpad/index.tsx (57k lines, single consolidated file)│
├─────────────────────────────────────────────────────────────────┤
│  🔖Shared (L360-3883)                                          │
│    Types, Enums (PanelKind, PanelPosition, PanelSection)        │
│    Ports (PanelSections, PanelVisibility, PanelSizes)           │
│    Store, Commands, App Plugin/Event Registry                   │
│  🔖ConsolidatedApps (L26301-49295)                              │
│    🔖Kit (L26304-45703)                                         │
│      DesignApp → PiecesWorkbenchContent → TypeTreeItem/DesignTreeItem│
│    🔖Quality (L44568-45226)                                     │
│      QualityWorkbench, QualityWorkbenchQualities                │
│    🔖Docs (L47081-47497)                                        │
│      Docs Workbench (doc sections tree)                         │
│  🔖SketchpadCore                                                │
│    PanelSectionProvider/Context (L22400-22600)                  │
│    LayoutWrapper (L25382+) — wires left/right panels            │
│    SidePanel Tabs state (L22600-22722)                          │
├─────────────────────────────────────────────────────────────────┤
│  elements/ui/index.tsx                                          │
│    Layout (L10351) — flex layout shell                          │
│    SidePanel (L17403) — tabbed resizable panel                  │
│    Tree, TreeItem, TreeContent, TreeRow — tree rendering        │
│    DraggableAvatar, Avatar — drag-enabled avatar circles        │
│    Scrollable, LevelProvider — chrome primitives                │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow

```
Kit/Design/Quality/Docs store (YDoc sync or local)
     │
     ▼
useKit(), useKitTypes(), useKitDesigns()  ← granular hooks
     │
     ▼
App component useEffect → addSection("workbench", { id, content: () => <PiecesWorkbenchContent /> })
     │
     ▼
PanelSectionContext.sections.workbench  ← sorted by specificity then order
     │
     ▼
LayoutWrapper reads usePanelSections("workbench") → creates SidePanelTab
     │
     ▼
Layout → SidePanel(position="left") → DynamicPanelTabContent → PanelTabContent
     │
     ▼
Tree → TreeItem/TreeRow → DraggableAvatar(useDraggable) → Canvas drop target
```

### Control Flow

```
User opens kit → URL: /kits/:guid
  → LayoutWrapper detects appType="kit" or "design"
  → App component mounts (DesignApp/KitApp/QualityApp/DocsApp)
  → useEffect calls addSection("workbench", ...) with content renderer
  → PanelSectionProvider updates sections.workbench[]
  → LayoutWrapper reads panelConfigs[appType] → registers SidePanelTab("left", ...)
  → Layout renders <SidePanel tabs={leftSidePanelTabs} ...>
  → User sees workbench tree in left panel

User drags type from workbench → canvas:
  → DraggableAvatar fires useDraggable (dnd-kit)
  → DndContext.onDragStart → setActiveDraggedType(type)
  → Canvas useDroppable fires DndContext.onDragEnd
  → CustomEvent "design-drag-end" dispatched
  → DesignApp listens and calls addPiece({type: {guid}})
```

### State Ownership

| State                           | Owner                                                   | Location                           |
| ------------------------------- | ------------------------------------------------------- | ---------------------------------- |
| `PanelSections.workbench`       | `PanelSectionContext`                                   | `semio/sketchpad/index.tsx` ~22527 |
| `panelVisibility.leftSidePanel` | Per-app XState state (e.g. `designApp.panelVisibility`) | App plugin stores                  |
| `panelSizes.leftSidePanelWidth` | `SketchpadState.panelSizes`                             | Sketchpad zustand-like store       |
| `activeLeftTabId`               | `SidePanelTabsContext`                                  | `semio/sketchpad/index.tsx` ~22600 |
| Kit data (types/designs)        | YDoc sync store via `useKit()`                          | Kit store (XState + Yjs)           |
| Drag state                      | `DndContext` (dnd-kit)                                  | `LayoutWrapper` ~25971             |
| Active interaction              | `SketchpadState.activeInteraction`                      | Sketchpad store                    |

### Config

| Config                     | Symbol                                          | File:Line          |
| -------------------------- | ----------------------------------------------- | ------------------ |
| Panel kind → position/icon | `panelKindConfigs`                              | `index.tsx:~1081`  |
| App → panels declaration   | `designConfig.getPanels()`                      | `index.tsx:~39445` |
| Default panel visibility   | `EMPTY_PANEL_VISIBILITY`                        | `index.tsx:~570`   |
| Default panel sizes        | `defaultPanelSizes` inside stores               | `index.tsx:~9538`  |
| Workbench avatar size      | CSS `--size-workbench` + `size-workbench` class | Tailwind config    |

### External Dependencies

- `@dnd-kit/core` — drag-and-drop primitives (`useDraggable`, `useDroppable`, `DndContext`)
- `@semio/ui` (aka `elements/ui/index.tsx`) — `Layout`, `SidePanel`, `Tree`, `TreeItem`, `DraggableAvatar`, `Avatar`
- `react-hotkeys-hook` — keyboard shortcuts
- `react-router-dom` — URL-based app routing
- `i18next` — label localization (`useLabel`, `useTranslation`)
- `yjs` / `y-protocols` — CRDT synced kit data

---

## 3. How Control Works

### Section Registration (the core mechanism)

Every app registers workbench content via `useEffect` calling `addSection("workbench", ...)`. This is the **only** way content enters the workbench:

```typescript
// Design app (index.tsx ~39018)
addSection("workbench", {
  id: "semio.sketchpad.app.kit.pieces",
  specificity: 20,
  order: 1,
  content: () => <PiecesWorkbenchContent />,
});

// Docs app (index.tsx ~47241+)
addSection("workbench", {
  id: "semio.sketchpad.app.docs.docs",
  specificity: 20,
  order: 1,
  content: () => <Workbench />,
});

// Quality app (index.tsx ~45316+)
addSection("workbench", {
  id: "semio.sketchpad.app.quality.formulaNodes",
  specificity: 20,
  order: 1,
  content: () => <QualityWorkbench />,
});
```

Sections have `specificity` (higher wins) and `order` (lower first) for sorting.

### Panel Visibility Toggle

```
User clicks navbar toggle
  → togglePanelVisibility("leftSidePanel")
  → App XState event (e.g. DESIGN.TOGGLE_PANEL)
  → Updates panelVisibility.leftSidePanel in app state
  → LayoutWrapper reads panelVisibility.leftSidePanel
  → Conditionally renders <SidePanel> in Layout.leftSidePanel prop
```

### Panel Tab Selection

`LayoutWrapper` registers one `SidePanelTab` per panel definition that has `position: LEFT`:

- `PanelKind.WORKBENCH` → left tab with `WorkbenchIcon`
- `PanelKind.TOOLS` → left tab with `ToolsIcon`

Active tab controlled by `activeLeftTabId` / `setActiveLeftTabId` from `SidePanelTabsContext`.

### Rendering Pipeline

```
PanelSectionProvider.sections.workbench: PanelSection[]
  → LayoutWrapper.useEffect registers SidePanelTab(id, icon, content: () => <DynamicPanelTabContent panelKey="workbench" />)
  → DynamicPanelTabContent calls usePanelSections("workbench")
  → PanelTabContent sorts sections, maps to tree sections
  → <TreeStateProvider> → <Tree sections={...} />
  → Each section.content() renders (e.g. <PiecesWorkbenchContent />)
  → PiecesWorkbenchContent renders TypeTreeItem/DesignTreeItem with DraggableAvatar
```

### Event Handling

- **Hover:** `onPointerEnter` → `hoverTypes([guid])` or `hoverDesigns([guid])` → XState event → diagram node highlight
- **Double-click:** `navigateToType(kitGuid, typeGuid)` or `navigateToDesign(...)` → URL change → app transition
- **Drag start:** `useDraggable` → `DndContext.onDragStart` → `setActiveDraggedType(type)` + `setActiveInteraction("dragging")`
- **Drag end:** `DndContext.onDragEnd` → `CustomEvent("design-drag-end")` → DesignApp handles add-piece
- **Add action:** inline action button → `addPiece({...})` or `kitAppCommands.addType(newType)`

---

## 4. Design Surfaces

### Easy to Change

- **Add new workbench section for an existing app:** Add one `useEffect` with `addSection("workbench", ...)` in the app component. Follow `PiecesWorkbenchContent` or `QualityWorkbench` as pattern.
- **Change section order:** Adjust `order` and `specificity` values in `addSection` calls.
- **Change workbench icon:** Modify `panelKindConfigs[PanelKind.WORKBENCH].icon` (~L1082).
- **Change workbench default size:** Modify `leftSidePanelWidth` in default panel sizes (~L9538).
- **Style tree items:** Modify Tailwind classes in `TypeTreeItem`/`DesignTreeItem` components.
- **Add avatar size:** Modify `--size-workbench` CSS variable, `size-workbench` utility class.

### Risky to Change

- **`PanelSectionContext` add/remove logic** (~L22527): shared by ALL panels (workbench, details, tools, etc.). Changes here affect every panel.
- **`PanelKind` enum and `panelKindConfigs`**: Adding/removing panel kinds requires updating all app configs, visibility types, size types, and the layout shell.
- **`Layout` component in `elements/ui`**: The `SidePanel` position/flex layout is shared infra. Changing left panel structure affects all apps.
- **`LayoutWrapper`** (~L25382-26200): The 800-line function that wires everything together. Extremely coupled to all apps, routing, themes, DnD, toolbar, etc.

### Hidden Coupling

- `"workbench"` **is a string key**, not `PanelKind.WORKBENCH`. The panel section system uses string keys (`"workbench"`, `"details"`, `"tools"`) that MUST match the `PanelSections` interface field names. A typo silently breaks content registration.
- **Workbench ≠ leftSidePanel.** The workbench is ONE TAB in the left side panel. The left side panel can also contain a "tools" tab. `panelVisibility.leftSidePanel` toggles the entire left panel, not just the workbench tab.
- **`DesignApp` adds workbench content for ALL design sub-views** (the "pieces" section). Removing it breaks type/design drag-and-drop onto the design canvas.
- **`DndContext` wraps the entire layout** in `LayoutWrapper`. All drag from workbench flows through this single DndContext. Multiple nested DndContexts would break drop detection.
- **Legacy "workbench" window component name.** Both `DesignApp` and `DocsApp` have `removeLegacySideTabsFromWindowLayout` / `removeWorkbenchWindowFromLayout` code that strips `componentName === "workbench"` from stored window layouts. This is migration cleanup from when workbench was a GoldenLayout window pane instead of a side panel.

### Invariants

- Every app MUST call `removeSection` in the `useEffect` cleanup to prevent stale sections persisting across app transitions.
- `PanelSection.id` MUST be globally unique across all apps. Convention: `"semio.sketchpad.app.{appId}.{panel}.{sectionName}"`.
- The `content` field MUST be `() => ReactNode` (thunk) for lazy rendering, not a bare `ReactNode`.
- `specificity` and `order` govern sort: higher specificity wins, then lower order first.

### Constraints

- The file is 57k lines. There is no module boundary between apps and infrastructure — everything is in `index.tsx`.
- No test isolation: Playwright e2e tests test workbench behavior through full-app interaction (panel toggles, tree visibility, drag-and-drop).
- Panel sections are ephemeral React state (`useState`), not persisted. On refresh, sections re-register from app `useEffect`s.

---

## 5. Change Guide

### Layout Changes

| Change                         | Where                                                                       |
| ------------------------------ | --------------------------------------------------------------------------- |
| Move workbench to right side   | Change `panelKindConfigs[WORKBENCH].position` to `RIGHT` at ~L1082          |
| Change workbench default width | Modify `leftSidePanelWidth` in default sizes at ~L9538                      |
| Add resize min/max             | Modify `minSize`/`maxSize` in `LayoutWrapper` leftSidePanel prop at ~L26001 |
| Change panel chrome/border     | Modify `SidePanel` in `elements/ui/index.tsx` ~L17403                       |

### New Workbench Section

```typescript
// In your app component's body:
const addSection = useAddPanelSection();
const removeSection = useRemovePanelSection();

useEffect(() => {
  addSection("workbench", {
    id: "semio.sketchpad.app.myapp.mysection",
    specificity: 20,
    order: 3,
    content: () => <MyWorkbenchContent />,
  });
  return () => removeSection("workbench", "semio.sketchpad.app.myapp.mysection");
}, [addSection, removeSection]);
```

### New App with Workbench

In the app's `AppConfig.getPanels()`:

```typescript
getPanels: () => [
  createPanelDefinition(PanelKind.WORKBENCH, "semio.sketchpad.navbar.panelToggle.workbench.show"),
  // ... other panels
],
```

This registers the workbench tab in the left side panel for that app.

### Add Drag-and-Drop Item

1. In a workbench content component, add `useDraggable` from `@dnd-kit/core`
2. Wrap in `DraggableAvatar` component
3. Set `data: { type: "myKind", guid: item.guid }` on the draggable
4. In the target canvas component, handle via `useDroppable` + `DndContext.onDragEnd`

### Styling

| What                         | Where                                                             |
| ---------------------------- | ----------------------------------------------------------------- |
| Avatar circle size           | `--size-workbench` CSS variable, `size-workbench` Tailwind class  |
| Gap between avatar and label | `gap-double` class on tree item label wrapper                     |
| Tree item padding/spacing    | `elements/ui/index.tsx` Tree/TreeItem components                  |
| Side panel chrome            | `SidePanel` in `elements/ui/index.tsx` ~L17403                    |
| Right panel compact sizing   | `rightSidePanelElementSizingClassName` in `LayoutWrapper` ~L25800 |

### Persistence

- Panel visibility is per-app XState state, persisted via the app's store.
- Panel sizes (`leftSidePanelWidth`) are in `SketchpadState.panelSizes`, persisted to local storage.
- Workbench sections are NOT persisted — they are ephemeral React state rebuilt on mount.
- Window layouts (GoldenLayout) ARE persisted in app settings.

---

## 6. Safe Modification Plan

### Before Any Change

1. **Read the `PanelSections` interface** (~L1235). Understand the `workbench` key.
2. **Find all `addSection("workbench", ...)` calls:**
   ```bash
   grep -n 'addSection("workbench"' semio/sketchpad/index.tsx
   ```
   Currently: ~L39018 (Design/PiecesWorkbenchContent), ~L47285 (Docs/sections), ~L47290 (Docs/overview). Quality app registers via workbench too (~L45316+).
3. **Find all `removeSection("workbench", ...)` calls** to verify cleanup.
4. **Trace the rendering path:** `PanelSectionProvider` → `usePanelSections("workbench")` → `DynamicPanelTabContent` → `PanelTabContent` → `Tree`.

### Smallest Steps to Make a Workbench Change

1. **Add a section:** Add `addSection("workbench", { id: "...", content: () => <div>test</div> })` in an existing app's `useEffect`. Verify it appears in the left panel.
2. **Remove a section:** Remove the `addSection` call and its `removeSection` cleanup. Verify nothing renders.
3. **Move a section between apps:** Remove from one app's `useEffect`, add to another's. Verify correct app scoping.
4. **Test visibility toggle:** Toggle `leftSidePanel` in navbar. Verify the workbench tab disappears/reappears.
5. **Test app transition:** Navigate from one app to another. Verify old workbench sections are cleaned up and new ones appear.

### What to Trace

- `PanelSectionProvider.addSection` — set a breakpoint to see what sections are being added
- `usePanelSections("workbench")` — verify the returned array matches expectations
- `LayoutWrapper` leftSidePanel prop — verify tabs are correctly assembled

### What to Test

- Panel toggle (leftSidePanel visibility) per app
- Section presence after app navigation
- Drag-and-drop from workbench to canvas
- Section cleanup on unmount (no stale sections after navigating away)
- Multiple apps with workbench (Design + Quality + Docs should each have their own sections)

### What to Avoid Breaking

- Do NOT change `PanelSections` interface field names without updating ALL `addSection`/`removeSection`/`usePanelSections` calls.
- Do NOT remove the `DndContext` wrapper in `LayoutWrapper` — it breaks all drag-and-drop.
- Do NOT add a second `DndContext` inside a workbench component — nested DndContexts conflict.
- Do NOT forget `removeSection` in `useEffect` cleanup — causes stale sections and memory leaks.
- Do NOT change `panelKindConfigs[WORKBENCH]` without updating visibility logic mapping.

---

## 7. Open Questions / Unknowns

| Question                                                                                                                                                                                                              | How to Verify                                                                                                                        |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Why does Kit app config NOT include `PanelKind.WORKBENCH` in `getPanels()`? (`kitConfig.getPanels()` at ~L18049 only has TOOLBAR + DETAILS.) Yet the workbench still shows via the Design app's section registration. | Check if Kit app has its own workbench content registration elsewhere, or if it relies on the Design app when navigated to a design. |
| The `DesignApp` registers `addSection("workbench", { id: "semio.sketchpad.app.kit.pieces" })` — the ID says "kit" but it's registered by the Design app. Is this intentional cross-app sharing?                       | Trace navigation flow: Kit→Design. The "pieces" section is shared because Design inherits kit scope.                                 |
| Quality app workbench sections — where exactly are they registered? The `QualityWorkbench` component exists but the `addSection` call needs tracing.                                                                  | Search for `addSection("workbench"` in the Quality app region (~L45316-45661).                                                       |
| Legacy `"workbench"` window component name in stored layouts — how long until this migration code can be removed?                                                                                                     | Check if any persisted layout JSON still contains `componentName: "workbench"`. This is defensive migration code.                    |
| The `PanelSectionContext` has NO deduplication guard on `id` beyond filtering — what happens if two apps register the same section ID?                                                                                | Test: register same ID from two apps. Current code: last-write-wins (filter + push).                                                 |
| Type app and Quality app — do they share the same workbench "pieces" section, or do they register separately?                                                                                                         | Grep for their specific `addSection("workbench"` calls.                                                                              |

---

## Dependency/Ownership Map

```
elements/ui/index.tsx
  └─ Layout (shell) → renders SidePanel(position="left")
  └─ SidePanel → renders tabs, active tab content, resize handle
  └─ Tree/TreeItem/TreeContent/TreeRow → tree structure
  └─ DraggableAvatar → drag-enabled avatar circles
  └─ Avatar → plain avatar circles

semio/sketchpad/index.tsx
  └─ PanelKind.WORKBENCH + panelKindConfigs → panel type registry
  └─ PanelSections.workbench → section array type
  └─ PanelSectionProvider → React context managing sections
  └─ LayoutWrapper → wires leftSidePanel to Layout
  └─ DndContext → drag-and-drop context wrapping entire app
  └─ DesignApp → registers PiecesWorkbenchContent (types + designs tree)
  └─ QualityApp → registers QualityWorkbench (formula nodes) + QualityWorkbenchQualities
  └─ DocsApp → registers Workbench (doc sections) + Overview (doc pages)
  └─ HomApp → no workbench sections
  └─ KitApp → no workbench panel definition (uses Design app's sections when in design context)
  └─ TypeApp → likely shares via kit scope
```

## "How to Change It" Checklist

- [ ] **Add workbench section:** Add `addSection("workbench", ...)` + cleanup in app `useEffect`
- [ ] **Remove workbench section:** Remove `addSection` + `removeSection` calls
- [ ] **Enable workbench for new app:** Add `createPanelDefinition(PanelKind.WORKBENCH, ...)` to app's `getPanels()`
- [ ] **Change workbench position:** Edit `panelKindConfigs[PanelKind.WORKBENCH].position`
- [ ] **Change workbench icon:** Edit `panelKindConfigs[PanelKind.WORKBENCH].icon`
- [ ] **Change workbench width:** Edit `leftSidePanelWidth` in default panel sizes
- [ ] **Add drag-and-drop:** Add `useDraggable` to workbench item + handle in canvas `DndContext.onDragEnd`
- [ ] **Add tree item actions:** Use `actions` prop on `TreeItem` (see Design workbench add-piece pattern)
- [ ] **Add hover feedback:** Use `onPointerEnter`/`onPointerLeave` on workbench items calling app hover commands
- [ ] **Test panel toggle:** Toggle `leftSidePanel` visibility via navbar or settings
- [ ] **Test section cleanup:** Navigate between apps, verify no stale workbench sections

## Prioritized Reading Order

1. **`semio/sketchpad/index.tsx` L922** — `PanelKind` enum (understand the workbench is one of 7 panel kinds)
2. **`semio/sketchpad/index.tsx` L1058-1329** — Panel type system (`PanelKindConfig`, `panelKindConfigs`, `PanelSections`, `PanelSection`)
3. **`semio/sketchpad/index.tsx` L22400-22600** — `PanelSectionProvider` (the React context that holds all panel sections)
4. **`semio/sketchpad/index.tsx` L25280-25420** — `PanelTabContent`, `DynamicPanelTabContent` (renders sections into Tree)
5. **`semio/sketchpad/index.tsx` L25420-26200** — `LayoutWrapper` (wires left/right side panels to Layout)
6. **`elements/ui/index.tsx` L10300-10380** — `Layout` component (the shell)
7. **`elements/ui/index.tsx` L17387-17500** — `SidePanel` component (tabbed resizable panel)
8. **`semio/sketchpad/index.tsx` L39000-39440** — Design app workbench registration (`PiecesWorkbenchContent`, `TypeTreeItem`, `DesignTreeItem`)
9. **`semio/sketchpad/index.tsx` L45046-45187** — Quality app workbench (`QualityWorkbench`, `QualityWorkbenchQualities`)
10. **`semio/sketchpad/index.tsx` L47081-47120** — Docs app workbench
11. **`semio/sketchpad/index.tsx` L39445-39500** — `designConfig.getPanels()` (how apps declare which panels they want)
12. **`semio/sketchpad/index.tsx` L18037-18060** — `kitConfig` (note: no WORKBENCH panel — investigate why)
