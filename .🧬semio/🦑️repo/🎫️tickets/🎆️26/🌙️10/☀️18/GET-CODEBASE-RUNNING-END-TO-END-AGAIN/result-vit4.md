# result-vit4: Playwright browsers + all vitest configs (2026-10-20, executor vit4)

Foreground, no sub-agents, no git writes, no `bun test` beyond one fixture test, no cargo. Runner `C/vit4-tools/runvt.py` (vit3 runner + `--configLoader native`, `SEMIO_TICKET_DIR=C`, `PLAYWRIGHT_BROWSERS_PATH`, optional `VT_SHIM=1`). Single config: `C/vit4-tools/one.sh <config> [cap]`. From 13:05 the coordinator's TOTAL WRITE PAUSE holds: remaining repo edits are STAGED in `C/vit4-tools/staged-apply.py` (dry run by default, `--write` applies; anchored, idempotent).

## 1. Playwright browsers
- `bun repo run workspace:deps-browsers` FAILED to start: `Cannot find module '@semio-tech/ui/axes/plan'` from `📚️library/🏗️build/🚀️bootstrap/🎚️ui-axes/🟦️.ts` (the CLI closure imports the new, still unlinked `@semio-tech/ui` package; `bun repo --help` works, `repo run` does not). Peer batch in flight; the bare import inside the CLI closure breaks the "stay RELATIVE" rule of the brief.
- Fallback (what `deps-browsers` itself runs, `native/🟦️.ts`: `bun node_modules/playwright/cli.js install chromium` with `repoToolCacheEnv`): `PLAYWRIGHT_BROWSERS_PATH=<repo>/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright bun node_modules/playwright/cli.js install chromium ffmpeg` (playwright 1.64.1, locked). rc 0. Installed: `chromium-1248`, `chromium_headless_shell-1248`, `ffmpeg-1013` next to the existing 1234/1011. Cache path is the repo cache root, not `~/Library/Caches/ms-playwright`.
- Effect: `framework 2d` (Chromium mode) is now GREEN (8 files / 153 tests).

## 2. Result: 46 of 59 configs green (vit3: 42 of 60)
Config list changed on disk: `gismap inference renderer`, `layout renderer-contract`, `puzzle renderer-contract` no longer exist (consolidated into `✏️s/🧪️tests/🏘️plugin-composition/🎚️renderer-contract`); new `framework 📐️geometry` config. List now `C/vit4-tools/list2.txt` (59). Final pass `f1`: every config run once, one process at a time.

