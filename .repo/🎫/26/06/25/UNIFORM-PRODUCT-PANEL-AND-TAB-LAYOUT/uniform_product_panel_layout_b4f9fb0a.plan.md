---
name: Uniform Product Panel Layout
overview: "Standardize every product onto one shell layout: Workbench (Hierarchy default + Catalogue), Details (Inspection default), Settings (Mode default + App + General), and fill every Inspection tree with fully-wired editable input controls bound to the live selection."
todos:
  - id: ticket
    content: Read repo://goals, associate, open repo-MCP ticket for uniform panel/tab layout
    status: completed
  - id: foundation-settings
    content: Rewrite createFrameworkSettingsPanelTabs into Mode/App/General tabs; extend SettingsHostApi with theme + app identity; add theme persistence in ui/react and apply in both views
    status: completed
  - id: foundation-playground
    content: Fix PlaygroundView to merge settings-kind tabs + augmentPanelTabs.settings and wire App-tab display host; add canonical Hierarchy/Catalogue/Inspection tab constants+icons
    status: completed
  - id: puzzle3d
    content: "Puzzle 3D: Kinds->Catalogue, Inspector->Inspection, remove settings tab (fold into App)"
    status: completed
  - id: puzzle2d-wires
    content: "Puzzle 2D + Wires: Kinds->Catalogue, Inspector->Inspection, move details Settings into App"
    status: completed
  - id: cad
    content: "CAD: Catalog->Catalogue, Selection->Inspection, make geometry selection editable"
    status: completed
  - id: presentation
    content: "Presentation: Tile->Inspection, add Catalogue tab"
    status: completed
  - id: puzzle5d
    content: "Puzzle 5D: editable Inspection for Part/Grip with new store patch commands; Kinds->Catalogue"
    status: completed
  - id: flow
    content: "Flow: add controller selection + canvas bridge; Hierarchy + Catalogue(Kinds+Extensions) + editable Inspection"
    status: completed
  - id: procedural
    content: "Procedural: Hierarchy + Catalogue + editable Inspection using existing selection"
    status: completed
  - id: dag-map-shooting
    content: "DAG, Map, Shooting: build Hierarchy + Catalogue + editable Inspection (shot/asset, layer/feature, node)"
    status: completed
  - id: sketchpad
    content: "Sketchpad: replace text-stack details with editable Inspection bound to routeSelection; ensure Hierarchy + Catalogue"
    status: completed
  - id: coda
    content: "CODA: re-architect custom Electron shell onto ProductShell with uniform tabs + single-selection model"
    status: completed
  - id: verify
    content: Build/typecheck/test affected packages, register launch.json commands, close ticket
    status: completed
isProject: false
---

# Uniform Product Panel & Tab Layout

Standardize all products onto one shell: Workbench [Hierarchy (default), Catalogue], Details [Inspection (default)], Settings [Mode (default), App, General], with fully two-way editable Inspection controls per product.

Work happens inside a new repo-MCP ticket (read `repo://goals` first, associate, `ticket_open`). All temp/log files go under the ticket folder. Code is added to existing files using regions.

## Phase 1 - Framework foundation (do first; unblocks all products)

Target files:
- [framework/product/platform/renderer/react/index.tsx](framework/product/platform/renderer/react/index.tsx)
- [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)
- [ui/react/index.tsx](ui/react/index.tsx)
- [framework/core/index.ts](framework/core/index.ts)

Settings -> 3 uniform tabs. Rewrite `createFrameworkSettingsPanelTabs` (~1621) to return Mode (order -300, default), App (-200), General (-100):
- Mode tab: extract the mode selector currently inside `buildFrameworkSettingsGeneralTree` (rows ~1576-1595) into its own tree from `SettingsHostApi.modes/activeModeId/setActiveModeId`. Always present (falls back to single pseudo-mode like the navbar does).
- App tab: app identity (`app.id`, resolved label, iconId), theme `Select` (system/light/dark), and named-layout management (reuse `DisplayHostApi.namedLayouts/saveCurrentLayout/applyNamedLayout/deleteUserLayout`). Product-specific behavior settings (e.g. puzzle selection method/mode) fold in here.
- General tab: keep compact, expertise, compute workers; remove the mode row.

Extend `SettingsHostApi` (~1486) with `theme`/`setTheme` and `appId/appLabel/appIconId`; pass `getDisplayHost` into the App tab factory (signature change to `createFrameworkSettingsPanelTabs`). Add `readStoredUiChromeTheme`/`writeStoredUiChromeTheme` in `ui/react` next to compact/expertise (~1821-1847); apply via `useElementsSurfaceChrome` in both `PlatformView` (~4521) and `PlaygroundView` (theme currently hardcoded `system`).

