---
name: Auto-Derive Playgrounds From Examples
overview: 'Eliminate hand-written `core/playground.ts` across all 24 playground apps by introducing a generic, config-driven playground factory in `framework-playground-core`, and split the conflated "fixture" concept into two distinct, uniformly-implemented concepts: internal test **fixtures** and user-facing **examples** that auto-populate the navbar dropdown.'
todos:
 - id: core-example-api
   content: Rename Fixture→Example API surface in framework/product/playground/core/index.ts and add loadPlaygroundExampleCatalog glob helper
   status: completed
 - id: core-factory
   content: Add generic createPlaygroundApp(config) factory to framework-playground-core replacing per-app Playground subclass + PlaygroundAppDefinition boilerplate
   status: completed
 - id: shared-plumbing
   content: Rename NavbarFixtureSelect, PLAYGROUND_LOCKED_FIXTURE_ID, and related plumbing in ui/react, ui/styling/vite-elements-assets.ts, repo/lib/js/index.ts
   status: completed
 - id: registry-devtools
   content: Update app-registry.ts to import base packages (no /playground subpath), drop the alias in dev/vite.config.ts
   status: completed
 - id: migrate-apps
   content: "Migrate all 24 apps: fixture/->example/ dirs, delete playground.ts, consolidate into index.ts Play region, rename fixture-* identifiers"
   status: completed
 - id: materialize-examples
   content: Author example/*.json for apps that only had inline defaults (sequence, imperative, lowpoly, flow, vcs, presentation, dag, wires, layout, gis graph, cad, trinity/rewrite) and wire their dropdowns
   status: completed
 - id: validate
   content: Build all 24 playground apps, run full test/typecheck suite, spot-check dropdowns and locked-example builds in the browser
   status: completed
isProject: false
---

# Auto-Derive Playgrounds From Examples

## Problem today

Every technology (`draw`, `note`, `s`, `puzzle/5d`, `forms`, `flow`, …, 24 in total) hand-writes a nearly-identical `core/playground.ts`:

```62:99:draw/core/playground.ts
export class PlaygroundDraw extends Playground {
	readonly id = DRAW_PLAY_APP_ID;
	createRuntime(): Platform { /* boilerplate identical in shape across all apps */ }
	registerBodies(): void { registerDrawPlayDeclarativeBodies(); }
}
export const drawPlayAppDefinition: PlaygroundAppDefinition = { id, label, controllerId, modes, createPlayground, bootRenderer, devHost };
```

Plus near-duplicate glob/fixture-catalog boilerplate (`createDrawPlayFixtureHost`, `drawFixtureIdFromGlobPath`, `drawFixtureLabelFromId` — byte-for-byte the same shape in [draw/core/playground.ts](draw/core/playground.ts) and [note/core/playground.ts](note/core/playground.ts)).

Today "fixture" conflates two different concerns:

- **Test data** — inputs that exist purely to drive automated-test assertions.
- **User-facing samples** — JSON documents that populate the playground navbar dropdown ("Examples").

Per investigation, most apps reuse the _same_ JSON file for both purposes, and several apps (`sequence`, `imperative`, `lowpoly`, `flow`, `vcs`, `presentation`, `dag`, `wires`, `layout`) have **no dropdown at all** — just an inline TS default with no navbar picker.

## Target architecture

### 1. Shared "Example" mechanism (replaces "Fixture" playground API)

In [framework/product/playground/core/index.ts](framework/product/playground/core/index.ts), rename the whole fixture surface to "example" and add a single glob-driven catalog loader so apps stop hand-rolling id/label derivation:

- `PlaygroundFixtureHost` → `PlaygroundExampleHost` (`getExampleCatalog()`)
- `PlaygroundFixtureCatalog` / `PlaygroundFixtureOption` → `PlaygroundExampleCatalog` / `PlaygroundExampleOption`
- `PLAYGROUND_NO_FIXTURE_ID` / `PLAYGROUND_NO_FIXTURE_OPTION` → `PLAYGROUND_NO_EXAMPLE_ID` / `PLAYGROUND_NO_EXAMPLE_OPTION`
- `eagerPlayFixtureGlob` → `eagerPlayExampleGlob`
- `playgroundLockedFixtureId` / `isPlaygroundFixtureLocked` / `playgroundResolvedFixtureId` → `*ExampleId`/`isPlaygroundExampleLocked` (env var `PLAYGROUND_LOCKED_FIXTURE_ID` → `PLAYGROUND_LOCKED_EXAMPLE_ID`)
- `resolvePlaygroundFixtureCatalog` → `resolvePlaygroundExampleCatalog`
- **New**: `loadPlaygroundExampleCatalog(globPattern, jsonSuffix, defaultId)` — does what every app currently hand-writes (`eagerPlayExampleGlob` → derive id from filename → Title-Case label → sort → build `{ defaultId, options, jsonById }`) in one call.

### 2. Generic `createPlaygroundApp(config)` factory (removes the per-app `Playground` subclass + `*PlayAppDefinition` object)

New export alongside `Playground`/`PlaygroundAppDefinition`:

```ts
export interface PlaygroundAppConfig extends AppDefinition {
 readonly keybindings?: readonly PlaygroundKeybinding[];
 readonly createRuntime: () => Platform;
 readonly registerBodies: () => void;
 readonly registerSurfaceHosts?: () => void;
 readonly bootRenderer: (pg: Playground, rootId?: string) => void | Promise<void>;
}