IMPORTANT harness caveat: the repo root `node_modules` (generation 7082...) was synced before 14 workspace packages were added to the root `package.json` (`framework-hash`, `framework-transient`, `framework-filesystem`, `framework-manifest`, `framework-effect`, `framework-trace`, `framework-interaction`, `framework-onboarding`, `framework-action-argument-resolution`, `ui`, `ui-contract`, `plugin-registry-publication`, `repo-product`, `repo-server-library`; created 11:04, manifest-batch2). Without links, 25+ configs fail at import. The run therefore used `VT_SHIM=1` (vit4-tools `shimcore.mjs`/`shimhook.mjs`/`shimreg.mjs`: a Node `module.register` resolve fallback plus a Vite `resolveId` plugin that read the packages' own `exports` from the workspace dirs; no repo or node_modules write). The shim cannot reach bun/esbuild child processes. Without the shim (first pass `r1`, 37 configs run) most configs were red with `Cannot find package '@semio-tech/framework-hash|transient|...'`. The pending dependency sync removes the need for the shim; rerun without `VT_SHIM` afterwards.

| config | vit3 | now | fix / cause |
|---|---|---|---|
| mit-bestand presentation (33.projektetage) | green | GREEN 2/35 | unchanged |
| mit-bestand demonstrator | green | GREEN 5/31 | needs links (shim) |
| fem | green | GREEN 15/64 | needs links (shim) |
| cad extensions x4 (structure, building, spatial-shape, energy) | green | GREEN | needs links (shim) |
| remodel | green | GREEN 4/1373 | needs links (shim) |
| trinity lsp | green | GREEN 2/14 | needs links (shim) |
| puzzle 5d-react, puzzle | green | GREEN | needs links (shim) |
| plugin-composition renderer-contract (replaces gismap/layout/puzzle renderer-contract) | n/a | GREEN 5/31 | config consolidated by a peer |
| framework geometry (new) | n/a | GREEN 1/26 | new config |
| hub admin / stdio publication / hub testing | green | GREEN | hub testing needs links |
| hub tests | RED (JSON attributes) | GREEN 12/70 | JSON attributes already attributed by peers; native config loader |
| play | RED (http-cache-semantics) | GREEN 7/75 | see below: published worker static import rewrite |
| framework 2d | RED (Chromium 1248) | GREEN 8/153 | Chromium 1248 installed |
| actor | RED (wire-turn lanes) | GREEN 12/107 | see below: lane fixture |
| registry | RED (JSON attrs) | GREEN 7/90 | see below: catalog-view TLA |
| framework course react | RED (1 stable) | GREEN 41/1036 | native config loader + links |
| root `tests`, async, r3f, kernel, kernel label fixture, replication, machine, canvas react-renderer, 3d, schema, presentation x2, pets x2, store, window-kits, shell, os root, server, repo client mcp/sqlite, coordinator, framework root, course (framework product) | green | GREEN | unchanged (kernel label, r3f, replication need links) |
| teaching architecture course | RED (tar-stream, ancestry, ticket) | RED 1 of 13 failed (12 pass) | fixed: image ancestry (`realpathSync(tmpdir())`), proctor ticket env (`SEMIO_TICKET_DIR` in runner), config load (native loader). Left: `resolves authored public source wrapper .../generated/ui-axes`: workspace-exports fixture row `sourceExports[0]` has `source` == `target` (a generated projection, no longer a re-export wrapper after the ui-axes pipeline change). Peer-owned (ui-axes) |
| ui-styling | green | RED 1 of 3 | `resolveAssetDeliveryModeV1` esbuild subprocess: `Could not resolve @semio-tech/framework-transient` (unlinked, class A) |
| ui-react | RED (bun:ffi, Chromium) | RED 2 of 46 | fixed `bun:ffi` Vite resolution (below); next in chain `storage/fixed` top-level `fileURLToPath(new URL(.., import.meta.url))` fails in the jsdom client environment (`The URL must be of scheme file`) -> STAGED lazy path. `react-package-export` layered renderer law: `chrome/🟦️.ts` now re-exports `@semio-tech/ui/elements/LayeredOverview` (11:04 batch), the test's `@semio-tech/` external rule hides it (peer-active ui split) |
| catalog (dev) | RED | RED 2 of 4 | `Cannot find module @semio-tech/framework-filesystem/observation` in generator child (class A) |
| cad dev | RED | RED 3 of 13 | esbuild `Could not resolve` x37 (class A); `geometry.current-browser-binding-required` (class D) |
| gis services | RED | RED 1 file | `semio-os-mcp` binary gate (class D) |
| gis inference bridge | RED | RED 1 file | stale path: composition moved under `🧩️component` -> STAGED path fix; then binary gate (class D) |
| gis service-composition | RED | RED 1 of 3 | wasm guest `reactor/poll: guest fault` (class D) |
| os mcp | RED | RED 4 of 8 | `semio-os-mcp` binary absent (class D) |
| registry composition | RED | RED 1 of 8 | `@semio-tech/framework-os-node-graph-rs/index` is a wasm-bindgen package (class D) |
| os dev | RED | RED 2 of 19 | bun children cannot resolve unlinked packages (class A); module-graph budget laws `vite config module graph` 260 > 40 modules and browser-entry graph (peer import re-pointing, peer-active) |
| engine react | RED | RED 23 of 221 (cap 420 s hit once under load, 230 s alone) | stubbed wasm packages -> `X is not a function` (D), bun children unlinked (A), `fixtures/🧯️wgpu-transient-notice` path after test split -> STAGED, Rust-text laws (peer) |
| wgpu | RED | RED 11 of 56 | Rust source text laws over files peers are splitting (peer-active) |

## 3. Fixes applied (before the pause)
1. `framework-plugin` registry `📖️catalog-view/🟦️.ts`: `filterProjectedPluginRegistry` loaded `🔎️discovery` through `createRequire(...)('../discovery')`. Node 24 `require(esm)` refuses a graph with top-level await (`ERR_REQUIRE_ASYNC_MODULE`), Bun accepted it. Now a static `closeProjectedPluginV1` / `resolveProjectedPluginIdsV1` in catalog-view (uses `@semio-tech/repo-lib/inventory/tooling` `runtimeComponentClosure`); `🔎️discovery/🟦️.ts` `resolveRegistryPluginIdsForFilter` delegates to it (single implementation). registry config 6/7 -> 7/7 files, 90 tests.
2. `🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts` `vitestArgumentsV1`: adds `--configLoader native` after `--config`. Vite's bundle config loader inlines relative `.cjs` library files and turns `require("node:fs")` into `Dynamic require of "node:fs" is not supported` (teaching, course react); native loading is the same module graph under Node and Bun (parameter properties were removed by vit3, so Node strip-only works). Fixture-driven law updated: `🧪️tests/🧫️fixtures/🔣️.json` expected argv, `🧪️tests/🟦️.ts` slice 10 -> 12. Verified: `SEMIO_TEST_ARTIFACT_DIR=<short> bun test ./🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🧪️tests/🟦️.ts` 1 pass, 17 expects.
3. `🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts` and `🏃️process/🔒️leases/📁️physical/🟦️.ts`: `await import("bun:ffi")` -> `process.getBuiltinModule("bun:ffi")` (typed cast; Vite `vite:import-analysis` failed on the literal in client environments; Bun returns the module, Node returns undefined and the jobserver throws a clear error). `bun build --target=bun` on both files: ok.
4. Play site publisher `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/🟦️.ts`: REAL DEFECT. The generated shard worker now has a static `import { monotonicClock } from "../vendor/.../clocks.js"` (`worker-bootstrap-and-faults`), but the publisher rewrote only `import("...")` calls to the modules origin, so the CDN worker would have requested the vendor file from the page origin. New `absolutizeRelativeImports` rewrites dynamic, `from` and side-effect relative imports. Fixture `🧪️playpages/🧫️fixtures/🔣️.json` `generatedImports` gains the static row and a per-row `publishedUrl`; test compares per-row published URLs against the independent es-module-lexer parse. play config: 6 -> 7 files green (75 tests).
5. Actor typed-operation-page law: TS and Rust (`Diagnostic => 15`) have 16 lanes; fixture and its schema said 15 (`diagnostic` lane missing, `invalidTags` [15,255]). Fixture + schema now 16 lanes, invalid [16,255]. actor config green (107 tests).
6. Teaching `🧪️tests/🐳️image/🟦️.ts`: `realpathSync(tmpdir())` (macOS `/var` symlink violates the product's "no link in the ancestry" rule, which stays).
7. Runner/harness only (C/vit4-tools): `SEMIO_TICKET_DIR`, `PLAYWRIGHT_BROWSERS_PATH`, native loader, shim, function-config wrapper.

## 4. STAGED (apply after "PAUSE LIFTED": `python3 C/vit4-tools/staged-apply.py` then `--write`)
- `💾️storage/🧱️fixed/🟦️.ts`: `directoryScript` const -> lazy function (3 edits in 2 anchors). NOT verified: the ui-react scope-resolution chain may fail further behind it.
- `gis-inference-bridge/🟦️.ts`: `hostManifest` -> `🌎️hub/🧩️compositions/🌍️gis/🧩️component/📦️packages/🦀️rust/Cargo.toml` (the `gis2d` row with `mcpHost` lives there now). Then it reaches the binary gate (class D).
- engine react `🔸️carries-one-mirror-activation/🟦️.tsx`: `../🧫️fixtures` -> `../../🧫️fixtures`.

## 5. Remaining blockers by class
- A. Workspace links for the 14 new packages (dependency sync pending). Affects ui-styling, catalog, cad dev, os dev, engine react bun children, plus every shim-dependent green. Also the `repo run` CLI closure now imports `@semio-tech/ui/axes/*` bare (`🚀️bootstrap/🎚️ui-axes/🟦️.ts`): violates the relative-import rule; `bun repo run ...` is broken until the link exists.
- B. JSON import attributes: no unattributed JSON import remains in the library outside tests (grep of the library: only `@semio-tech/...` package-specifier JSON imports in 5 non-test files, not used on a Node path in these configs). `bun:ffi`: fixed above.
- C. Chromium 1248: solved.
- D. Native/wasm builds: `semio-os-mcp`, `semio-s-gis-services-mcp`, wasm-bindgen packages, wasm guest, cad browser binding.
- E. Peer-active: wgpu Rust-text laws, os dev module-graph budgets, ui-axes fixture row, ui `LayeredOverview` package split.

## 6. Verification
- `python3 C/vit4-tools/runvt.py list2.txt f1.jsonl ...` x3 calls (single process): 59 configs, 46 rc 0. Per-fix re-runs with `one.sh`: registry, play, actor, teaching (each as reported). The decisive count (46/59) is the shimmed final pass; fixes 1 to 6 landed before it except the teaching image/ticket fixes (verified separately: teaching 3 failed -> 1 failed).
- Not run: `bun test` of anything else, cargo, Python oracles, runs without `VT_SHIM`, repo `bun repo run workspace:deps-browsers` (failed as above), any staged edit.
- Generated output under `C/🗑️generated/vit4-out` and `C/🗑️generated/l/vit4a` deleted at the end; no `[DEBUG]` lines added.
