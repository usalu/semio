---
name: Restore Playgrounds App Split
overview: Fix the build-breaking bugs blocking every playground, migrate `cad` into the shared `PlaygroundAppDefinition` registry, then refactor all 24 technologies so `core/playground.ts` only wires fixtures/dev-host while `core/index.ts` owns the actual app (Controller, selection, hover, tools, panels, windows, `build<X>WorkflowDefinition`) so each app truly runs standalone as a playground or hosted inside `os`/`s`.
todos: []
isProject: false
---

# Restore Playgrounds + Split App From Playground

## Ticket

Work happens inside the existing (reopened) ticket `.repo/🎫️/26/07/02/FIX-KIT-WIRES-FORCE-GRAPH-ON-VFS-UNFOLD` doesn't fit — this is new scope, so open a fresh ticket under goal `🎯️framework🎯️playground` (check `repo://goals` first) before starting, e.g. `SPLIT-PLAYGROUND-APP-ARCHITECTURE`.

## Phase 0 — Unblock every playground (root cause first)

1. **CSS import typo** blocks all 23 registered playgrounds identically (`[@tailwindcss/vite] Missing "./tailwind.css" specifier`). Fix [framework/product/playground/dev/globals.css](framework/product/playground/dev/globals.css):

   ```css
   @import "@semio-tech/ui-styling/ui.css";
   ```

   (`ui/styling/js/package.json` exports `./ui.css`, not `./tailwind.css` — confirmed the only reference to this wrong specifier in the repo.)

2. **cad vite config syntax error** — [cad/js/renderer/play/vite.config.ts](cad/js/renderer/play/vite.config.ts) is missing its `export default createPlaygroundPlayViteConfig({` opening line (dangling object literal starting at `playEntryKind: "cad",`). This blocks any `nx build` that transitively depends on `@semio-tech/cad-js-renderer`. Fix it as part of Phase 1 migration (file gets rewritten anyway) or standalone first if Phase 1 is deferred.

