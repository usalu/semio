---
name: Playground App Isolation Hardening
overview: Fix the misleading "22 plugins" build log for single-app playground dev, make plugin registry generation resilient to unrelated broken app crates, add automated guardrails that enforce app/host and app/app isolation going forward, and repair two dead references to the removed `framework/product/playground/...` path (one of which currently breaks a test that encodes exactly this isolation invariant).
todos:
 - id: log-clarity
   content: Clarify registry-catalog vs build-scope log messages in framework/plugin/registry/script.ts and framework/product/os/dev/script.ts; throw on empty filtered target list
   status: completed
 - id: fault-tolerant-registry
   content: Make generatePluginRegistry/parsePluginCargo skip-and-warn on malformed crate manifests instead of throwing for the whole catalog
   status: completed
 - id: lint-cross-plugin-dep
   content: Extend PluginCapabilityLintScript to fail on cross-plugin-crate Cargo dependencies
   status: completed
 - id: lint-host-awareness
   content: Extend PluginCapabilityLintScript to fail if plugin crate sources reference host-mode symbols (SEMIO_PLUGIN, PLAYGROUND_APP_KIND, studioMode, pluginFilter)
   status: completed
 - id: fix-stale-test
   content: Repoint repo/lib/js/index.test.ts playground-renderer isolation test to framework/renderer/react/os-shell.tsx + ui-interpreter.tsx and update boot-entrypoint assertion
   status: completed
 - id: fix-cad-vitest-config
   content: Remove dead framework/product/playground alias/consts from cad/renderer/js/vitest.config.ts
   status: completed
 - id: verify
   content: Re-run dev:puzzle:3d, program lint, and the repaired test to confirm fixes
   status: completed
isProject: false
---

# Playground App Isolation Hardening

## Root cause of the reported bug

`bun run dev:puzzle:3d` correctly builds **only** the `puzzle` plugin crate (verified from the live terminal transcript: only `Compiling puzzle-plugin v0.1.0` appears, then `[DEBUG] built program puzzle (wasm32-wasip2) -> …/plugin-modules/puzzle`, then Vite serves on the correct port `6013`). The `[DEBUG] generated plugin registry (22 plugins)` line comes from a separate, cheap **catalog** step (`framework/plugin/registry/script.ts` `GenerateScript`) that always parses every `*/plugin/rs/Cargo.toml` in the repo to produce the `pluginId → crate path` lookup table — it does not compile anything. That catalog step's log wording is what makes it _look_ like 22 apps are being built.

That said, this catalog step is a real (if narrow) isolation gap: `generatePluginRegistry` (`framework/plugin/registry/script.ts:53`) throws on the first malformed `Cargo.toml` (`parsePluginCargo` at line 42-51), which means **a single-app playground session for `puzzle3d` can be broken by an unrelated broken crate in, say, `shooting/plugin/rs`.** That directly violates "an app being developed shouldn't depend on any other app."

## Fixes

### 1. Clarify build-scope logging

- [`framework/plugin/registry/script.ts`](framework/plugin/registry/script.ts) `GenerateScript.run` (line 95): reword to `plugin registry catalog refreshed (N known plugin crates)`.
- [`framework/product/os/dev/script.ts`](framework/product/os/dev/script.ts) `buildPlugins()` (line 250-268): after computing `targets`, log the actual build scope, e.g. `[DEBUG] program build scope: puzzle (1/22 known plugins)`, and **throw a clear error** if a `filterPlugin` was given but resolves to zero targets (currently it silently builds nothing).

### 2. Make registry generation fault-tolerant

- [`framework/plugin/registry/script.ts`](framework/plugin/registry/script.ts) `generatePluginRegistry`/`parsePluginCargo`: catch per-crate parse errors, `console.warn` and skip that crate instead of throwing for the whole catalog, so one broken app never blocks another app's dev/build loop.

### 3. Add automated isolation guardrails (extend existing lint, no new files)

Extend `PluginCapabilityLintScript` in [`framework/product/os/dev/script.ts`](framework/product/os/dev/script.ts) (already runs via `bun ./📜️script.ts verify` and already walks `cargo metadata` for plugin crates):

- **No cross-plugin dependency**: fail if any `*/plugin/rs` package lists another plugin crate's package name as a direct dependency (cross-checked against the generated registry's `packageName` set, excluding itself).
- **No host-mode awareness**: fail if any plugin crate's `lib.rs`/module sources reference host/session concepts like `SEMIO_PLUGIN`, `PLAYGROUND_APP_KIND`, `studioMode`, `pluginFilter` — this locks in the already-clean "apps registered once, host-agnostic" property (verified 0 current matches) as an enforced invariant rather than convention.

### 4. Repair dead `framework/product/playground/...` references

The old pre-consolidation playground product directory no longer exists (host logic lives in `framework/renderer/react/os-shell.tsx` + `ui-interpreter.tsx` now). Two live files still point at it:

- [`repo/lib/js/index.test.ts`](repo/lib/js/index.test.ts) line 639-646 — test `"framework playground renderer has no per-technology registerUi surface host APIs"` reads a nonexistent file and will throw `ENOENT` whenever this suite runs. Repoint it at `framework/renderer/react/os-shell.tsx` (and/or `ui-interpreter.tsx`), keep the same regex assertion (no `registerUi<Tech>SurfaceHost` symbols — this _is_ the app-isolation invariant for the renderer host), and swap the removed `bootPlaygroundApp` check for the current boot entrypoint (`bootFrameworkOs`).
- [`cad/renderer/js/vitest.config.ts`](cad/renderer/js/vitest.config.ts) line 14, 26 — `rendererRoot`/`rendererIndex` and the `@framework/playground/renderer/react` alias point nowhere and are unused (confirmed no references in `cad/renderer/js/index.tsx`); delete the dead consts and alias entry.

### Out of scope (flagged, not fixed here)

- `framework/product/AGENTS.md` documents the same obsolete `framework/product/playground/dev` path — cannot edit `AGENTS.md` files per repo rules.
- `compose/client/lib/sketchpad/js/pw-loader.mjs` and `.storybook/main.ts` alias a whole tree of paths that no longer exist (`framework/product/platform/*`, `framework/product/playground/*`, `puzzle/{2d,3d,5d}/react`, `gis/2d/react`, `gis/2d/play`) — this is `compose`/storybook tooling territory, a separate and larger dead-code cleanup outside today's playground/app-isolation scope.

## Verification

- Re-run `bun run dev:puzzle:3d` and confirm the new `[DEBUG] program build scope: puzzle (1/22 known plugins)` line appears and only `puzzle-plugin` compiles.
- Run `bun ./📜️script.ts program lint` (via `framework/product/os/dev`) to confirm the new cross-plugin-dependency and host-awareness checks pass on the current (clean) codebase.
- Run the repaired `repo/lib/js/index.test.ts` test and confirm it passes against the current renderer host files.
