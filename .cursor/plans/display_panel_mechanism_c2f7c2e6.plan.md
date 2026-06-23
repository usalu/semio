---
name: Display Panel Mechanism
overview: Add a general, product-agnostic "Display" left panel (next to Workbench) with two tabs — Windows (window kinds and their drag-to-place templates) and Layout (reusable, savable named layouts) — wired once in the shared ProductShell so every product (platform, playground, presentation, puzzles) gets it automatically.
todos:
  - id: ticket
    content: Open repo-MCP ticket (read repo://goals, associate with a goal) before editing
    status: completed
  - id: core-types
    content: Add WindowTemplate, NamedLayout, templateId/instanceId on window node, templates on BaseWindowKindRuntime, and display PanelKind + StoragePort/NamedLayoutStore in framework/core/index.ts
    status: completed
  - id: mode-canvas
    content: Extend @semio-tech/ui-react Mode with onLayoutChange and external window-template drop handling
    status: completed
  - id: shell-canvas
    content: Refactor ShellModeCanvas for dynamic template instances, apply-layout, and live-layout capture
    status: completed
  - id: display-panel
    content: Build DisplayPanel component with Windows (draggable templates) and Layout (apply/save/delete) tabs
    status: completed
  - id: synthesize-panel
    content: Auto-inject the display panel in PlatformView and PlaygroundView, add display panel icon
    status: completed
  - id: persistence
    content: Add localStorage StoragePort adapter wired into NamedLayoutStore keyed per app id
    status: completed
  - id: i18n
    content: Replace panelToggle.windows with panelToggle.display and add Display tab/action labels
    status: completed
  - id: authoring
    content: Author templates + named layouts and camera-preset command for puzzle/3d (showcase), puzzle/2d, and wires
    status: completed
  - id: legacy-windows
    content: Migrate sketchpad client panel:"windows" usage to display
    status: completed
  - id: tests
    content: Extend existing test files for core types/store, Mode drop/onLayoutChange, DisplayPanel, and verify at runtime via launch.json
    status: completed
isProject: false
---

# Display Panel Mechanism

Add a general "Display" left panel with two tabs: **Windows** (window kinds + drag-to-place templates) and **Layout** (reusable, user-savable named layouts). The mechanism is defined once in `@semio-tech/framework-core` + the shared shell renderer, so all products inherit it.

## Decisions (confirmed)
- `display` becomes a brand-new left `PanelKind`; the legacy `windows` kind is removed/repurposed.
- Templates and named layouts are author-declared, AND users can save the current canvas arrangement as a new reusable layout (persisted).
- Window templates are **dragged** from the Windows tab onto the canvas to place them; clicking a layout in the Layout tab replaces the whole arrangement.

## 1. Core concepts (`framework/core/index.ts`)
- In `#region 🔖Layout`, add optional `templateId` + `instanceId` to `WindowLayoutWindowNode` so a layout slot can pick a template and so multiple instances of one kind can coexist.
- New `#region 🔖WindowTemplate`: 
  - `WindowTemplate { id; label; iconId?; controllerId?; command?; args? }` (the `command`/`args` are dispatched on the `CommandBus` when a window is created from the template, e.g. apply a camera preset).
- New `#region 🔖NamedLayout`:
  - `NamedLayout { id; label; iconId?; layout: WindowLayout; origin: "builtin" | "user" }`.
  - Helpers `mergeNamedLayouts`, and a `createNamedLayout(...)` factory.
- Extend `BaseWindowKindRuntime` (line ~661) with `readonly templates: readonly WindowTemplate[] = []`.
- `#region 🔖SideTab`: change `PanelKind` to `"display" | "overview" | "workbench" | "details" | "settings" | "chat"`, and `LEFT_PANEL_KINDS = ["workbench", "display", "overview"]` (display adjacent to workbench). Update `PANEL_KINDS`/`panelSide` accordingly.
- New `#region 🔖DisplayStore`: a render-neutral `StoragePort` interface (`get(key)/set(key,value)/remove(key)`) and a `NamedLayoutStore extends Store<NamedLayout[]>` that loads/saves user layouts per app id behind that port (no direct browser API in core — satisfies the "external libs behind an interface" rule).

## 2. `@semio-tech/ui-react` `Mode` canvas (`ui/react/index.tsx`)
- Extend `ModeProps` (line 15703) with `onLayoutChange?(layout: WindowLayoutNode)` and `onTemplateDrop?(payload, target)`.
- In `Mode` (line 16597): call `onLayoutChange` whenever `setLayoutState` mutates from user docking/closing/resizing (drag-dock, `closeWindow`, axis resize).
- Add external-drop handling in the existing drop-zone logic (`refreshDropZone`/drop handlers near 16699+): accept a new MIME `application/x-compose-window-template`; on drop, compute the target stack/side and invoke `onTemplateDrop` (the shell inserts the new window). This reuses the existing `ModeDropZone` machinery already built for internal window dragging.
- `ModeWindowDescriptor` already keyed by `id`; ensure dynamic instance ids render via the existing `windowsById` map.

## 3. Shared shell canvas (`framework/product/platform/renderer/react/index.tsx`)
`ShellModeCanvas` (line ~776) is the single chokepoint used by both `PlatformView` and `PlaygroundView` (via `ProductShell`). Refactor it to support dynamic, template-instantiated windows:
- Lift layout to state: track `currentLayout` + `windowInstances` (map `instanceId -> { windowKindId, templateId }`).
- Build the `windows` array for `Mode` from BOTH the static `windowKinds` and the dynamic instances (resolve body component by `windowKindId`, render title from template label).
- `onTemplateDrop`: create a unique `instanceId`, insert a `window` node at the drop target, dispatch the template's `command` via `commandBus`, update state.
- `onLayoutChange`: store the live layout (used by Display "Save layout").
- Apply named layout: expose an imperative entry (via context/prop) so the Display panel can replace `currentLayout` with `namedLayout.layout` (remapping any `templateId` slots to fresh instances + dispatching their template commands).
- Parse/serialize already exists (`parseWindowLayout`/`stringifyWindowLayout`) for persistence.

## 4. DisplayPanel component (`framework/product/platform/renderer/react/index.tsx`, new region)
A single React component rendered as the `content` of a `SidePanelTabConfig` section (same pattern as the puzzle inspector panel), with two internal tabs using existing `@semio-tech/ui-react` primitives:
- **Windows tab**: list `activeMode.windowKinds`; under each kind list its `templates` as draggable rows. Each row sets `dataTransfer` MIME `application/x-compose-window-template` with `{ windowKindId, templateId }` (mirrors the existing tree palette drag mechanism, `dragData` at `ui/react/index.tsx:7859`).
- **Layout tab**: list builtin + user `NamedLayout`s; click applies (replace arrangement); a "Save current layout" action snapshots the live layout into the `NamedLayoutStore`; user layouts get a delete affordance.
- Reads window kinds/layouts and the apply/save callbacks from a small `DisplayContext` provided by `ShellModeCanvas`/`ProductShell`.

## 5. Synthesize the Display panel for every product
- In `resolveAppPanelTabsByKind` (platform renderer ~2695) and the playground equivalent (`PlaygroundView`, ~933), auto-inject a synthetic `display` tab whenever the active app/mode has window kinds, with `content: <DisplayPanel/>`. This makes it appear for platform, playground, and presentation-play (which runs on the playground shell) without per-app wiring.
- Add `display` to `PANEL_KIND_ICON` (platform renderer ~2389) e.g. `"layout-grid"` (freed from old `windows`).

## 6. Persistence adapter
- Add a `localStorage`-backed `StoragePort` implementation in the renderer layer (platform renderer already uses `localStorage` at ~3797), wired into `NamedLayoutStore` keyed `compose.display.layouts.${appId}`. Core stays browser-API-free.

## 7. i18n (`ui/react/index.tsx`)
- Replace the `panelToggle.windows` entries (lines ~1349, ~1660) with `panelToggle.display`; add Display tab labels ("Windows", "Layout") and actions ("Save layout", "Delete layout"). Update the test at ~19545 if it asserts the old key.

## 8. Author templates + layouts (showcase + coverage)
- **puzzle/3d** (best showcase for top/perspective/north): in `puzzle/3d/play/index.ts` add `templates` (Top, Front, Perspective, North) to the window kind, plus builtin `NamedLayout`s (e.g. "Quad" = top left-top, north left-bottom, perspective right). Add a camera-preset command handler in `puzzle/3d/play` + `puzzle/3d/react` (apply orientation/projection to the Three.js camera).
- **puzzle/2d** (`puzzle/2d/play/index.ts`) and **reasoning/mindmap/wires** (`reasoning/mindmap/wires/play/index.ts`): add a couple of templates per pane/kind and one builtin named layout to prove generality.

## 9. Migrate legacy `windows` reference
- `compose/client/lib/sketchpad/js/index.ts:14386` registers `panel: "windows"`; repoint it to `display` (its windows-list panel is semantically the new Display) so the shared `PanelKind` change keeps the build green.

## 10. Tests + run config
- Extend existing test files only (no new files): `framework/core` tests for `WindowTemplate`/`NamedLayout`/`PanelKind`/store; `ui/react` tests for `Mode` `onLayoutChange`/template-drop and panel-toggle label; renderer tests for DisplayPanel drag payload + apply/save.
- Verify via existing `launch.json` entries (puzzle 3d/2d play) that the Display toggle appears next to Workbench, dragging a template places a window, and saving/applying layouts works (confirm at runtime per repo rules).

## Notes
- Per repo rules: open a repo-MCP ticket first (read `repo://goals`), keep all temp artifacts in the ticket folder, edit existing files using `#region`/subregions, extend existing tests, and close the ticket with a summary when done.