3. Re-verify: `cd framework/product/playground/dev && bun ./📜️script.ts build --app <each of the 23 entries>` should all succeed (bypasses nx's unrelated `^build` fan-out; use this instead of `nx run …:build` for fast iteration). Entries: `2d 3d 5d gis-2d wires draw writer raster forms flow dag imperative sequence layout lowpoly procedural-2d procedural-3d shooting s vcs trinity-jack trinity-rewrite presentation`. Fix any remaining app-specific compile errors surfaced (there may be a few beyond the CSS bug — the migration ticket touched every `playground.ts`).

4. Smoke-test a representative sample with `dev` (vite dev server + fetch) to confirm runtime boot, not just build, since `BuildScript` in `framework/product/playground/dev/script.ts` currently skips `devHost.prebuild` (only `DevScript` calls it) — check whether any app's build silently misses fixture assets because of this and fix if so.

## Phase 1 — Migrate `cad` into the shared playground registry

`cad` has no `core` package; its entire app (fixtures, `CadPlayShellController`, document/inspector builders, `buildCadWorkflowDefinition`, and the React root `CadPlayRoot`) lives in one 190k-character file [cad/js/renderer/play/index.tsx](cad/js/renderer/play/index.tsx).

- Create `cad/js/renderer/core/{index.ts,playground.ts,package.json,project.json,vitest.config.ts}` following the `draw/core` package as the template (`@semio-tech/cad-js-core` is already taken by the spatial model package, so name this one distinctly, e.g. `@semio-tech/cad-js-renderer-core`, nested under the existing `cad/js/renderer` workspace umbrella like `framework/product/platform/{core,renderer/react}` does).
- Split `play/index.tsx` per the Phase 2 rule below: Controller/tools/document/inspector/`buildCadWorkflowDefinition` → `core/index.ts`; fixture ids/options/`devHost`/`createPlayground`/`bootRenderer`/`cadPlayAppDefinition` → `core/playground.ts`; pure-JSX (`CadPlayRoot`, `registerCadPlaySurfaceHosts`) → `cad/js/renderer/react/index.tsx` (new, parallel to other technologies' `*/react` packages) or merge into existing `cad/js/renderer/index.tsx`.
- Register `cadPlayAppDefinition` in [framework/product/playground/core/app-registry.ts](framework/product/playground/core/app-registry.ts).
- Add `cad` to `PACKAGE_ROOT_BY_ENTRY`/`ENTRY_TO_HOST` in [framework/product/playground/dev/script.ts](framework/product/playground/dev/script.ts).
- Add `dev:cad`/`dev:cad:*` fixture scripts to root `package.json` (mirroring `dev:puzzle:3d:concrete-forest`) and a matching `resolvePlaygroundDevApp` case in [script.ts](script.ts).
- Update `.vscode/launch.json` cad dev entries to the shared `bun ./📜️script.ts dev cad` path, and remove/retire `cad/js/renderer/script.ts`'s standalone `play/vite.config.ts`-based dev/build once folded into the shared runner.
- Delete `cad/js/renderer/play/` once migrated (matches precedent: the original migration ticket deleted all 23 migrated `*/play` directories).

## Phase 2 — Split app vs. playground for all 24 technologies

**Target end-state per technology**, using `draw/core` (has `internal.ts` for pure domain model) and `puzzle/3d/core` (has none) as the two shapes to generalize from:

- `core/index.ts` ("the app" — used by both the playground harness and `s`/os hosting):
  - The `Controller` subclass (selection, hover payload handling, `toggleSelectableKind`/`toggleVisibleKind`-style commands).
  - Tools, panel tabs, and all window/panel declarative body builders (document trees, inspector trees, settings trees, kinds trees).
  - `build<X>AppRuntime` / `build<X>Runtime` / `register<X>DeclarativeBodies` / `register<X>SurfaceHosts`.
  - `build<X>WorkflowDefinition()` (os/`s` wiring) — already here in most technologies; keep and simplify since it can now reference the app pieces directly instead of importing the whole `<x>PlayAppDefinition`.
  - Pure domain-model types/helpers stay wherever they already live (`internal.ts` if the technology has one — don't relocate those, they're already correctly separated).
  - Embedded `if (import.meta.vitest)` tests for everything above (move the corresponding tests out of `playground.ts`).
- `core/playground.ts` ("the playground" — fixtures + dev-host wiring only):
  - Fixture catalog constants/options, default fixture id, `resolve<X>PlayFixtureSlug`, fixture JSON load/serialize helpers.
  - `devHost` config (`playEntryKind`, `prebuild`, etc.).
  - `createPlayground()` / `bootRenderer()` gluing the app (from `index.ts`) to the selected fixture.
  - The `<x>PlayAppDefinition: PlaygroundAppDefinition` export itself.
  - Embedded tests scoped to fixture selection/resolution only.

**Rule of thumb**: if the code would need to change when swapping in a different fixture, it's a playground concern → stays in `playground.ts`. If it defines _how the app behaves_ regardless of which fixture is loaded, it's app logic → moves to `index.ts`.

Do this per technology, verifying after each one: `bun nx run <pkg>:test` passes, `bun ./📜️script.ts build --app <entry>` (from `framework/product/playground/dev`) still succeeds, and (where a `build<X>WorkflowDefinition` test exists in `s/core` or `s/play`) the `s` extension loading test still passes.

Technologies (23 existing + cad from Phase 1), roughly smallest-first to build momentum before the largest files:
`imperative`, `reasoning/mindmap/wires`, `vcs`, `layout`, `writer`, `trinity/jack/host-core`, `mathematical/graph/port/directed/dag`, `trinity/rewrite`, `lowpoly`, `framework/product/presentation`, `sequence`, `s`, `forms`, `shooting`, `raster`, `draw`, `flow`, `gis/2d`, `procedural/2d`, `puzzle/5d` (+ `puzzle/5d/react` which currently also carries force-graph merge logic — check if any of that belongs in `index.ts` too), `procedural/3d`, `puzzle/2d`, `puzzle/3d`, `cad`.

## Phase 3 — Full verification

- Build every playground app via the direct script path (bypasses unrelated nx fan-out): `2d 3d 5d gis-2d wires draw writer raster forms flow dag imperative sequence layout lowpoly procedural-2d procedural-3d shooting s vcs trinity-jack trinity-rewrite presentation cad`.
- Run the full test suite for every touched package (`bun nx run-many -t test` scoped to touched projects, or `bun ./📜️script.ts test` for full confidence).
- Spot-check `s`/os integration boots the same apps with the same behavior as the playground (selection/hover/tools/panels/windows/options identical, since it's now literally the same `index.ts` app in both hosts).
- Update the two AGENTS.md files that currently describe the old shape ([framework/product/playground/AGENTS.md](framework/product/playground/AGENTS.md) says apps boot "using `PlaygroundAppDefinition` exports from each technology `core/playground.ts`" — clarify that the definition lives in `playground.ts` but the app itself lives in `index.ts`) — **do not edit `AGENTS.md` files per the workspace rule**; skip this if it would require editing a protected file, just leave a note in the ticket summary instead.
- Close out via `ticket_close` with a summary listing every file touched.
  </plan>
  <todos>[{"id":"fix-css-import","content":"Fix framework/product/playground/dev/globals.css tailwind.css -> ui.css typo blocking all playgrounds"},{"id":"fix-cad-vite-config","content":"Fix dangling object literal / missing export default in cad/js/renderer/play/vite.config.ts"},{"id":"verify-all-builds-baseline","content":"Build all 23 registered playground apps directly via framework/product/playground/dev script.ts and fix any remaining app-specific compile errors"},{"id":"migrate-cad-registry","content":"Create cad core/playground split, register cadPlayAppDefinition in app-registry.ts, wire dev script/root script.ts/launch.json, delete old cad/js/renderer/play"},{"id":"split-imperative","content":"Split imperative/core: move Controller/app logic to index.ts, leave fixtures in playground.ts"},{"id":"split-wires","content":"Split reasoning/mindmap/wires/core app vs playground"},{"id":"split-vcs","content":"Split vcs/core app vs playground"},{"id":"split-layout","content":"Split layout/core app vs playground"},{"id":"split-writer","content":"Split writer/core app vs playground"},{"id":"split-trinity-jack","content":"Split trinity/jack/host-core app vs playground"},{"id":"split-dag","content":"Split mathematical/graph/port/directed/dag/core app vs playground"},{"id":"split-trinity-rewrite","content":"Split trinity/rewrite/core app vs playground"},{"id":"split-lowpoly","content":"Split lowpoly/core app vs playground"},{"id":"split-presentation","content":"Split framework/product/presentation/core app vs playground"},{"id":"split-sequence","content":"Split sequence/core app vs playground"},{"id":"split-s","content":"Split s/core app vs playground"},{"id":"split-forms","content":"Split forms/core app vs playground"},{"id":"split-shooting","content":"Split shooting/core app vs playground"},{"id":"split-raster","content":"Split raster/core app vs playground"},{"id":"split-draw","content":"Split draw/core app vs playground (keep internal.ts as-is)"},{"id":"split-flow","content":"Split flow/core app vs playground"},{"id":"split-gis-2d","content":"Split gis/2d/core app vs playground"},{"id":"split-procedural-2d","content":"Split procedural/2d/core app vs playground"},{"id":"split-puzzle-5d","content":"Split puzzle/5d/core (and puzzle/5d/react force-graph logic) app vs playground"},{"id":"split-procedural-3d","content":"Split procedural/3d/core app vs playground"},{"id":"split-puzzle-2d","content":"Split puzzle/2d/core app vs playground"},{"id":"split-puzzle-3d","content":"Split puzzle/3d/core app vs playground (largest file, 6164 lines)"},{"id":"split-cad","content":"Split cad's newly-created core app vs playground"},{"id":"final-verification","content":"Build+test all 24 playgrounds and verify s/os hosting still works identically, close ticket"}]
