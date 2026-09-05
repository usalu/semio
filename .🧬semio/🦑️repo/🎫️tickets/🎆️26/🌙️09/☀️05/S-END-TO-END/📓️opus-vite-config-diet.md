# Lane E — `vite-config-diet`

Opus implementer report for ticket `26/09/05/S-END-TO-END`. All numbers below are measured on this machine on 2026-09-05 (session interrupted ~06:00 by an API limit and resumed 13:30 after a reboot; load average ~300 throughout the resumed half).

## Result in one line

`⚙️vite.config.ts`'s bundled config graph went from **131 modules / 4 004 648 source bytes** to **32 modules / 473 472 source bytes**; the served React shell now reports `VITE ready in 10136 ms` (was `415982 ms`), and touching `📇️directory/🟦️.ts` or `📚️library/🔍️discovery/🟦️.ts` no longer restarts Vite.

## Why the config graph is the right unit

Vite loads this config with `--configLoader bundle` (`📚️library/📦️packages/🟦️typescript/🟦️.ts`, `withViteConfigLoader`). Vite's own `externalize-deps` plugin (`node_modules/vite/dist/node/chunks/config.js:35917-35944`) marks **every** bare specifier external, so the config's real bundle graph is exactly its relative-import closure. Every module in that closure is (a) parsed by esbuild on every boot and (b) added to Vite's config-dependency watch set — one peer edit anywhere in it restarts the dev server.

## Before / after

| Metric | Before | After |
|---|---|---|
| modules in the config graph | 131 | 32 |
| source bytes parsed | 4 004 648 | 473 472 |
| esbuild bundle bytes | 3 261 209 | 371 315 |
| `VITE ready in` (served, loaded machine) | 415 982 ms (coordinator, 2026-09-05 baseline) | **10 136 ms** (lane E, 13:46) |
| restart on `📇️directory/🟦️.ts` edit | yes | **no** |
| restart on `📚️library/🔍️discovery/🟦️.ts` edit | yes | **no** |