export function createPlaygroundApp(config: PlaygroundAppConfig): PlaygroundAppDefinition {
 class ConfiguredPlayground extends Playground {
  readonly id = config.id;
  readonly keybindings = config.keybindings;
  createRuntime = config.createRuntime;
  registerBodies = config.registerBodies;
  registerSurfaceHosts = config.registerSurfaceHosts ?? (() => {});
 }
 return { ...config, createPlayground: () => new ConfiguredPlayground(), bootRenderer: config.bootRenderer };
}
```

Every app now needs exactly one call to `createPlaygroundApp({...})` — no subclass, no separate file. This also **removes the circular-export problem** that `fix-playground-circular-exports.ts` worked around (`playground.ts` importing from `index.ts` and vice versa), since everything lives in one file.

### 3. `core/playground.ts` is deleted everywhere; `core/index.ts` owns it all

Each app's `index.ts` gets a trailing region:

```ts
//#region 🔖️Play
const drawExampleCatalog = loadPlaygroundExampleCatalog("../example/*.draw.json", ".draw.json", "semio");

export const drawPlayAppDefinition = createPlaygroundApp({
  id: "draw-play", label: "Draw", controllerId: "draw-play",
  modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit",
  keybindings: [{ key: "ctrl+a,meta+a", controllerId: "draw-play", command: "selectAll" }],
  createRuntime: () => { /* same body as today's createRuntime */ },
  registerBodies: () => registerDrawPlayDeclarativeBodies(),
  bootRenderer: async (pg) => (await import("@semio-tech/framework-playground-renderer-react/draw")).bootDrawPlay(pg),
  devHost: { playEntryKind: "draw", resolveDedupe: [...], optimizeDeps: {...} },
});
//#endregion 🔖️Play
```

- `<app>/core/package.json` drops the `"./playground"` export.
- [framework/product/playground/dev/vite.config.ts](framework/product/playground/dev/vite.config.ts) drops the `@semio-tech/${packageRoot}-core/playground` alias (only the `-core` alias remains, now resolving everything).
- [framework/product/playground/core/app-registry.ts](framework/product/playground/core/app-registry.ts) imports `@semio-tech/<pkg>-core` directly (no `/playground` subpath) — it stays a manual, static list of 24 branches because Vite requires literal `import()` strings for per-app tree-shaking; that constraint is orthogonal to "no playground.ts".

### 4. Fixture vs Example: complete split, enforced everywhere

- **Example** (`example/` directory per app, e.g. `draw/example/semio.draw.json`) — curated, user-facing sample documents. `loadPlaygroundExampleCatalog` globs this directory and _is_ the dropdown. Every app gets one, even those that today only have an inline `*_PLAY_DEFAULT_FIXTURE` TS constant (`sequence`, `imperative`, `lowpoly`, `flow`, `vcs`, `presentation`, `dag`, `wires`, `layout`'s orphaned `sample.layout.json`, `gis/2d`'s graph sample, `cad`, `trinity/rewrite`) — these inline constants get materialized into real `example/*.json` files and the dropdown is wired up for the first time, then the inline TS duplicate is deleted.
- **Fixture** (kept, narrowed) — reserved strictly for automated-test inputs that are not meant to be user-facing (e.g. `repo/lib/js` bundle-path tests, `compose/assets` VS Code extension test fixtures already correctly named). Where a test today asserts against playground sample content (e.g. draw's "renders the semio emblem…" vitest in `playground.ts`), the test now reads directly from the `example/` catalog — using an example as a regression input is fine; it just stops being _called_ a fixture.
- Rename `fixture-slugs.ts` → `example-slugs.ts` per app, `*_PLAY_FIXTURE_DEFAULT_ID` → `*_PLAY_EXAMPLE_DEFAULT_ID`, `PlayFixtureHostConfig` → `PlayExampleHostConfig`, controller command `setActiveFixture` → `setActiveExample`.
- Shared UI: `NavbarFixtureSelect` (in `ui/react` / `@semio-tech/ui-react`, consumed by [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)) → `NavbarExampleSelect`.
- [ui/styling/vite-elements-assets.ts](ui/styling/vite-elements-assets.ts) and [repo/lib/js/index.ts](repo/lib/js/index.ts) (`playgroundPlayViteDefine`, `PLAYGROUND_LOCKED_FIXTURE_ID` embedding, `LOCKED_FIXTURE_ENV`) get the same rename.

## Per-app migration recipe (applied identically to all 24 technologies)

`draw`, `note`, `writer`, `forms`, `raster`, `shooting`, `procedural/2d`, `procedural/3d`, `gis/2d`, `puzzle/2d`, `puzzle/3d`, `puzzle/5d`, `layout`, `sequence`, `imperative`, `lowpoly`, `flow`, `mathematical/graph/port/directed/dag`, `reasoning/mindmap/wires`, `trinity/jack/host-core`, `trinity/rewrite`, `s`, `vcs`, `framework/product/presentation`, `cad/js/renderer`.

1. Rename/create `fixture/` → `example/` (git mv where it exists; author JSON from the current inline default where it doesn't).
2. Delete `core/playground.ts`. Move its contents into `core/index.ts`'s new `//#region 🔖️Play` using `createPlaygroundApp` + `loadPlaygroundExampleCatalog` (or a custom `createRuntime`/example-loading hook for outliers like `puzzle/5d`'s fetch-by-URL loader and `s`'s cross-app aggregation of `draw`/`writer`/`note` examples — these keep bespoke loading logic but still collapse into `index.ts`, no separate file).
3. Rename fixture-flavored identifiers (`fixture-slugs.ts`, `*_FIXTURE_*`, `setActiveFixture`, `PlayFixtureHostConfig`) to their example equivalents; update every inline `if (import.meta.vitest)` block accordingly.
4. Drop `"./playground"` from `core/package.json` exports.
5. Convert puzzle 2d/3d/5d, gis/2d, cad, trinity's direct-import fixture patterns to the standard `loadPlaygroundExampleCatalog` glob convention for uniformity, keeping bespoke runtime-loading (fetch/disk) only where the app's _production_ runtime (not just the playground) genuinely needs it (puzzle/5d).

## Validation

- `bun run build --app <kind>` for all 24 kinds (mirrors the prior RESTORE-PLAYGROUNDS-APP-SPLIT verification in [.repo/🎫️/26/07/02/RESTORE-PLAYGROUNDS-APP-SPLIT/restore-playgrounds-verify-log.md](.repo/🎫️/26/07/02/RESTORE-PLAYGROUNDS-APP-SPLIT/restore-playgrounds-verify-log.md)).
- `bun test` across every touched package (playground-core, each app's core, ui-react, repo/lib/js).
- Typecheck (`tsc --noEmit` via nx) repo-wide since this touches shared exports consumed everywhere.
- Spot-check a locked-fixture-style build (`PLAYGROUND_LOCKED_EXAMPLE_ID`) still renders the single-example host correctly (puzzle 5d screenshot builds depend on this).
- Open 2-3 dev playgrounds (draw, s, a newly-wired one like sequence) in the browser to confirm the navbar dropdown now lists examples correctly.

## Process note

Per repo rules this work happens inside an MCP ticket (read `repo://goals` first, likely reopening or extending the `RESTORE-PLAYGROUNDS-APP-SPLIT` lineage under a new ticket since scope changed materially), with all temp logs/scripts kept in that ticket folder, and edits made via regions in existing files (no new files besides the per-app `example/*.json` content and the deleted `playground.ts`).