Playground settings/merge fix: extend `usePlaygroundViewShellData` (~1185-1211) and the `augmentPanelTabs` type (~1103) to also merge `panel === "settings"` and `augmentPanelTabs.settings`, matching `resolveAppPanelTabsByKind` (~4032). Wire the App-tab display host in `PlaygroundView` (it already builds display tabs ~1337-1340).

Shared tab identity: add canonical tab id/icon/label constants (Hierarchy, Catalogue, Inspection) so every product references the same names/icons via `shellTabIconComponent`.

## Phase 2 - Rename + restructure products already on the shell

For each: Workbench = Hierarchy (default) + Catalogue; Details = single Inspection tab; drop per-product Settings tabs (content moves to App).
- Puzzle 3D [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) (~3266): Kinds->Catalogue, Inspector->Inspection, remove the `panel:"settings"` tab (fold settings body into App).
- Puzzle 2D + Wires [puzzle/2d/play/index.ts](puzzle/2d/play/index.ts) and host `augmentPanelTabs` (~6006): Kinds->Catalogue, Inspector->Inspection; move the misplaced details "Settings" panel into App settings.
- CAD [cad/js/renderer/play/index.tsx](cad/js/renderer/play/index.tsx) (~3259): Catalog->Catalogue, Selection->Inspection.
- Presentation [framework/product/presentation/play/index.ts](framework/product/presentation/play/index.ts) (~602): Tile->Inspection; add a Catalogue tab (tile/figure templates).

## Phase 3 - Build missing Hierarchy / Catalogue / Inspection

Reuse the puzzle 2d/3d pattern: snapshot -> declarative tree of `field`/`input`/`select`/`vec3`/`numberStepper` controls with `onChange: cmd(...)` -> controller/host patch -> re-render.
- Puzzle 5D [puzzle/5d/play/index.ts](puzzle/5d/play/index.ts): replace read-only "Status" with editable Inspection for Part/Grip (add patch commands on `Puzzle5dStore`); Kinds->Catalogue.
- Flow [flow/play/index.ts](flow/play/index.ts): add `selectedNodeIds`+revision to `FlowPlayController`, bridge `FlowCanvas.onSelectionChange/selectedNodeIds` in `FlowPlayPaneSurfaceHost`; add Hierarchy (widgets/synapses from `fixtureJson`), fold Kinds+Extensions into Catalogue sections, add editable Inspection.
- Procedural [procedural/play/index.ts](procedural/play/index.ts): controller already has selection; add Hierarchy + Catalogue (Kinds+Extensions) + editable Inspection.
- DAG [mathematical/graph/.../dag/play/index.ts](mathematical/graph), Map [gis/map/play/index.ts](gis/map/play/index.ts), Shooting [shooting/play/index.ts](shooting/play/index.ts): build all three tabs. Shooting Inspection edits the active shot/asset (`setShotCamera`, `setActiveShotShape`, etc.); Map edits selected layer/feature; DAG edits selected node.

## Phase 4 - Sketchpad (PlatformView product)

[compose/client/lib/sketchpad/js/index.ts](compose/client/lib/sketchpad/js/index.ts): replace the static `SketchpadDetailsPanel` text-stack (~14348-14466) with an editable Inspection panel bound to `routeSelection` (pieces/connections) dispatching compose-react patch commands; ensure Hierarchy + Catalogue workbench tabs. Settings already flow through `PlatformView`.

## Phase 5 - CODA (heaviest; different paradigm)

[coda/client/ui/desktop/renderer.tsx](coda/client/ui/desktop/renderer.tsx) is a custom frameless Electron shell with 7 sidebar pages, MCP-resource data, no controller/canvas. Re-architect onto `ProductShell`:
- Workbench: Hierarchy = project/run/iteration tree (or existing `OntologyTree`/`ValidationTree`); Catalogue = frameworks/properties/platforms templates.
- Details: Inspection = selected validation node / property / iteration with editable inputs invoking MCP tools (`window.coda.tool`).
- Settings: Mode/App/General (project + config).
- Keep the custom title bar + Welcome gate; map the 7 pages to a center summary window + panel tabs. Introduce a single-selection model.

## Phase 6 - Verify

Build + typecheck affected packages via launch.json/nx targets; run existing test files (extend, do not add new ones) e.g. [ui/styling/js/index.test.ts](ui/styling/js/index.test.ts) and product tests; confirm runtime behaviour with `[DEBUG]` logs where selection wiring is new. Register any new executable commands in [.vscode/launch.json](.vscode/launch.json). Close ticket with summary + file list.

## Notes
- Infinite (`infinite/world/r3f`, `infinite/cavas`) is a library, not a product - no layout; its consumers (already listed) own the panels.
- "Catalogue" consolidates prior Kinds/Catalog/Extensions into one tab (multiple sections allowed).
- No legacy/compat shims; rename in place and fix all fixtures/hosts at once.