Per-import graphs measured before the work (esbuild, Vite's externalize rule):

| config import | modules | bundle bytes |
|---|---|---|
| dev `📜️script.ts` | 131 | 3 267 728 |
| `📇️registry/📜️script.ts` | 20 | 721 271 |
| `🖱️ui/🎨️styling/🟦️.ts` | 23 | 102 117 |
| `🔌️plugin/🏪️store/📥️store.ts` | 15 | 141 733 |
| `🧑‍💻dev/🚚️distribution/🟦️.ts` | 4 | 27 665 |
| `🧑‍💻dev/🏷️brand/🟦️.ts` | 2 | 34 668 |
| `📇️registry/📦️deployment/🟦️.ts` | 6 | 11 428 |
| `📇️registry/🤖️generated/🎮️playgrounds.ts` | 1 | 22 085 |

Heaviest inputs before: `📚️library/🔍️discovery/🟦️.ts` 807 231 B, dev `📜️script.ts` 452 597 B, `🎭️actor/📮️shard-client/🟦️.ts` 442 966 B, `📚️library/📦️packages/🟦️typescript/🟦️.ts` 293 242 B, `💻️os/🟦️.ts` 285 140 B, `📇️registry/📜️script.ts` 189 622 B.

Traced chains (esbuild metafile BFS):

```
⚙️vite.config.ts -> 🧑‍💻dev/📜️script.ts   -> 📚️library/🔍️discovery/🟦️.ts
⚙️vite.config.ts -> 🧑‍💻dev/📜️script.ts   -> 🎭️actor/📮️shard-client/🟦️.ts
⚙️vite.config.ts -> 📇️registry/📜️script.ts -> 💻️os/🟦️.ts -> 💻️os/🔨️modules/📇️directory/🟦️.ts
⚙️vite.config.ts -> 🖱️ui/🎨️styling/🟦️.ts   -> 📚️library/📦️packages/🟦️typescript/🟦️.ts
⚙️vite.config.ts -> 🏪️store/📥️store.ts -> 🔌️plugin/📦️packages/🟦️typescript/🟦️.ts -> 📚️library/📦️packages/🟦️typescript/🟦️.ts
```

All five are cut.

## What changed

### 1. Dev-server Vite plugins moved out of the task router

- **new** `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🔌️vite-plugins.ts` (430 lines) — `semioProductionTestBoundaryVitePlugin`, `semioBackboneVitePlugin`, `semioBlobVitePlugin`, `semioPluginHotSwapVitePlugin` plus their private helpers (lazy `bun:sqlite` ctor, per-path handle cache, backbone read/write, folder watch, SSE keepalive, `scanBuiltPluginModules`), moved verbatim from `📜️script.ts:138-377`, `379-460`, `470-554`. Imports only `node:fs`, `node:path`, `node:url`, `@semio-tech/framework-os`, a type-only `@semio-tech/framework`, `📇️registry/📦️deployment/🟦️.ts` and `🔏️hash/🟦️.ts` — 8 modules, no library, no dev script. Own graph: 8 modules.
  - It exports `REPO_ROOT` / `PLUGIN_MODULES_ROOT`, derived from `import.meta.url` instead of `getWorkspaceRoot()` (which is what dragged the library in). Verified equal at runtime: `REPO_ROOT === getWorkspaceRoot()` → `true`, `PLUGIN_MODULES_ROOT` exists.
- `📜️script.ts:41-42` — the `@semio-tech/framework-os` import narrowed to `decodePackValue, encodePackValue`; new `import { PLUGIN_MODULES_ROOT, PLUGIN_SOURCE_WATCH_PATH, backboneDbHandleFor, scanBuiltPluginModules } from "./🔌️vite-plugins.ts"`. `📜️script.ts:69` `const pluginOutRoot = PLUGIN_MODULES_ROOT;` — single source, no duplicated path literal.
- `📜️script.ts:6156` and `📜️script.ts:6305` — the two in-source tests that read the plugin source by AST now read `🔌️vite-plugins.ts` instead of `import.meta.url`; `📜️script.ts:6347` injects `PLUGIN_MODULES_ROOT` (renamed free variable).
- `⚙️vite.config.ts:12` and `♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts:8` import the plugins from the new module.

### 2. `isHostPluginFilter` replaced by a pure generated-data lookup

- `🔌️plugin/📇️registry/🟦️.ts` — new `//#region 🏠️HostPlaygroundFilter` with `isHostPlaygroundFilter(pluginFilter?, playgrounds = PLAYGROUND_BUILD_TARGETS, targets = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS])`. Same resolution order and same host predicate (`host !== undefined`) as `projectedHostPluginFilter` (`📇️registry/📜️script.ts:546`), over the generated TS modules rather than the JSON projection. Two array scans, no filesystem.
- `⚙️vite.config.ts:10, 63, 77` call it; the config no longer imports `📇️registry/📜️script.ts` at all (that import was the only path to `💻️os/🟦️.ts` and `📇️directory/🟦️.ts`).
- Shared test vector: `📖️generated-projection.test.ts` gained *"the generated-module host predicate is semantically identical to the projection one"*, which drives **both** functions off the existing `🧫️fixtures/📖️generated-projection.json` `expectations` rows **and** cross-checks them on the live projection for every variant, alias and plugin id (123 filters incl. `undefined`, `""`, `not-a-plugin`).

### 3. Library edges cut (this is what stopped the `🔍️discovery` restarts)

`📚️library/📦️packages/🟦️typescript/🟦️.ts` imports `🔍️discovery/🟦️.ts` at line 14 and re-exports it, so *any* edge into that barrel pulls the 807 KB taxonomy walk. Two edges existed; both are gone, by splitting two coherent domains out of the barrel (barrel keeps them via `export *`, so no consumer changed):

- **new** `📚️library/🎮️playground/🟦️.ts` (145 lines) — `PlaygroundHostKind`, `loadFrameworkOsPlaygroundCatalog`, the whole `🔌️PlaygroundDevPorts` region (`PLAYGROUND_PORTS` proxy, `playgroundDevPort(String)`, `playgroundTestPort(String)`, `playgroundPortEnv`, `allPlaygroundReservedPorts`, `OS_HUB_PORT(_ENV)`, `PLAYGROUND_LOCKED_EXAMPLE_ENV`, `playgroundLockedExampleIdFromEnv`, `playgroundPlayViteDefine`). Barrel re-exports it in a new `//#region 🎮️Playground`; `🎨️styling/🟦️.ts:30` now imports from it. Styling's own graph: 23 → 10 modules.
- **new** `📚️library/🏃️process/🟦️.ts` (174 lines) — budget classes (`BUILD/CMD/ORCHESTRATOR/DAEMON_BUDGET_MS` + resolvers, `defaultBudgetMs`, `budgetTimeoutHint`), `RunCmdOpts` + presets, `runCmdInternal`/`runCmd`/`runCmdStatus`/`tryRun`, `resolveWorkspaceBin`, `runNodeBinStatus`/`runNodeBin`, and the `SemioBuildMode` block. Barrel re-exports it in `//#region 🏃️Process` and imports back the 20 symbols it still uses itself. `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:17` now imports its five helpers from there instead of the barrel — which is what removed the store's edge.
- `📚️library/🗂️workspaces/🟦️.ts` gained `//#region 🔎️WorkspaceRoot` with `getWorkspaceRoot` (moved out of the barrel, which now imports it back). Needed because both new modules use it and importing it from the barrel would have reintroduced the cycle into `🔍️discovery`.

Barrel is 6710 → 6578 lines minus the process block.

### 4. Tests

- **new fixture** `🧑‍💻dev/📦️packages/🟦️typescript/🧫️fixtures/⚙️config-graph.json` — language-agnostic contract: `entry`, `deny` (8 paths incl. dev `📜️script.ts`, `📇️registry/📜️script.ts`, library `🟦️.ts`, `🔍️discovery/🟦️.ts`, `🏗️builder/🟦️.ts`, `💻️os/🟦️.ts`, `📇️directory/🟦️.ts`, `📮️shard-client/🟦️.ts`), `require` (5 paths), `maxModules: 40`, `maxSourceBytes: 700000`.
- `🧹️config.test.ts` — four new cases in `describe("vite config module graph")`: deny, require, bounds, and an **independent oracle** that spawns `bun build --target=bun --packages=external --sourcemap=external` and asserts Bun's sourcemap `sources` set equals esbuild's TS-module set (23 modules; the other 9 graph inputs are JSON, which Bun does not list). Pinned `@vitest-environment node` — at `long` the suite default is jsdom, whose `TextEncoder` trips esbuild's `instanceof Uint8Array` invariant.
- The file is already in the package's vitest `include`, so `@semio-tech/framework-os-dev:test` covers it; no new Nx target or launch entry was needed.

## Commands and real output

Graph measurement (esbuild with Vite's externalize rule, run from the repo root):

```
$ bun scratchpad/lane-e/graph.ts …/⚙️vite.config.ts          # before (reconstructed via the old entry)
files 131 srcBytes 4004648 bundleBytes 3261209
$ bun scratchpad/lane-e/graph.ts …/📜️script.ts               # today: the old graph, unchanged, as proxy
files 136 srcBytes 4090408 bundleBytes 3301049
$ bun scratchpad/lane-e/graph.ts …/⚙️vite.config.ts          # after
files 32 srcBytes 473472 bundleBytes 371315
```

Bundle checks (`bun build --target bun --packages external`):

```
⚙️vite.config.ts                          Bundled 32 modules in 264ms
🧑‍💻dev/📜️script.ts                        Bundled 136 modules in 307ms
🧑‍💻dev/🔌️vite-plugins.ts                  Bundled 8 modules in 26ms
📇️registry/📜️script.ts                    Bundled 22 modules in 148ms
🏪️store/📥️store.ts                        Bundled 13 modules in 32ms
📚️library/📜️script.ts                     Bundled 18 modules in 42ms
📚️library/📦️packages/🟦️typescript/🟦️.ts   Bundled 17 modules in 102ms
🎨️styling/🟦️.ts                           Bundled 10 modules in 35ms
♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts  Bundled 138 modules in 133ms
```

Runtime probe that every moved symbol still resolves through the barrel (37 named imports):

```
missing: none
workspaceRoot: /Users/ueli/Documents/semio
buildBudgetMs: 1200000 semioBuildMode: dev cargoProfileDir(dev): debug
s dev port: 6070 env: S_OS_PORT reserved: 123
catalog rows: 60 viteDefine: {"import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID":"\"\"","import.meta.vitest":"undefined"}
resolveWorkspaceBin(vite): /Users/ueli/Documents/semio/node_modules/.bin/vite
```

Tests:

```
$ SEMIO_TEST_LEVEL=quick bun …/vitest.mjs run --config 🧪️tests/🟦️.ts     # @semio-tech/framework-os-dev
 Test Files  2 passed (2)
      Tests  66 passed | 28 skipped (94)

$ SEMIO_TEST_LEVEL=long  bun …/vitest.mjs run --config 🧪️tests/🟦️.ts 🧹️config.test.ts
 Test Files  1 passed (1)
      Tests  7 passed (7)

$ SEMIO_TEST_LEVEL=long  bun …/vitest.mjs run -t "routes encoded OS watcher and installation requests through the actual adapter handlers"
      Tests  1 passed | 95 skipped (96)
$ SEMIO_TEST_LEVEL=long  bun …/vitest.mjs run -t "emits runtime URL assets but no dead in-source-test assets in production"
      Tests  1 passed | 102 skipped (103)

$ bun …/vitest.mjs run --config 🧪️tests/🟦️.ts 📖️generated-projection.test.ts   # @semio-tech/plugin-registry
 Test Files  1 passed (1)
      Tests  4 passed (4)
```

Non-vacuity control — adding `🎨️styling/🟦️.ts` (a module that *is* in the graph) to the fixture's `deny` list makes the gate fail, then reverted:

```
× keeps every denied module out of the bundled config graph 1082ms
AssertionError: 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts is reachable from ⚙️vite.config.ts — Vite parses and watches it on every boot
      Tests  2 failed | 5 passed (7)
```

Served boot on port 6075 (`S_OS_PORT=6075 SKIP_PLUGIN_BUILD=1 SKIP_ENGINE_BUILD=1 bun ./📜️script.ts dev s served`, foreground, killed by pid from `lsof -tiTCP:6075`; port 6070 untouched). Each of the three files was appended a blank line and byte-restored from a copy, 4 s apart:

```
 8:1:46:07 PM [vite] (client) Re-optimizing dependencies because vite config has changed
10:  VITE v7.3.6  ready in 10136 ms
12:  ➜  Local:   http://127.0.0.1:6075/
13:[lane-e] --- marker A ---            ← touch 📇️directory/🟦️.ts … then 📚️library/🔍️discovery/🟦️.ts … then 🎨️styling/🟦️.ts
34:1:46:28 PM [vite] ../../../../../../🔨️modules/🖱️ui/🎨️styling/🟦️.ts changed, restarting server...
35:1:46:29 PM [vite] server restarted.
36:1:46:32 PM [vite] ../../../../../../🔨️modules/🖱️ui/🎨️styling/🟦️.ts changed, restarting server...
37:[lane-e] --- marker B ---
50:1:46:39 PM [vite] server restarted.
[lane-e] port 6075 pids: 44715
```

Between marker A and marker B, `📇️directory/🟦️.ts` and `📚️library/🔍️discovery/🟦️.ts` produced **zero** restart lines; `🎨️styling/🟦️.ts` (still legitimately in the graph, it supplies the config's Vite plugins) produced two, which is the positive control proving the watcher was live. Port 6075 was free after the run.

## Remaining blockers / notes

1. **`🎨️styling/🟦️.ts` is still a restart trigger.** It is a genuine config dependency (10 Vite-plugin factories the config mounts), now only 10 modules deep. Removing it would mean splitting the styling package's Vite-plugin surface from its dev-server surface — a separate lane; the win left is small (86 KB of the remaining 473 KB).
2. **An unrelated peer breakage was live during the boot**: `✘ [ERROR] No matching export in "…📦️packages/🟦️typescript/🟦️.ts" for import "parseDirectorySpaceAdministrationPageV1"` from `📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:149`, which failed Vite's dependency scan (`Failed to run dependency scan. Skipping dependency pre-bundling.`). That is the COMPLETE-SEMIO fleet's directory-module edit, not this lane; the shell still served.
3. **`@semio-tech/plugin-registry` has two pre-existing red tests** unrelated to this lane: `🚀️launch.test.ts` *"exposes every owned generator preview exactly once in contract order"* (received has an extra `dev-distribution-bundle` — taxonomy `generatorContracts` vs `.vscode/launch.json` drift; `.vscode/launch.json` was already dirty at ticket start and is regenerated by every `dev s` boot) and *"keeps generated native, root preflight, and MCP runtime profiles identical without debug"*; plus `✅️catalog-complete.test.ts` timing out at its 5 s budget under load ~300. None touch the modules changed here.
4. **`🚚️distribution/🔗️inputs.json` was deliberately left alone.** It is a hand-curated static authority (12 paths) that already omits several config imports (`📥️store.ts`, `📦️deployment/🟦️.ts`, `🎮️playgrounds.ts`), and adding `🔌️vite-plugins.ts` would require regenerating `📤️distribution/🧾️manifest.json` via a full production `vite build`. Recommended follow-up when a distribution regeneration is being run anyway: add `🔌️vite-plugins.ts` to that list and to `taxonomy.json`'s `dev-distribution-bundle.inputPatterns`.
5. **Pre-existing type noise in the moved code**: `bun:sqlite` `TS2307` and one `TS2352` in `🔌️vite-plugins.ts` are verbatim from `📜️script.ts` (bun types are not in the ad-hoc `tsc` invocation used here). `tsc` on the library barrel reports exactly three errors, all pre-existing and unrelated (`ExtensionPackageManifestRecord.directoryName`, a `Bun` global, and an implicit `any` in a Playwright `page.evaluate`).
6. **`♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts` is still 138 modules** — it imports its own `./📜️script.ts`, which pulls the library. Same treatment would apply; out of this lane's scope (`s` shell only).

## Files touched

Created:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🔌️vite-plugins.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧫️fixtures/⚙️config-graph.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts`

Updated:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧹️config.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️generated-projection.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts`
