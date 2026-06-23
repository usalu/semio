---
name: Sketchpad Platform Refactor
overview: Rename framework/platform's ProductRuntime to a declarative Platform class with a fixed component vocabulary (table, puzzle2d, puzzle3d, puzzle5d, cad, panel), make the React renderer depend on @puzzle/* and @cad/* to provide those components built-in, and rewrite sketchpad as a single clean declarative `new Platform({...})` instance — migrating home table, kit table, and kit diagram (puzzle2d) end-to-end as the reference slice while keeping the remaining apps working and tracked as follow-ups.
todos:
  - id: open-ticket
    content: "Open repo MCP ticket: run search for prior sketchpad/platform tickets, read repo://goals to associate, reopen or ticket_open the platform-refactor ticket."
    status: completed
  - id: core-rename
    content: Rename ProductRuntime->Platform (+ ProductDefinition->PlatformDefinition, ProductSubscriber, SurfaceRouter methods) and add declarative `new Platform(def)` constructor in framework/platform/core/index.ts.
    status: completed
  - id: core-components
    content: Replace board/scene3d UiNode surface kinds with table/puzzle2d/puzzle3d/puzzle5d/cad/panel; update builders and isCanvasOnlyWindowBody/assertCanvasOnlyWindowBody.
    status: completed
  - id: renderer-components
    content: Add @puzzle/* and @cad/* deps to the react renderer; replace per-surface host registries with a built-in componentKind->component map + registerSurfaceBinding adapter seam; rename ProductView->PlatformView and update UiRenderer.
    status: completed
  - id: sketchpad-platform
    content: Replace ensureSketchpadDeclarativeShell/manifest/body/surface registration with buildSketchpadPlatform() returning new Platform({...}); render via PlatformView; update boot + Sketchpad export.
    status: completed
  - id: sketchpad-slice
    content: "Migrate the vertical slice cleanly: home table (table), kit table (table), kit diagram (puzzle2d) with kit-state data adapters via @compose/react."
    status: completed
  - id: sketchpad-rest
    content: Re-wire remaining apps (design/type/quality/doc/feedback) onto puzzle5d/cad/panel with thin adapters so they keep working; mark deep rewrites as follow-up tickets.
    status: completed
  - id: consumers-build
    content: Update desktop/vscode consumers and fix sketchpad package.json exports (index.tsx->index.ts) and project.json cwd (react->js); grep+fix all stale ProductRuntime/board/scene3d references.
    status: completed
  - id: tests-run
    content: Extend framework vitest inline tests and sketchpad embedded Playwright tests (no new files); run both and verify runtime with [DEBUG] logs.
    status: completed
  - id: close-ticket
    content: Remove [DEBUG] logs, close the ticket with summary and all touched files; open follow-up tickets for deferred apps and the svelte renderer.
    status: completed
isProject: false
---

# Sketchpad to Platform Refactor

## Locked decisions
- `ProductRuntime` -> renamed to `Platform`; sketchpad becomes one declarative `new Platform({...})` instance. Core stays pure TS, render-agnostic. No legacy names kept.
- React renderer (`@framework/platform/renderer/react`) depends on `@puzzle/2d|3d|5d/react` and `@cad/js/renderer` and registers them as built-in named component kinds.
- Component kinds replace `board`/`scene3d`: `table`, `puzzle2d`, `puzzle3d`, `puzzle5d`, `cad`, `panel`.
- Scope = architecture + one vertical slice: build the full Platform + all component kinds, migrate `home table`, `kit table`, `kit diagram` (puzzle2d) end-to-end; remaining apps stay working and become follow-up tickets.

## Target architecture

```mermaid
flowchart TB
  subgraph sketchpad ["compose/sketchpad (thin index.ts)"]
    def["new Platform(PlatformDefinition)"]
    adapters["per-surface data adapters (kit state -> component props)"]
  end
  subgraph core ["@framework/platform (pure TS)"]
    Platform[Platform]
    App[AppRuntime]
    Mode[ModeRuntime]
    WK["WindowKind -> componentKind"]
    Nodes["UiNode: table | puzzle2d | puzzle3d | puzzle5d | cad | panel"]
  end
  subgraph react ["@framework/platform/renderer/react"]
    PV[PlatformView]
    UR[UiRenderer]
    CM["component map: kind -> React component"]
  end
  subgraph comps ["component packages"]
    UiTable["@ui/react Table"]
    P2["@puzzle/2d/react"]
    P3["@puzzle/3d/react"]
    P5["@puzzle/5d/react"]
    CAD["@cad/js/renderer"]
  end
  def --> Platform
  Platform --> App --> Mode --> WK --> Nodes
  adapters --> CM
  Platform --> PV --> UR --> CM
  CM --> UiTable
  CM --> P2
  CM --> P3
  CM --> P5
  CM --> CAD
```

## A. framework/platform/core ([framework/platform/core/index.ts](framework/platform/core/index.ts))
- Rename `ProductRuntime` -> `Platform` (class at 666-721) and all sibling names: `ProductSubscriber` -> `PlatformSubscriber`, `ProductDefinition` -> `PlatformDefinition`, `WindowBodyViewContext.runtime` type, etc. Update `SurfaceRouter.flattenFromProductDefinition` -> `flattenFromPlatformDefinition`/`flattenFromPlatformApps`.
- Add a declarative constructor: `new Platform(def: PlatformDefinition)` that builds `AppRuntime`/`ModeRuntime`/`WindowKindRuntime` from data (reusing existing `PluginManifest`/`SurfaceRouter` machinery). Keep imperative `addApp` for plugins.
- Replace `board`/`scene3d` surface node kinds in the `UiNode` union (93-101) with `puzzle2d`/`puzzle3d`/`puzzle5d`/`cad` (keep `table`, `panel`). Each carries `componentKind`, `surfaceId`, `controllerId`, optional `paneId`, and an opaque `props?: JsonValue`/binding id.
- Replace builders `buildScene3dWindowBody`/`buildBoardWindowBody` (104-116) with `buildPuzzle2dWindowBody`/`buildPuzzle3dWindowBody`/`buildPuzzle5dWindowBody`/`buildCadWindowBody`; keep `buildTableWindowBody`.
- Update `isCanvasOnlyWindowBody`/`assertCanvasOnlyWindowBody` (118-133) and the error message to the new kind set.
- All new code uses `//#region 🔖...` structuring per repo rules; docstrings start with an emoji.

## B. framework/platform/renderer/react ([framework/platform/renderer/react/index.tsx](framework/platform/renderer/react/index.tsx))
- Add deps to its `package.json`: `@puzzle/2d/react`, `@puzzle/3d/react`, `@puzzle/5d/react`, `@cad/js/renderer`.
- Replace the four `registerUi*SurfaceHost` registries + dispatch (1354-1413, 1490-1538) with a single component-kind model:
  - Built-in `componentRenderers: Record<ComponentKind, React.ComponentType<ComponentProps>>` wiring `table`->`@ui/react` `Table`, `puzzle2d`->`@puzzle/2d/react`, `puzzle3d`->`@puzzle/3d/react`, `puzzle5d`->`@puzzle/5d/react` `FiveD`, `cad`->`@cad/js/renderer`.
  - `registerSurfaceBinding(surfaceId, adapter)` where the host supplies a data/props adapter (plain data in, component props out) keyed by `surfaceId`. This is the renderer-agnostic seam a future Svelte renderer reuses with the same core + its own component map.
- Rename `ProductView` -> `PlatformView` and update `UiRenderer` (1587) to instantiate `componentRenderers[node.componentKind]` with adapter output; update `mountReactApp` references.

## C. sketchpad rewrite ([compose/client/lib/sketchpad/js/index.ts](compose/client/lib/sketchpad/js/index.ts))
- Replace `ensureSketchpadDeclarativeShell()` + `buildSketchpadExtensionManifest()` + `registerSketchpadDeclarativeBodies()` + `registerSketchpadUiSurfaceHosts()` (~22081-22420) with a single `buildSketchpadPlatform(): Platform` returning `new Platform({...})` whose apps reference component kinds:
  - home: `home-main` -> `table`
  - kit: `table` -> `table`, `diagram` -> `puzzle2d`
  - design: `scene`/`diagram` -> `puzzle5d`; type -> `cad`; docs/feedback/quality -> `panel`/existing.
- Migrate the slice cleanly: home table, kit table, kit diagram. Implement `registerSurfaceBinding` adapters that read kit state (via `@compose/react`) and return `table`/`puzzle2d` props. Kit diagram is re-wired from today's FiveD-flat onto the `puzzle2d` component per the chosen mapping.
- Keep remaining apps (design/type/quality/doc/feedback) functional by binding them to `puzzle5d`/`cad`/`panel` with thin adapters wrapping their current React implementations; deeper clean-rewrite of those deferred to follow-up tickets.
- Render via `<PlatformView platform={...} />`. Update boot path and `Sketchpad` export. Reorganize regions so the file stays a single clean `index.ts`.

## D. consumers + build fixes
- [compose/client/ui/desktop/renderer.tsx](compose/client/ui/desktop/renderer.tsx): update app-config registration (note: `qualityConfig` is referenced but not exported — fix to match real exports).
- [compose/client/ui/vscode/webview.tsx](compose/client/ui/vscode/webview.tsx): update `ensureSketchpadDeclarativeShell` import to the new Platform entry.
- [compose/client/lib/sketchpad/js/package.json](compose/client/lib/sketchpad/js/package.json): fix `exports` (`./index.tsx` -> `./index.ts`).
- [compose/client/lib/sketchpad/js/project.json](compose/client/lib/sketchpad/js/project.json): fix `cwd` `compose/client/lib/sketchpad/react` -> `.../js` in setup/dev/test targets.
- Grep the repo for any other `ProductRuntime`/`ProductView`/`board`/`scene3d` references and update.

## E. tests + ticket
- Extend framework vitest inline tests in core/renderer (no new files) to cover `Platform` construction from `PlatformDefinition` and the new component kinds/validation.
- Extend the embedded Playwright tests in sketchpad `index.ts` (no new files) for home table, kit table, kit diagram. Run framework vitest and sketchpad Playwright; confirm runtime via console logs (`[DEBUG]` prefixed, removed after).

## Constraints / notes
- Per repo rules, `AGENTS.md` files are not editable, so `framework/platform/AGENTS.md` ("WindowKind (table, board, scene)") will remain slightly stale after the rename; the code is the source of truth.
- All work happens inside a repo MCP ticket; remaining-app clean rewrites become separate follow-up tickets.

## Follow-up tickets (deferred)
- Clean-rewrite design app (`puzzle5d` flat + spatial), type app (`cad`), quality app, docs app, feedback app onto the Platform component model.
- Svelte renderer (`@framework/platform/renderer/svelte`) implementing the same core + component map / surface-binding seam.