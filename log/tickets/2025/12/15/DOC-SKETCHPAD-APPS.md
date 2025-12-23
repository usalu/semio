---
slug: DOC-SKETCHPAD-APPS
summary: Document Sketchpad apps and core mechanisms
prompt: >-
  The dev docs are significantly incomplete with the implementation. All apps in
  sketchpad are missing. The core mechanisms are not explained etc.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.956Z"
commit: "0000000000000000000000000000000000000000"
iterations:
  - prompt: >-
      The dev docs are significantly incomplete with the implementation. All
      apps in sketchpad are missing. The core mechanisms are not explained etc.
    date:
      started: "2025-12-15T17:24:42.224Z"
    model: claude-opus-4-5
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 2fb81ef29354981c1b9625769dba4a06360a4aef
    files:
      updated:
        - path: AGENTS.md
          lines:
            added: 2279
            removed: 340
      created: []
      removed: []
    lines:
      added: 2279
      removed: 340
---

# Previously

The AGENTS.md documentation had architecture-level documentation for the Sketchpad but was missing:

- Documentation for individual apps (Home, Kit, Type, Design, Quality, Docs)
- Core mechanism documentation (shared.ts types, YPath, DerivedStore, App Plugin Registry)
- Store factory registry documentation

# Plan

1. Read all app source files (Home.tsx, Kit.tsx, Type.tsx, Design.tsx, Quality.tsx, Docs.tsx)
2. Extract state interfaces, events, hooks, and commands from each app
3. Add comprehensive documentation for all 6 apps under "##### Sketchpad Apps" section
4. Add core types documentation (HookResult, enums, PanelSystem, ToolSystem, App IDs)
5. Add YPath and DerivedStore documentation
6. Add App Plugin Registry documentation
7. Add Store Factory Registry documentation

# Changes

## AGENTS.md

### Added "##### Sketchpad Apps" section with documentation for:

**Home App (Home.tsx):**

- HomeState: panelVisibility, selection, sortColumn/Direction, loadingKits
- Events: HOME.TOGGLE_PANEL, HOME.SET_PANEL_VISIBILITY, HOME.SELECT_KIT/DESELECT_KIT, HOME.SET_SORT
- Hooks: useHomeApp(), useHomeSelection(), useHomeLoadingKits(), useHomePanelVisibility()

**Kit App (Kit.tsx):**

- KitAppState: panelVisibility, selection (9 artifact types), hover, filterSearch, expandedRows, sorting
- Selection Types: types, designs, qualities, interfaces, tags, concepts, files, folders, authors
- Events: KIT.TOGGLE*PANEL, KIT.SELECT*\_/DESELECT\_\_, KIT.SET*HOVER, KIT.SET_FILTER_SEARCH, KIT.CREATE*\*
- Hooks: useKitApp(), useKitAppSelection(), useKitAppHover(), useKitAppFilterSearch()

**Type App (Type.tsx):**

- TypeAppState: panelVisibility, activeTool, selection (ports/models), hover, camera, focusedPortGuid, selectedModelGuid
- Events: TYPE.TOGGLE*PANEL, TYPE.SET_TOOL, TYPE.SELECT*\_/DESELECT\_\_, TYPE.SET_HOVER, TYPE.SET_CAMERA
- Hooks: useTypeApp(), useTypeAppSelection(), useTypeAppHover(), useTypeAppCamera(), useTypeAppActiveTool()

**Design App (Design.tsx):**

- DesignAppState: panelVisibility, activeTool, selection (pieces/connections/port), hover, camera, diagram state
- Commands: semio.designApp.selectAll, semio.designApp.deselectAll, semio.designApp.deleteSelected
- Events: DESIGN.TOGGLE*PANEL, DESIGN.SET_TOOL, DESIGN.SELECT*\_/DESELECT\_\_, DESIGN.DELETE_SELECTED
- Hooks: useDesignApp(), useDesignAppSelection(), useDesignAppHover(), useDesignAppCamera(), useDesignAppActiveTool()

**Quality App (Quality.tsx):**

- QualityAppState: panelVisibility, activeTool, selection (formulaNodes), hover, formulaNodes
- FormulaNode: id, kind (function/quality/variable/unit/value), name, children, x, y
- Formula Functions: Numeric, Branching, Data, Text, Comparison
- Events: QUALITY.TOGGLE*PANEL, QUALITY.SET_TOOL, QUALITY.SELECT*\*/DESELECT_FORMULA_NODE, QUALITY.SET_HOVER
- Hooks: useQualityApp(), useQualityAppSelection(), useQualityAppHover()

**Docs App (Docs.tsx):**

- MDX loading system: loadMDXFile(), getAllMDXFiles(), getMDXFilesBySection(), getAllSections()
- Heading state: useHeadings(), headingsState.registerHeading(), headingsState.setActiveHeading()

### Added "### Core Types (shared.ts)" section:

- Hook Result Types: HookResult<T>, HookNoSetResult<T>, helper functions
- Core Enums: Theme, Expertise, Mode, StoreStatus, ToolKind, WindowKind, PanelPosition, PanelKind
- Panel System: PanelKindConfig, PanelVisibility, PanelSection with positioning rules
- Tool System: Tool<TState>, ToolMode, ToolDefinition interfaces
- App IDs: KitAppId, TypeAppId, DesignAppId, QualityAppId

### Added "### YPath and DerivedStore" section:

- YPath types and segments for navigating Y.js structures
- Path helpers: yPathMapKey(), yPathArrayIndex(), yPathArrayItemById()
- DerivedNode<T> class for cached computations
- DerivedStore class for managing derived nodes

### Added "### App Plugin Registry" section:

- AppPlugin and AppMachineContribution interfaces
- Registration functions: registerAppPlugin(), getAppPlugins(), etc.
- EventHandlerConfig for dynamic event dispatch
- Event handler registry functions
- Guard registry functions

### Added "### Store Factory Registry" section:

- Factory registration functions for avoiding circular dependencies
- Factory getters for each app type
