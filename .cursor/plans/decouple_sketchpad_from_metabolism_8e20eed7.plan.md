---
name: Decouple Sketchpad From Metabolism
overview: Make the sketchpad fully independent of the metabolism kit by replacing the hardcoded metabolism auto-seed/buttons/asset-root with a generic, env-driven multi-kit preload mechanism, keeping metabolism only in tests and in new launch configs. Stop bundling metabolism into the `js` and `play` builds.
todos:
 - id: ticket
   content: Read repo://goals and open a ticket for decoupling sketchpad from metabolism
   status: completed
 - id: preload-env
   content: Add COMPOSE_SKETCHPAD_PRELOAD_KITS define to js and play vite configs (replacing COMPOSE_SKETCHPAD_E2E)
   status: completed
 - id: preload-impl
   content: Replace metabolism auto-seed block with generic sketchpadPreloadKitUrls()/preloadSketchpadKits() in index.ts and wire init call
   status: completed
 - id: asset-base
   content: "Generalize asset root: add assetBaseUrl to registerKitStore/getKitAssetBaseUrl and thread through open/attach + sketchpadKitFileUrlById"
   status: completed
 - id: remove-buttons
   content: Remove metabolism/Nakagin command buttons, command cases, and shell command registrations from index.ts
   status: completed
 - id: play-build
   content: Remove metabolism.zip dev middleware and generateBundle emit from play vite.config.ts
   status: completed
 - id: tests
   content: Update unit tests for new preload/asset API and rewrite E2E tests + playwright webServer env to use preloaded kit
   status: completed
 - id: launch
   content: Add metabolism-preloading launch configs for sketchpad and sketchpad-play in .vscode/launch.json
   status: completed
 - id: verify
   content: Run dev (plain + metabolism), build both apps, grep for metabolism, run full test suite; close ticket
   status: completed
isProject: false
---

# Decouple Sketchpad From Metabolism Kit

## Problem

The metabolism kit is baked into sketchpad production code and builds:

- [compose/client/lib/sketchpad/js/index.ts](compose/client/lib/sketchpad/js/index.ts) hardcodes the metabolism fixture URL, kit id `f042c2a4-...`, asset root, auto-seed, and two "Open metabolism fixture / Nakagin filtered fixture" commands+buttons (lines ~10656-10685, ~11529-11580, ~14181-14182, ~15039, ~15123-15131, ~15254-15255, ~15664).
- [compose/client/lib/sketchpad/play/vite.config.ts](compose/client/lib/sketchpad/play/vite.config.ts) serves and emits `metabolism.zip` into the build (lines ~146-182). No code references `/metabolism.zip`, so it is dead weight.

## Goal

- No `metabolism` reference in non-test sketchpad code or in either build output.
- A generic mechanism to preload a flexible list of kits, opt-in via env.
- New launch configs (and the Playwright server) supply the metabolism kit URL through that env.
- Tests keep using the metabolism fixture (test regions are stripped from builds).

## 1. Generic multi-kit preload (env-driven)

Env var `COMPOSE_SKETCHPAD_PRELOAD_KITS` = comma/whitespace-separated kit URLs (empty/unset = preload nothing).

Expose to client via `define` in both [compose/client/lib/sketchpad/js/vite.config.ts](compose/client/lib/sketchpad/js/vite.config.ts) and [compose/client/lib/sketchpad/play/vite.config.ts](compose/client/lib/sketchpad/play/vite.config.ts), replacing the `COMPOSE_SKETCHPAD_E2E` define:

```ts
"import.meta.env.COMPOSE_SKETCHPAD_PRELOAD_KITS": JSON.stringify(process.env.COMPOSE_SKETCHPAD_PRELOAD_KITS ?? ""),
```

In `index.ts` `🔖KitHost` region, replace the metabolism-specific block (`SKETCHPAD_DEV_FIXTURE_METABOLISM_WIP_PATH/URL`, `SKETCHPAD_DEV_FIXTURE_KIT_URL`, `SKETCHPAD_DEV_FIXTURE_NAKAGIN_FILTERED_URL`, `ensureSketchpadDevFixtureKitLoaded`, `seedSketchpadDevFixtureKitIfEmpty`, hardcoded `metabolismKitId`) with generic helpers:

- `sketchpadPreloadKitUrls(): string[]` parses `import.meta.env.COMPOSE_SKETCHPAD_PRELOAD_KITS`.
- `preloadSketchpadKits(): Promise<void>` opens each URL via `openSketchpadKitFromImport(url, { kind: "fixture", navigate: false, assetBaseUrl: <dir of url> })`, skipping already-open kits.

Call sites:

- Init (~line 15664): replace `seedSketchpadDevFixtureKitIfEmpty()` with `void preloadSketchpadKits()`, gated only by `sketchpadPreloadKitUrls().length > 0` (drop the DEV/E2E gate; production build injects `""`).
- Navigation handler (~line 15034-15040): remove the DEV `ensureSketchpadDevFixtureKitLoaded(path)` lazy-load block (eager init preload covers it).

