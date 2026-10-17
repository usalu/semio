# Result: Suite Load Fixes (slice SUITE-LOAD-FIXES, prefix `slf`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. 2026-10-16 22:30 to 23:05. Tools: `slf-tools/` (this folder `F`). Logs and raw outputs: `🗑️generated/slf-*` (nothing deleted). No `[DEBUG]` log was added. Git calls ran with `-c core.fsmonitor=false` because the fsmonitor daemon kept dying ("daemon terminated"); that is read-only.

STATUS: COMPLETE. Every row below was run.

## 0. Verdict

- **Load matrix** (same 24 vitest routes, `slf-tools/suites.tsv`; base = tree at 22:30, after = tree at 23:00): files that load **402 to 419 of 434** (base had 435 files; hub-admin +2 files found, ui-react -1 stub removed, mcp -2 non-suite files excluded). Tests collected 8,504 to 8,701, passing 8,327 to 8,520.
- **20 of 24 routes load completely** (before 17). Still with load failures: **cad 9** (waits on the user's `bun install` decision, untouched as ordered), **hub-tests 1**, **ui-react 1**, **mcp 4**. Those three are not cad-like in cause but each needs something outside a re-point (section 4): uncatalogued hub schema modules owned by another slice, an undeclared/uninstalled dependency `color`, and a build product (`semio-os-mcp` binary) that its tests deliberately refuse to skip.
- **Failure sets of suites that already loaded are identical or smaller**: 0 files stopped loading. Per-suite deltas are 0 except quiz-react 2 to 1 (flaky test that passed this time), renderer-react +1 and registry-composition +2, all three inside files that did not load before or timing flakes (section 3.2). The ownership-gate, Storybook and cad items are in sections 2 and 4.
- **Storybook** (`.storybook/main.ts`) failed at HEAD for **three** stale imports, not one (`pluginModulesRoot` was only the first). A dry config load (module import plus `viteFinal` for DEVELOPMENT and PRODUCTION, scopes `""`, `framework/os`, `ui`, `infinite`, `puzzle/3d`) now succeeds (section 1).
- **Vitest configuration ownership gate** reads its fixture again: the gate runs (4 pass, 3 fail on content; it was dead with ENOENT) (section 2).

## 1. Storybook config (`.storybook/main.ts`)

Dry load: `slf-tools/sb-load.mjs` (bun) imports `main.ts`, reads `stories`, `staticDirs`, `addons`, then calls `viteFinal({plugins:[],resolve:{}}, {configType})` for both modes with an issued cache directory under the products partition (`SEMIO_VITE_CACHE_DIRECTORY`) and `SEMIO_BUILD_MODE=prototype`. No server was started. Output: `🗑️generated/slf-sb-load-final.log`, `slf-sb-load-scopes.log`.

Faults found one after the other by running the load (each one hid the next):

| # | Stale reference | Replaced by | Why |
|---|---|---|---|
| 1 | `pluginModulesRoot` from `🧑‍💻dev/♻️activation/🟦️.ts` | `builtPluginModulesRootIn(repoRootPath, readBuildMode(process.env.SEMIO_BUILD_MODE))` for both dev and build | The leased "active projection" function was removed (commit 690); the activation module keeps only the mode-qualified root. The launcher issues `SEMIO_BUILD_MODE=prototype` for `storybook dev` (`serveRepositoryStorybookV1`) and the real mode for build, so one call serves both. Same call the play builder uses. |
| 2 | `playgroundAssetVitePlugins(root, scopeAssets)` | `createAssetBuildPluginsV1(root, scopeAssets, PLAYGROUND_ASSET_PROVIDERS_V1)` | Removed from the styling vite builder in commit 667 (providers are now explicit: `PLAYGROUND_ASSET_PROVIDERS_V1`). `StoryScope.assets` was already typed `AssetDeliveryDeclarationV1[]`; its docstring still named the removed function and now names the new one. |
| 3 | `playgroundFlowWasmDevStubPlugin(root)` | `…(root, flowBrowserDeliveryV1(root, repositoryCachePartitionRootDataV1(root,"products"), {version:1, mode, command}))` | The plugin now requires an issued delivery (as in the play and demonstrator builders). |
| 4 | `viteCacheDirectory(repoCacheDirectory(root,"vite","storybook"), env, import.meta.url)` | `viteCacheDirectory(repoRootPath, {...process.env, SEMIO_BUILD_MODE: buildMode})` | The function now takes the workspace and validates the launcher-issued `SEMIO_VITE_CACHE_DIRECTORY`; the old call would have thrown at `viteFinal`. `repoCacheDirectory` import dropped. |

`buildMode` (`prototype` for DEVELOPMENT, else `readBuildMode(env)`) is now one local in `viteFinal`, also used for `canonicalWasmBuildVitePlugin` and `semioViteBuild`. Results: stories 14 (empty scope), DEVELOPMENT 15 alias entries / 17 plugins, PRODUCTION 15 / 18; `framework/os` 13 aliases and 17 plugins (its `assets` go through `createAssetBuildPluginsV1`); `ui` 11/6, `infinite` 10/6, `puzzle/3d` 9/6.

Type check (`tsc -p slf-tools/tsconfig.storybook.json`, root options, only `main.ts` as root file): 8 errors in `.storybook/main.ts`, all of two kinds that predate this slice: `OwnedBuildPlugin` not assignable to Vite `PluginOption` at the `config.plugins.push/unshift` lines (the repo's owned build-plugin type is narrower than Vite's; same pattern on lines I did not touch), and two spread-iterator typings on `assetsInclude`/`allowedHosts`. Not fixed (out of scope; they do not affect the runtime load). Log `🗑️generated/slf-tsc-storybook.log`.

## 2. Vitest configuration ownership gate

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts` read `libraryRoot/🧫️fixtures/…` (absent); now `libraryRoot/🧪️tests/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json`. The same stale directory was spelled in the gate's Nx input list (`📦️packages/🟦️typescript/📋️project.json`: `…/📚️library/🧫️fixtures/{🧰️framework-root-source-topology,🖥️os-source-topology}/🔣️.json`), now `…/🧪️tests/🧫️fixtures/{…}` (both fixtures exist there).

Run: `cd <library>/📦️packages/🟦️typescript && bun test ../../🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts` (log `slf-ownership-gate.log`): **4 pass, 3 fail** in 63 s (before: dead at the fixture read). The 3 failures are content, not path: (a) "loads every explicit owner and preserves its behavioral projection" (hash drift of owners; `ass` already measured 16 of 20 alias-touched owner hashes stale, and this slice changed two more owner configs, hub-admin and registry-composition, see section 5), (b) "requires every runVitest call to select an anonymous semantic configuration" (the list of unresolved config references such as `✏️s/🧑‍💻dev/🪐️space/…/📜️script.ts: ../../🧪️tests/🎚️config/🟦️.ts -> unresolved`), (c) "registers the gate through package and Nx" (expects the package script `nx run …`, the package has `bun --cwd … repo run …`). These need an owner decision; not touched.

## 3. Vitest routes

### 3.1 Per-route causes and fixes

| Route | Before | Cause | Fix | After |
|---|---|---|---|---|
| hub-admin | 0 of 1 | `🧪️tests/🎚️config` resolved `../🧹️environment`, `../🛡️admin`, `../🧪️command-routing` against the package directory (`dir`), not the config directory; only the in-source I18n file was found and the setup file did not exist | New `configDir`; the three `resolve(dir, "../…")` calls use `configDir` | 3 of 3 files (the two component suites were never collected before), 22 of 22 tests pass |
| hub-tests | 8 of 12 | (a) `creation-progress` read the fixture from `📇️directory/🧬️schema/…` (moved to `📇️directory/🧪️tests/🧫️fixtures/🌱️space-artifact-creation-v1`); (b) `document-growth` and `two-client-document` crashed at import with `paths[0] must be string`: `🔌️plugin/🌐️browser-bundle/📜️script.ts` used Bun's `import.meta.dir`, which is undefined under vitest (node) and was evaluated at module top level; then both located `🧫️fixtures` by `readdir(hubRoot).find(includes("fixtures"))`, which no longer exists at the hub root (it is `🧪️tests/🧫️fixtures`, and `🌍️gis-map-fixtures` also matches the substring) | (a) path re-point; (b) one `browserBundleDirectory = dirname(fileURLToPath(import.meta.url))` replaces all 13 `import.meta.dir` uses in that script (same value under bun, valid under node); both tests now use `join(hubRoot, "🧪️tests", "🧫️fixtures")` (three call sites) | 11 of 12 (see 4.1 for the last one) |
| ui-react | 43 of 45 | (a) `🖱️ui/🧪️tests/📐️overlay-flow/🟦️.ts` is a 110-byte stub holding one docstring: commit 671 ("Fixtures Are Testing Examples Only") deleted the fixture-owned schema `🖱️ui/🧬️schema/📐️overlay-flow` that this test validated against and emptied the test, but the explicit `include` of the ui-react config still lists it; (b) ShellScope component test needs `color` | (a) the include line removed from `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` and the empty stub deleted (copy in `🗑️generated/slf-overlay-flow-stub.ts.pre`); the fixture is still covered by the renderer `🗣️Interpreter/🧪️tests/📐️overlay-flow` test and two Rust tests through the public entry `loadUiOverlayFlowFixture`; (b) see 4.2 | 43 of 44 files, 1,166 of 1,166 tests pass |
| mcp | 3 of 10 | (a) the include glob `🧪️tests/*/🟦️.ts` also took `🎚️config/🟦️.ts` and `🧫️fixtures/🟦️.ts` (non-suites: "No test suite found"); (b) `✅️approval-request` imported `bun:test` and `import.meta.dir` | (a) both added to `exclude`; (b) `vitest` imports plus `dirname(fileURLToPath(import.meta.url))` | 4 of 8 (see 4.3) |
| renderer-react | 206 of 215 | Nine suites read fixtures from `engine/🧫️fixtures` or `<element>/🧫️fixtures`, which moved to `engine/🧪️tests/🧫️fixtures` and `<element>/🧪️tests/🧫️fixtures`; `time-travel` also had three doubled join segments left by an old codemod (`join(…, "🧪️tests/🧫️fixtures/🧫️history-reprojection/🔣️.json", "🧫️history-reprojection", "🔣️.json")` gives ENOTDIR); `diff-view-produced-surface` loaded the deleted schema `🖱️ui/🧬️schema/🆚️diff-view-presentation` | Path re-points in `♿️wgpu-accessibility-interaction` (3 reads), `🎮️wgpu-browser-input-wire`, `🪟️app-mode-layouts`, `🌳️GraphTimelineHost/🎨️layout`, `📌️ChromePanels` (keybinding capture), `📡️EventFeedHost/🎨️layout`, `🤖️AgentApprovals` (2 reads), `⏪️time-travel` (3 joins). `diff-view-produced-surface`: the schema read, the `Ajv2020` import and the two-line schema validation of the presentation corpus were removed (the schema was deleted on purpose by the fixtures policy, no domain contract replaced it; the independent diff oracle assertion of the same test stays). No public fixture entry exists for the engine, so the paths are direct. | 215 of 215 files |
| registry-composition | 7 of 8 | `📖️generated-projection` requires `SEMIO_TEST_ARTIFACT_DIR`, which only the launcher issued | The config sets it the way the mcp config does: `repoTestArtifactEnvironment(repoRoot, "registry-composition")` into `test.env` | 8 of 8 |

### 3.2 Matrix (base to after)

From `🗑️generated/slf-vcmp-final.txt` (summaries `slf-sum-{base,after}.json`, raw reports `slf-vt-{base,after}-<suite>.json`). Method as in `RESULT-ALIAS-SUBPATH-SWEEP.md` §3: `slf-tools/vt.sh` runs `node vitest.mjs run --config <owner config> --reporter=json` from the owner package directory with `SEMIO_TEST_LEVEL=long` and an explicit `SEMIO_VITEST_POLICY`; a file counts as not loaded when its result is `failed` with zero test rows.

| Suite | Files | Loaded b to a | Tests b to a | Pass b to a | Fail b to a |
|---|---:|---|---|---|---|
| projektetage | 2 | 2 to 2 | 35 | 35 | 0 |
| cad | 13 | 4 to 4 | 12 | 10 | 2 |
| hub-admin | 3 (1) | 0 to **3** | 0 to 22 | 0 to 22 | 0 |
| hub-tests | 12 | 8 to **11** | 67 to 72 | 56 to 59 | 9 to 9 |
| async | 2 | 2 | 27 | 27 | 0 |
| actor | 10 | 10 | 104 | 104 | 0 |
| replication | 2 | 2 | 21 | 21 | 0 |
| ui-styling | 3 | 3 | 82 | 80 | 2 |
| ui-react | 44 (45) | 43 to 43 | 1,166 | 1,166 | 0 |
| canvas-react-renderer | 1 | 1 | 10 | 10 | 0 |
| quiz-react | 31 | 31 | 926 | 924 to 925 | 2 to 1 |
| presentation-react | 7 | 7 | 140 | 140 | 0 |
| pets | 26 | 26 | 1,133 | 1,133 | 0 |
| os | 23 | 23 | 609 | 499 | 110 |
| mcp | 8 (10) | 3 to **4** | 28 to 29 | 27 to 28 | 1 to 1 |
| framework | 6 | 6 | 306 | 306 | 0 |
| infinite-world-r3f | 1 | 1 | 103 | 103 | 0 |
| renderer-react | 215 | 206 to **215** | 3,043 to 3,206 | 3,006 to 3,168 | 37 to 38 |
| registry-composition | 8 | 7 to **8** | 159 to 165 | 148 to 152 | 11 to 13 |
| server | 1 | 1 | 18 | 18 | 0 |
| shell | 2 | 2 | 7 | 6 | 1 |
| quiz | 11 | 11 | 451 | 451 | 0 |
| presentation | 2 | 2 | 56 | 56 | 0 |
| animate | 1 | 1 | 1 | 1 | 0 |
| **Total** | **434 (435)** | **402 to 419** | **8,504 to 8,701** | **8,327 to 8,520** | **175 to 177** |

(Brackets: base file count where it differs.) `vcmp.mjs`: 0 files stopped loading. Failing tests that did not fail before (every one explained):

- renderer-react +1: `⏪️time-travel/🧩️component` (62 tests, 61 pass; "the shell folds every polled step of a load into the loading program's projection …"), a file that did not load before; a behavioral failure, not a path.
- registry-composition +2: `📖️generated-projection` (newly loaded; its test imports the library discovery module `🔍️discovery/🟦️.ts`, which node's strip-only TypeScript refuses: "TypeScript parameter property is not supported in strip-only mode" at line 7865, the known Vite/Node strip-only trap) and `⏱️wgpu-worker-step-budget` "chunks the plugin graph …" (an 8 ms timing law that measured 8.78 ms while the machine ran 3 routes at load 35; the same file run alone afterwards fails exactly the 3 baseline tests, `slf-tools/vf.sh registry-composition wgpu-worker-step-budget`). `vcmp` lists the second as the one "regression"; it is jitter, not a change of mine.
- quiz-react 2 to 1: one test passed that failed at base (peer drift or flake); smaller, not larger.

## 4. Remaining load failures

### 4.1 hub-tests `📌️document-check-in` (1 file)
`hubSchemaExport` (in `🌎️hub/🤝️integration-harness`) refuses to run unless **every** hub `🧬️schema/🔣️.json` is in `📚️library/🔣️schema-catalog.json`. I regenerated the catalog (`bun ./📜️script.ts schema generate` from the repo root, 3,560 scopes, same set of scopes, only module digests changed, 1,966/1,747 line churn in the tracked generated file; `--check` said stale before), but seven hub modules are not catalogued because they are not valid scopes (inventory diagnostics, `slf-tools/inv.mjs`): `🌎️hub/🏗️bootstrap/🧾️provenance/📦️cargo/🧫️fixture/🧬️schema` (declares no `$id` and is flagged `schema-fixture-defines-schema`: a fixture owning a schema, against the fixtures policy; used by `cargo/🧫️fixture/🟦️.ts` and its tests), `🧩️compositions/🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧬️schema`, `🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📥️execution/🧬️schema`, `🚀️local-bootstrap/🎛️host/🧬️schema`, `🧪️tests/💾️backup-restore/🧬️schema`, `🧪️tests/🎟️carried-phase/🧬️schema`, `🧪️tests/🎛️run/🧬️schema`. Fixing this means moving each contract to a real domain scope or giving it a valid `$id`; that belongs to the owners (bootstrap-provenance, stdio composition, local-bootstrap, the three test folders). I did not weaken the harness gate.

### 4.2 ui-react `🐚️ShellScope/🧪️tests/🧩️component` (1 file)
Imports the third-party WCAG oracle `color` (non-literal dynamic import). `color` is installed nowhere (`node_modules/color` absent; no `"color"` in `bun.lock` or any `package.json`); `ui-styling`'s `🧩️suite` has the same import (inside a test, so it is one of the 2 failing ui-styling tests, not a load failure). Needs a user decision with `bun install`, same class as cad. Recommendation: declare the already pinned and oracle-registered `colord@2.10.0` (with its `a11y` plugin, `contrast()`/`luminance()`, `🔮️oracles` entry `quiz-react-colord-contrast`) as a ui-react devDependency and swap both oracles, or declare `color`. Not done here because both options touch `package.json`/`bun.lock` and the oracle registry.

### 4.3 mcp `🌅️modern-era`, `🏛️legacy-conformance`, `🔄️end-to-end`, `🧹️hygiene` (4 files)
They call `requireMcpBinary(repoRoot)` at module load ("so a missing black-box subject cannot skip green") and the staged `semio-os-mcp` binary (`📦️packages/🦀️rust/dist/build/semio-os-mcp`) does not exist; it is a cargo build product, and compile admission is single-slot and headroom-gated. Deliberate gate, left intact. The loadable 4 files include the `🔐️authenticated-hub-workspace` suite (1 failing test as at base).

### 4.4 cad (9 files), not touched
`@semio-tech/cad-js-module-spatial-shape` has no `node_modules/@semio-tech` link (6 files plus one Vite "Failed to resolve import") and `brepjs-opencascade/src/brepjs_single.wasm?url` is not installed (2 files). Waits on the user's `bun install` decision, identical to the `ass` record.

## 5. Peer-owned files and one-line re-points

I made **no edit under `🖱️ui/🧬️contract/**`**. Within `🖱️ui`: the ui-react test config (`🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, one include line removed) and the empty stub `🖱️ui/🧪️tests/📐️overlay-flow/🟦️.ts` (deleted). The renderer fixture reads that used `🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts` entries (`viewProducedSurfaceFixtureUrl`, `diffViewPresentationFixtureUrl`, `inlineTreeControlsFixtureUrl`, `inputOrderedScrollFixtureUrl`, `loadUiOverlayFlowFixture`) were already correct and unchanged.

Behavior-hash note: the gate of section 2 hashes owner configs. Changed owner configs this slice: hub-admin (include and setup paths), ui-react (include list), mcp (exclude list), registry-composition (`test.env`). Re-run the hash tool (`ass-tools/ownership-hash.mjs`) when the gate's content failures are taken on.

## 6. Foreign findings (not fixed)

1. Seven uncatalogued hub schema modules (4.1), one of them a fixture that defines a schema.
2. `color` undeclared and uninstalled (4.2), used by ShellScope and `ui-styling` tests.
3. `semio-os-mcp` not staged (4.3).
4. `🔍️discovery/🟦️.ts` (library) uses TypeScript parameter properties; every vitest route that imports it under node fails (`registry-composition` generated-projection).
5. The ownership gate's three content failures (section 2); `time-travel` 1 failing test; the `main.ts` type gaps (section 1).
6. The inner e2e blocks of `📈️document-growth` and `🤝️two-client-document` (skipped without `HUB_E2E=1`) still locate the directory schema, local-bootstrap and trace fixtures with `readdir(...).find(includes(...))` heuristics; I re-pointed only the hub fixture lookup that the loadable part needed.
7. `git` (fsmonitor) dies intermittently in this tree ("error: daemon terminated"); `-c core.fsmonitor=false` avoids it.

## 7. Commands run

| # | Command | Result |
|---|---|---|
| 1 | `xargs -P3 slf-tools/vt.sh base <suite>` over 24 routes, `vsum.mjs base`, `causes.mjs base` | `slf-sum-base.json`: 402 of 435 files load |
| 2 | `bun slf-tools/sb-load.mjs` (empty scope, `framework/os`, `ui`, `infinite`, `puzzle/3d`; DEVELOPMENT and PRODUCTION) before and after the fixes | before: `SyntaxError: Export named 'playgroundAssetVitePlugins' not found` (after the first fix); after: loads |
| 3 | `tsc -p slf-tools/tsconfig.storybook.json` | 8 errors in `main.ts`, pre-existing kinds |
| 4 | `slf-tools/one.sh <phase> <suite>` for hub-admin, mcp, renderer-react, registry-composition; `slf-tools/vf.sh hub-tests <filter>` (growth, two-client, creation-progress); `vf.sh registry-composition wgpu-worker-step-budget` | section 3 |
| 5 | `bun ./📜️script.ts schema generate --check`, then `schema generate`, `bun slf-tools/inv.mjs "cargo/🧫️fixture"` | stale; regenerated, 3,560 scopes; diagnostics 4.1 |
| 6 | `bun test …/🎚️vitest-configuration-ownership/🟦️.ts` | 4 pass, 3 fail |
| 7 | `xargs -P3 slf-tools/vt.sh after <suite>`, `vsum.mjs after`, `vcmp.mjs`, `causes.mjs after` | 419 of 434 files load; `slf-vcmp-final.txt` |

## 8. Files changed

- Storybook: `.storybook/main.ts` (4 stale references, `buildMode` local), `.storybook/📖️stories/🧭️coordination/🟦️.ts` (one docstring word).
- hub: `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🎚️config/🟦️.ts`; `🌎️hub/🧪️tests/{🌱️creation-progress,📈️document-growth,🤝️two-client-document}/🟦️.ts`.
- plugin: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts` (`import.meta.dir` to a node-valid directory const); `🔌️plugin/📇️registry/🧪️tests/🧩️composition/🎚️config/🟦️.ts`.
- ui-react: `🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`; deleted `🖱️ui/🧪️tests/📐️overlay-flow/🟦️.ts`.
- mcp: `🌉️mcp/🧪️tests/🎚️config/🟦️.ts`, `🌉️mcp/🧪️tests/✅️approval-request/🟦️.ts`.
- renderer (`📺️renderer/🧑‍🎨engine`): `🧪️tests/{♿️wgpu-accessibility-interaction/🟦️.tsx, 🎮️wgpu-browser-input-wire/🟦️.ts, 🪟️app-mode-layouts/🟦️.ts, 🆚️diff-view-produced-surface/🟦️.tsx}`; `🧱️elements/{🌳️GraphTimelineHost/🧪️tests/🎨️layout, 📌️ChromePanels/🧪️tests/🧩️component, 📡️EventFeedHost/🧪️tests/🎨️layout, 🤖️AgentApprovals/🧪️tests/🧩️component, 🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component}/🟦️.ts(x)`.
- library (parent-held, minimal): `📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts` (fixture path), `📚️library/📦️packages/🟦️typescript/📋️project.json` (one input glob), `📚️library/🔣️schema-catalog.json` (regenerated by its own generator; pre-image in `🗑️generated/slf-schema-catalog.pre.json`).
- Created in `F`: `slf-tools/` (`env.sh`, `suites.tsv`, `vt.sh`, `one.sh`, `vf.sh`, `vsum.mjs`, `vcmp.mjs`, `causes.mjs`, `sb-load.mjs`, `inv.mjs`, `tsconfig.storybook.json`) and this file. Pre-edit copies: `🗑️generated/slf-main.ts.pre`, `slf-overlay-flow-stub.ts.pre`.
- Not touched: `AGENTS.md`, `.vscode`, any ticket tool, any git state, cad, `🖱️ui/🧬️contract/**`.
