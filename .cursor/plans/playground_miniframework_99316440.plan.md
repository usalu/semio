---
name: Playground Miniframework
overview: Turn @elements/playground into a self-contained miniframework (its own react-neutral runtime + its own React renderer that depends only on @elements/ui), enforce that every side-panel tab is a Tree with sections and items via declarative classes, migrate elements scene play onto it (replacing the ad-hoc JSON details panel), and decouple @elements/board from @elements/playground.
todos:
  - id: ticket
    content: Open a repo ticket (repo MCP), read repo://goals, associate with the most appropriate goal.
    status: completed
  - id: pg-core
    content: Add playground's own react-neutral runtime core (CommandBus, Controller, ProductRuntime, AppRuntime, ModeRuntime, WindowKindRuntime, layout helpers, window/side-panel body registries, buildScene3dWindowBody, UiNode/WindowLayout/ToolItem/*ViewContext types); remove @elements/framework import and dependency.
    status: completed
  - id: pg-renderer
    content: Add playground's own React renderer (PlaygroundView, mountPlaygroundApp, useApp, side-panel registry, scene3d/table surface hosts, minimal canvas) depending only on @elements/ui; expose ./react export and deps.
    status: completed
  - id: pg-tree-classes
    content: Add declarative PureSidePanelTabDefinition + StaticTreePanelDefinition in playground and enforce every side-panel tab resolves to a Tree of sections+items (no content-only/JSON fallback).
    status: completed
  - id: scene-controller
    content: Retarget ScenePlayShellController and buildScenePlayAppRuntime in scene/play/index.ts to the playground core.
    status: in_progress
  - id: scene-panels
    content: Replace ad-hoc scene inspector/settings React panels with declarative Tree panel definitions (sections + items) for selected Objects/Vortices/Attractions and scene lists.
    status: pending
  - id: scene-render
    content: Render scene play via playground PlaygroundView (not framework-react ProductView); remove the always-on JSON details tab; convert remaining scene React.Component classes to function components.
    status: pending
  - id: board-decouple
    content: Decouple @elements/board from @elements/playground (use framework registerSidePanelBody, drop dependency + vitest aliases).
    status: pending
  - id: validate
    content: Extend existing vitest regions and run playground/scene/board suites; runtime-verify the tree details panel and absence of the JSON snippet; close the ticket with a summary.
    status: pending
isProject: false
---

# Playground Miniframework

## Goal

`@elements/playground` becomes a standalone miniframework for playgrounds: it owns its runtime core (no `@elements/framework` dependency) and its own React renderer (depends only on `@elements/ui`). Every panel tab is forced to be a `Tree` with sections + items through declarative classes. Elements scene play is migrated onto it, which removes the always-shown "same JSON snippet" details tab. `@elements/board` is decoupled from playground and stays purely on `@elements/framework(-react)`.

## Target architecture

```mermaid
graph TD
  ui["@elements/ui (pure React, no classes)"]
  pgcore["@elements/playground core (react-neutral runtime + Tree-panel classes)"]
  pgreact["@elements/playground/react (own renderer)"]
  scene["scene play (migrated)"]
  fwk["@elements/framework (+ -react)"]
  board["@elements/board"]

  pgreact --> ui
  pgreact --> pgcore
  scene --> pgreact
  board --> fwk
  fwk --> ui
```



Key rule confirmed with user: `@elements/ui` stays 100% pure React (no classes); `@elements/framework` and `@elements/playground` are the react-neutral abstraction layers. Playground gets its OWN copies of what it needs (no shared code with framework), its OWN renderer, and board drops its playground import.

## Phase 1 - Playground owns its runtime core

In [elements/lib/playground/index.ts](elements/lib/playground/index.ts), stop importing from `@elements/framework`. Add a react-neutral core (new region(s) in the same file, or a sibling `core` module under the package) duplicating only the subset playgrounds need from [elements/lib/framework/core/index.ts](elements/lib/framework/core/index.ts):

- `CommandBus`, `Controller`
- `ProductRuntime`, `AppRuntime`, `ModeRuntime`, `WindowKindRuntime`
- layout helpers `createDefaultLayout` / `createWindowLayout` and `WindowLayout` type
- window + side-panel body registries: `registerWindowBody` / `getWindowBodyFactory`, `registerSidePanelBody` / `getSidePanelBodyFactory`
- `buildScene3dWindowBody` and the `UiNode` union (scene3d/table/board/stack/text host-surface nodes used by playgrounds)
- the `ToolItem`, `WindowBodyViewContext`, `SidePanelBodyViewContext` types
- Update [elements/lib/playground/package.json](elements/lib/playground/package.json): remove the `@elements/framework` dependency.

