---
name: Sketchpad Platform Refactor
overview: Add a render-agnostic Component class document (Table/Puzzle2d/Puzzle3d/Puzzle5d/Cad/Panel) to @semio-tech/framework-platform-core, drive the React renderer generically from each Component's view-model, and rewrite compose/client/lib/sketchpad/js/index.ts as a pure-TypeScript Platform instance whose apps subclass those components (HomeTable, KitTable, KitDiagram, DesignScene/Diagram, TypeCad, Docs/Feedback panels) — deleting the legacy React-Router/GoldenLayout/XState/SketchpadStore shell.
todos:
 - id: core-components
   content: Add render-agnostic Component base class document (Table/Puzzle2d/Puzzle3d/Puzzle5d/Cad/Panel) + per-kind model contracts + ComponentRegistry on Platform in framework/product/platform/core/index.ts; extend inline vitest.
   status: completed
 - id: react-kind-renderers
   content: Add registerComponentKindRenderer + built-in generic per-kind renderers (driven by Component models) in framework/product/platform/renderer/react/index.tsx; wire surface resolution via Platform.getComponent; keep registerSurfaceBinding fallback; extend vitest.
   status: completed
 - id: sketchpad-domain-pure
   content: "Make sketchpad domain layer pure-TS: replace React-hook kit reads with @semio-tech/compose-react store-client subscriptions; convert shell/navigation/panel controllers to pure-TS Controllers driving Platform URI/active-app/panels."
   status: completed
 - id: sketchpad-kit-components
   content: Implement KitTable (Table) and KitDiagram (Puzzle2d) subclasses producing models from kit store client + route scope, porting MultiWindowApp/kit-diagram rendering logic.
   status: completed
 - id: sketchpad-design-type-components
   content: Implement DesignScene/DesignDiagram (Puzzle5d) and TypeCad (Cad) subclasses producing models, porting DesignApp/TypeApp/FiveD rendering logic.
   status: completed
 - id: sketchpad-home-docs-feedback
   content: Implement HomeTable (Table) and DocsPanel/FeedbackPanel (Panel) subclasses producing models, porting Home/Docs/Feedback rendering logic.
   status: completed
 - id: sketchpad-assembly-boot
   content: Rewrite buildSketchpadPlatform to register apps + component instances + pure-TS controllers; boot via React renderer mountPlatform; delete legacy Sketchpad/LayoutWrapper/*Config/*App/render*Surface/router/golden-layout/XState shell.
   status: completed
 - id: tests-verify
   content: Update embedded Playwright tests in index.ts for the new shell; run framework vitest + sketchpad Playwright; verify each app renders via Platform at runtime with [DEBUG] logs; confirm sketchpad definition is render-agnostic.
   status: completed
isProject: false
---

# Sketchpad → Render-Agnostic Platform Refactor

Work under the reopened ticket `2026/05/30/SKETCHPAD-PLATFORM-REFACTOR` (and `FRAMEWORK-CORE-ABSTRACTION` for the framework portion). Repo rules apply: no backwards compat, single `index.ts` per app, extend existing files with regions, keep tests in the existing files.

## Goal

- `compose/client/lib/sketchpad/js/index.ts` becomes a PURE TypeScript Platform instance (zero React/JSX).
- The framework exposes component base CLASSES; sketchpad SUBCLASSES them (`HomeTable extends Table`, etc.).
- React renderer renders each component kind generically from a render-agnostic model; a Svelte renderer can later read the same models.
- The legacy parallel shell (React Router, GoldenLayout `LayoutCanvas`, XState machines, `SketchpadStore`-as-shell, `SketchpadDeclarativeAppChrome`, all `*Config`/`*App` React components and `render*Surface` fns) is deleted.

## Architecture decisions

- The bulk of the work is the render-agnostic VIEW-MODEL per component kind. The React renderer (and later Svelte) must NOT depend on `@compose/*`; it consumes only the model. Therefore every interaction/affordance sketchpad needs (table columns/cell editors/row actions; diagram nodes/edges/port colors; 5d scene + topology; cad model; panel UiNode body) must be expressible in the model.
- Sketchpad component subclasses read domain data via `@semio-tech/compose-react` non-hook store clients (`createKitStoreClient`, `executeComposeKitCommand`, scopes) — these are framework-agnostic subscriptions, satisfying the `compose/sketchpad → @semio-tech/compose-react` layering rule without React hooks.
- The existing `registerSurfaceBinding(surfaceId, ReactComponent)` seam STAYS for playground/puzzle/cad play apps (out of scope). The new component-kind renderer layer is additive and is what sketchpad uses exclusively.

## Phase 1 — Framework core: Component class document

File: [framework/product/platform/core/index.ts](framework/product/platform/core/index.ts) (extend `🔖ComponentKind`/`🔖UiNode` regions; add `🔖Component` region).

- Add `abstract class Component` with `componentKind: ComponentKind`, `surfaceId`, `controllerId`, an `ObservableCell`-backed model, `subscribe(listener)`, `getModel()`, and `abstract buildModel(): TModel`.
- Add concrete base classes `Table`, `Puzzle2d`, `Puzzle3d`, `Puzzle5d`, `Cad`, `Panel`, each fixing `componentKind` and declaring its model contract:
  - `TableModel` (columns, rows, selection, sort, row/cell actions)
  - `Puzzle2dModel` (nodes, edges, port colors, selection, interactions)
  - `Puzzle3dModel` / `Puzzle5dModel` (scene + topology data)
  - `CadModel`; `PanelModel` (a `UiNode` body)
- Add a `ComponentRegistry` owned by `Platform` (extend `framework/core` `Platform` or wrap in platform/core): `registerComponent(component)`, `getComponent(surfaceId)`. Window-body builders already emit `surfaceId`; the registry maps it to the live instance.
- Extend the inline vitest suite in this file for the new classes/registry.

## Phase 2 — Framework React renderer: generic per-kind rendering

File: [framework/product/platform/renderer/react/index.tsx](framework/product/platform/renderer/react/index.tsx) (`ui-declarative-renderer` + `shell-bridge` regions).

- Add `registerComponentKindRenderer(kind, Component)` where the React component receives the `Component` instance, subscribes to its model via `useSyncExternalStore`, and renders generically:
  - `table` → `@semio-tech/ui-react` data table from `TableModel`
  - `puzzle2d` → `@semio-tech/puzzle-2d-react` board from `Puzzle2dModel`
  - `puzzle3d` → `@semio-tech/puzzle-3d-react`
  - `puzzle5d` → `@semio-tech/puzzle-5d-react` `FiveD` from `Puzzle5dModel`
  - `cad` → `@semio-tech/cad-js-renderer` from `CadModel`
  - `panel` → existing `UiRenderer` on `PanelModel.body`
- In `renderComponentHostSurface`/`renderBoundComponent`: resolve `node.surfaceId` → `platform.getComponent(surfaceId)` → kind renderer. Keep `surfaceBindingHosts` lookup first so playground still works.
- Provide built-in kind renderers for all six kinds; register them by default.
- Extend the inline vitest suite for the new resolution path.

## Phase 3 — Sketchpad domain layer made pure-TS

File: [compose/client/lib/sketchpad/js/index.ts](compose/client/lib/sketchpad/js/index.ts).

- Keep the pure-TS domain regions (sync interfaces, persistence, diff algebra, tutorials data, route-scope parsing) but remove their React coupling.
- Replace React-hook kit reads (`useKit*`, `useNavigation`, etc.) with subscriptions to `@semio-tech/compose-react` store clients / scopes used imperatively inside component classes.
- Turn `SketchpadShellController` and navigation/panel logic into pure-TS `Controller`s driving `Platform.setActiveAppId`/URI/panel visibility (no React Router; navigation is platform URI + `sketchpadAppIdFromPath`).

## Phase 4 — Sketchpad component subclasses (pure TS)

File: [compose/client/lib/sketchpad/js/index.ts](compose/client/lib/sketchpad/js/index.ts) (new `🔖SketchpadComponents` region).

- Implement, each computing its model from the kit store client + route scope:
  - `HomeTable extends Table`, `KitTable extends Table`
  - `KitDiagram extends Puzzle2d`
  - `DesignScene extends Puzzle5d`, `DesignDiagram extends Puzzle5d`
  - `TypeCad extends Cad`
  - `DocsPanel extends Panel`, `FeedbackPanel extends Panel`
- Port the rendering logic currently in `MultiWindowApp`/`DesignApp`/`TypeApp`/`render*Surface`/kit-diagram geometry into model production. This is the largest sub-task; split across generalists by app (kit, design, type, home/doc/feedback).

## Phase 5 — Platform assembly + boot; delete legacy shell

File: [compose/client/lib/sketchpad/js/index.ts](compose/client/lib/sketchpad/js/index.ts).

- `buildSketchpadPlatform()` (pure TS): construct `Platform`, register apps/modes/window-kinds (reuse the existing manifest at ~22200), instantiate the component subclasses, register them in the `ComponentRegistry`, wire pure-TS controllers.
- Boot: `index.ts` calls the React renderer mount in one statement (`mountPlatform(buildSketchpadPlatform(), root)` from `@semio-tech/framework-platform-renderer-react`) — no JSX in sketchpad. Svelte renderer later provides its own equivalent mount.
- Delete: `Sketchpad`, `LayoutWrapper`, `SketchpadDeclarativeAppChrome`, `SketchpadDeclarativeProductHost`, `AppRouter`, all `*Config`, `MultiWindowApp`, `DesignApp`, `TypeApp`, `DocsApp`, `Home`, `Feedback`, all `render*Surface` React fns, `SketchpadStore`-as-shell, XState shell machines, GoldenLayout shell usage, deprecated aliases.
- Update [index.html](compose/client/lib/sketchpad/js/index.html) only if the mount entry name changes (keep `#root`).

## Phase 6 — Tests, build, runtime verification

- Keep the ~8k lines of embedded Playwright tests at the bottom of `index.ts`; update selectors/flows broken by shell removal.
- Run framework vitest (`@semio-tech/framework-platform-core`, `@semio-tech/framework-platform-renderer-react`) and the sketchpad Playwright target.
- Verify runtime via dev server + console logs (prefix temporary logs with `[DEBUG] `) that each app (home/kit/design/type/doc/feedback) renders through the Platform with no legacy shell.
- Confirm `@semio-tech/compose-sketchpad` no longer imports `@semio-tech/framework-platform-renderer-react` for component implementations (only for the boot mount), keeping the definition render-agnostic.

## Out of scope (note for follow-ups)

- Migrating playground / puzzle / cad play apps off `registerSurfaceBinding` onto the component-class model.
- Implementing the Svelte renderer (tracked by `.repo/🎫/26/05/30/SKETCHPAD-PLATFORM-REFACTOR/follow-up-svelte-renderer.md`).