## 2. Generalize the hardcoded asset root

- Remove `SKETCHPAD_METABOLISM_KIT_ASSET_ROOT`.
- `sketchpadFixtureUrlFromKitRelativePath(relativePath, baseRoot)` takes the base as a parameter.
- Controller: `registerKitStore(kitId, store, { kind, assetBaseUrl })` stores `assetBaseUrl` in a new `kitAssetBaseUrls` map; add `getKitAssetBaseUrl(kitId)`. Thread `assetBaseUrl` through `attachSketchpadKitStore` / `attachSketchpadKit` / `openSketchpadKitFromImport`.
- `sketchpadKitFileUrlById(kit)` resolves a relative `row.path` against `getSketchpadShellController()?.getKitAssetBaseUrl(kit.id)`; when no base, leave the path untouched. The `representations/*.glb` to `/mesh/*` mapping (`sketchpadPuzzle3dMeshUrlForKitFile`) is metabolism-agnostic and stays.

## 3. Remove fixture command buttons/commands

In `index.ts` production code remove:

- the two `sketchpadPanelCommandButton("Open metabolism fixture"/"Open Nakagin filtered fixture", ...)` (~14181-14182),
- the `importFixtureKit` / `importNakaginFilteredKit` command cases (~15123-15131),
- the two `sketchpadShellCommand(...)` registrations (~15254-15255).

Generic kit opening (home dropzone / file open) is unaffected.

## 4. Stop bundling metabolism into builds

In [compose/client/lib/sketchpad/play/vite.config.ts](compose/client/lib/sketchpad/play/vite.config.ts) remove the `metabolismKitPath` `/metabolism.zip` middleware branch (~156-161) and the `generateBundle` emit of `metabolism.zip` (~172-182). The `js` config `/fixture/` dev middleware stays (dev-only `configureServer`, generic, not in build output) so preloaded fixture URLs resolve.

## 5. Tests (kept on metabolism, but updated for new API)

- Unit test "auto-seeds from nakagin filtered fixture URL" (~15881-15890, asserts removed constants): replace with a test of `sketchpadPreloadKitUrls()` parsing.
- `sketchpadKitFileUrlById` relative-path test (~16962-16968): register the kit on a controller with `assetBaseUrl: "/fixture/kit/dev/metabolism/wip/initialKit"` so it still asserts the metabolism-relative URL.
- E2E region (~17421-17493) currently clicks the removed buttons. Rewrite to rely on the preloaded kit (set via the Playwright server env below): assert the open kit appears on home / navigate to it via the kit row or navbar select instead of the command palette buttons.
- [compose/client/lib/sketchpad/js/playwright.config.ts](compose/client/lib/sketchpad/js/playwright.config.ts) `webServer.env`: replace `COMPOSE_SKETCHPAD_E2E: "1"` with `COMPOSE_SKETCHPAD_PRELOAD_KITS: "/fixture/kit/dev/metabolism/wip/initialKit/kit.compose.json"`.

## 6. New launch configs that preload the kit

In [.vscode/launch.json](.vscode/launch.json), add sibling configs next to the existing sketchpad dev entries (lines 308 and 432), following existing naming/grouping/order, each setting:

```jsonc
"env": {
  "COMPOSE_SKETCHPAD_PRELOAD_KITS": "/fixture/kit/dev/metabolism/wip/initialKit/kit.compose.json",
  "NX_WORKSPACE_DATA_DIRECTORY": "${workspaceFolder}/.nx/workspace-data-terminal"
}
```

- `🛠️dev🏘️compose✍️sketchpad🧪metabolism` -> `bun nx dev @semio-tech/compose-sketchpad`
- `🛠️dev🏘️compose✍️sketchpad🎛️play🧪metabolism` -> `bun nx run @semio-tech/compose-sketchpad-play:dev`

(Env is inherited by the nx -> vite child process and read in `vite.config` via `process.env`.)

## 7. Repo workflow

- Read `repo://goals`, then open a ticket (e.g. `Make Sketchpad Independent From Metabolism Kit`) via `ticket_open` before editing; put any temporary logs under the ticket folder; close with `ticket_close` listing touched files.

## Verification

- `bun nx dev @semio-tech/compose-sketchpad` (plain): home loads with no kit preloaded, no metabolism strings in the served bundle.
- metabolism launch config: metabolism kit preloads and renders pieces.
- `bun nx build @semio-tech/compose-sketchpad` and play build: grep output for `metabolism` -> none; play `dist` has no `metabolism.zip`.
- `bun nx test @semio-tech/compose-sketchpad` (vitest + Playwright) passes.

## Out of scope

- The showcase/docs MDX (`page/showcase/metabolism.mdx`, tutorial) is project content referencing external images, not the kit; left as-is.