## Phase 2 - Playground owns its React renderer + Tree enforcement

Add a renderer module to the playground package (e.g. `elements/lib/playground/react/index.tsx`) exposing `PlaygroundView`, `mountPlaygroundApp`, `useApp`, a side-panel body registry, and surface-host registration (scene3d/table), reusing pure-React primitives from `@elements/ui` (`Layout`, `SidePanel`, `Tree`, `Window`, `Navbar`, `Footer`, `TreeDataSection`/`TreeDataItem`). It must NOT import `@elements/framework` or `@elements/framework-react`.

- Provide the declarative classes `PureSidePanelTabDefinition` (abstract `resolveTab()`) and `StaticTreePanelDefinition` (wraps a `{ sections: TreeDataSection[] }`) - the same shape the board already references - living in the playground (react-neutral). Every side-panel tab is resolved to a `Tree` of sections + items; there is NO content-only/JSON fallback.
- Port the minimal canvas + `registerUiScene3DSurfaceHost`/`registerUiTableSurfaceHost` machinery the playground needs (currently in framework-react `UICanvas`); reuse `@elements/ui` where the primitive is already pure-React.
- Update package `exports` in [elements/lib/playground/package.json](elements/lib/playground/package.json) to expose both core (`.`) and renderer (`./react`); add `@elements/ui` + react deps.

## Phase 3 - Migrate elements scene play onto the playground

- [elements/lib/react/scene/play/index.ts](elements/lib/react/scene/play/index.ts): retarget `ScenePlayShellController` and `buildScenePlayAppRuntime` to the playground core (its `Controller`/`PlaygroundController`, `AppRuntime`, registries) instead of `@elements/framework`.
- [elements/lib/react/scene/index.tsx](elements/lib/react/scene/index.tsx): render via the playground renderer `PlaygroundView` instead of `@elements/framework-react`'s `ProductView`; register the scene3d surface host through the playground renderer.
- Replace the ad-hoc inspector/settings panels (`ScenePlayInspectorPanel`, `ScenePlaySettingsPanel`, `ScenePlayInspectorObjects/Vortices/Attractions`, lines ~4886-5318) with declarative `PureSidePanelTabDefinition` subclasses whose `tree` is `StaticTreePanelDefinition({ sections })`. Sections + items: one section per selected-kind group (Objects / Vortices / Attractions) with one tree item per selected entity (expandable to its editable fields), plus "Objects in scene" / "Attractions in scene" list sections, and a Settings tab section.
- This removes the always-on JSON snippet: the playground renderer has no `createDefaultAppDetailsTabs`/`AppPanelStatePreview` default (the source of the repeated `{"selection":{},"hover":{}}` in [elements/lib/framework/renderer/react/index.tsx](elements/lib/framework/renderer/react/index.tsx) lines 2072-2128, 2165).
- Keep `@elements/ui` scene components pure React (the user wants no classes in the react layer); convert the remaining `React.Component` classes in scene play (`PlaySurfaceFooter`, `PlaySceneCanvas*`, lines ~5435-5510) to function components as part of the migration.

## Phase 4 - Decouple board from playground

- [elements/lib/board/play/index.ts](elements/lib/board/play/index.ts): drop `import { registerPlaygroundSidePanelBodies } from "@elements/playground"` and call framework's own `registerSidePanelBody` directly in `registerBoardPlayDeclarativeBodies` (it already imports `registerSidePanelBody` from `@elements/framework`).
- [elements/lib/board/package.json](elements/lib/board/package.json): remove the `@elements/playground` dependency.
- Remove the `@elements/playground` aliases from [elements/lib/board/vitest.config.ts](elements/lib/board/vitest.config.ts) and [elements/lib/board/play/vitest.config.ts](elements/lib/board/play/vitest.config.ts).

## Validation

- Run playground, scene, and board vitest suites (extend existing `index.ts` test regions; no new test files): assert playground core has no `@elements/framework` import, scene play resolves to a Tree-only details panel (no JSON section), and board still registers its bodies without `@elements/playground`.
- Manually confirm at runtime (console logs with `[DEBUG]` prefix) that selecting an object in scene play renders a sections+items tree and that the repeated JSON snippet tab is gone.
- Follow repo ticket flow (open a ticket via repo MCP, structure new code with regions, close with summary).

## Out of scope / follow-up

- Board's own undefined `PureSidePanelTabDefinition`/`StaticTreePanelDefinition` references (currently in [elements/lib/board/index.tsx](elements/lib/board/index.tsx) lines 8516-8553) are a pre-existing framework-side gap; converting board's panels to the framework's tree mechanism is a separate effort unless you want it folded in.

