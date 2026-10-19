# result-vit: vitest configs 28 red, after the pass

Executor `vit`, foreground, no cargo, no `package.json`/`bun.lock` edit, no edit under the frozen `💾️storage` / `🔏️revision` paths. Runner: `C/vit-tools/runvt.py` (tst2 runner with per-config logs hashed instead of ASCII-sanitized, because emoji-only path tails collided and overwrote each other's logs). `node vitest.mjs run --config <cfg>` from the package cwd, `SEMIO_VITEST_POLICY` as `runVitestV1` builds it (the canonical policy runtime is `node`, so Bun-only code under test is a real failure, not a harness artifact). Harness env: `SEMIO_BUILD_MODE=prototype` (the launcher supplies it). Optional harness modes, never committed: `TST2_HOIST=1` (alias wrapper onto a rebuilt hoist of the current store generation `4151d226…`) and `PLAYWRIGHT_BROWSERS_PATH=<scratch dir symlinking chromium-1234 as 1248>`.

Scope note: the list of "51 configs" contains one false positive, `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts`. It is a test suite included by the sibling `🎚️config` (a vite-config graph law), not a vitest config, so the 28 red are 27 real configs.

## Result

Plain (as the package launcher runs it), 27 configs: before 3 green (animate, quiz-react, pets-react) -> after 7 green (+ jack lsp, root, styling, plugin-registry). With the missing packages and the Playwright browser revision supplied by the harness: 11 green (+ 2d, react-renderer, presentation-react, os r3f). Everything else is blocked by items outside this task (listed under "Record for deps / native"), by peer-active churn, or by real drift listed per config.

| # | config (short) | before (plain) | after (plain) | class | notes |
|---|---|---|---|---|---|
| 1 | `✏️s/🔌️plugins/🎞️animate` | red, `@tailwindcss/vite` missing | green 1/1 | deps landed meanwhile | not my change |
| 2 | `…/📐️cad/🧩️extensions/🏛️aec-building-structure` | red no tests, `brepjs` | red, `brepjs` | deps | `brepjs`, `brepjs-opencascade` |
| 3 | `…/🧩️extensions/📐️spatial-shape` | red, `brepjs` | red, `brepjs` | deps | same |
| 4 | `…/🧩️extensions/🔥️aec-building-energy` | red, `brepjs` | red, `brepjs` | deps | same |
| 5 | `🔱️trinity/…/🔌️jack/🧠️lsp` | red `self is not defined` | green 14 tests | config + test | config used the worker module itself as the suite. Protocol guards moved to `🔖️protocol/🟦️.ts`, new suites `🧪️tests/🔖️protocol` (JSON vector) and `🧪️tests/📡️worker` (mocked wasm session, stubbed `self`) |
| 6 | `🧑‍💻dev/🎭️variants/🌍️gis/💡️services` | red startup error `paths[0]` | red, `semio-os-mcp` binary gate | config fixed, then native | `import.meta.dir` (Bun-only) replaced by `import.meta.dirname` in the config and in `🧪️tests/📁️artifact-root`. Remaining: needs the native binary |
| 7 | `🧑‍💻dev/📐️cad` | red 8 files | red 8 files | deps + native | `brepjs`, `@dnd-kit/sortable`, `geometry.current-browser-binding-required` (browser binding regeneration) |
| 8 | `🧑‍💻dev/🧩️catalog` | red 4 | red 3 | real + generated | fixed: admission `maxRows` 128 -> 512 (playground rows are 156). Remaining: catalog source audit takes 78 s against a 30 s census budget and reports 13 `descriptor-pair` issues (descriptors need regeneration), session tests need the launcher's native caller capabilities |
| 9 | `🌎️hub/🔨️modules/🛡️admin` | red `@vitejs/plugin-react` | red 1 file | deps | `@dnd-kit/sortable`; the package's `node_modules` link points into an older store generation (`react-dom` from `02ad8f…` against `react` from `4151d2…`, hence `useState` of null) |
| 10 | `🏢️semio-tech/🎡️play` | red 7 | red 5 files / 9 tests | real drift | `http-cache-semantics` (deps). Fixed: `PLAY_PANE_SHELL_PROPS` gains `initialLocale` (`🧪️playpanecoverage`), Play's own catalog admission `maxRows` 128 -> 512 (a real runtime defect with 156 rows). Open: plugin descriptors are no longer committed under `🌎️hub/🧩️compositions/<plugin>/🔣️.json` (law reads them), `✏️s/🔌️plugins/🧪️tests` is not a plugin directory, `📋️project.json` has no `activate-prototype`/`prepare-prototype` targets, release graph now includes `@semio-tech/assets:build`, launch seed env gained `BUN_RUNTIME_TRANSPILER_CACHE_PATH`/`NODE_DISABLE_COMPILE_CACHE` (peer launch-seed rewrite in flight) |
| 11 | `🧪️tests` (root) | red no test files | green (pass with no tests) | config | by design the root owns no Vitest suite; `passWithNoTests: true` |
| 12 | `🧰️framework/…/◻️2d` | red 1 | red 1 | environment | Playwright wants chromium rev 1248, installed 1234. Green with the harness symlink |
| 13 | `…/🖱️ui/🎨️styling` | red 1 | green 82 tests | real (test) | `tileProxyVitePlugin` moved to `🏗️builder/🌐️vite/🛰️tile-proxy`; the spawned program imported it from the barrel |
| 14 | `…/🖱️ui/🎯️targets/⚛️react` | red 14 files | red 13 files / 1 test | deps + real | fixed: `Diagram` harness exported a `vi.hoisted` const (vitest 4 refuses), wgpu dock parity test read the pre-split Rust file. Remaining plain: `@dnd-kit/sortable`, `color`, `tailwindcss`. With hoist: 2 files (`color`, `tailwindcss`) |
| 15 | `…/🖼️canvas/🎨️react-renderer` | red `@react-three/fiber` | red `@dnd-kit/sortable` | deps | green with hoist |
| 16 | `…/❓️quiz/🎯️targets/⚛️react` | red `@vitejs/plugin-react` | green 929 | deps landed meanwhile | |
| 17 | `…/❓️quiz` | red 344 passed + 2 files | same | deps | `jstat` |
| 18 | `…/🎤️presentation/🎯️targets/⚛️react` | red | red 14 files | deps | `@dnd-kit/sortable`; green with hoist (140 tests) |
| 19 | `…/🐾️pets/🎯️targets/⚛️react` | red | green 175 | deps landed meanwhile | |
| 20 | `…/💻️os/…/♾️infinite/🌍️world/🎨️r3f` | red | red | deps | `@dnd-kit/sortable`; green with hoist (103 tests) |
| 21 | `…/💻️os/…/🌉️mcp` | red 4 | red 4 | native | `semio-os-mcp` binary missing (cargo build) |
| 22 | `…/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react` | red 158 files | red 158 files | deps + env + peer churn | plain: Playwright, `tailwindcss`, `@dnd-kit`. With hoist+browser: 13 files, e.g. frame-latency/package-integration (monolith split), Chromium WebGPU pixel tests, a 282 ms vs 250 ms timing law under load. Not touched: the wgpu split is being edited by peers (browser-frame-transport went from 10 to 6 failing between two of my runs) |
| 23 | `…/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu` | red 17 | red 19 | peer churn + deps | remaining with hoist: `tree-row-rects`, `wgpu-raster-residency`, `package-integration` (2), `shell-component-decomposition` (`tree-sitter-wasms`, deps). Fixed here: `⏱️retained-clock-deadlines` read the pre-split winit-app file (now `🎞️frame-generation-and-os-host`) |
| 24 | `…/🔌️plugin/📇️registry` | red, 97 files `No test suite` | green 90 tests | config + code | `exclude` replaced the default, so `node_modules/…/repo-coordinator/dist/prototype/**` mirrors were collected: now `...configDefaults.exclude, "dist/**"`. Then node-native `require()` of discovery failed on JSON without attribute and on TS parameter properties (strip-only mode): see changed files |
| 25 | `…/🔌️plugin/📇️registry/🧪️tests/🧩️composition` | red 5 | red 2 files | deps + generated + env | `@dnd-kit/sortable`, `framework-os-node-graph-rs` wasm package not built, Playwright rev 1248. Fixed: `retained-clock-deadlines` path |
| 26 | `…/💻️os/🔨️modules/🧑‍💻dev` | red 9 | red 5 files / 28 tests | real + runtime mismatch | Playwright (3), Bun-only code under test run on node (`Bun.Transpiler` in `🚚️distribution/📥️source`, `bun:sqlite`), stale expectations (`semio_hub_gis_component.wasm` vs `semio_hub_gis.wasm`, `preview2-shim` URL rebasing, vite-config reachable graph 622 modules vs 40 bound), jco/wast component fixtures. Not fixed |
| 27 | `…/💻️os` (root os config) | red 749 passed + 2 files | same | deps | `retry` (not in any store generation), `cockatiel` |

## Changed files (all under existing owners, none under the freeze)

- Config: `…/🔌️plugin/📇️registry/🧪️tests/🎚️config/🟦️.ts` (default excludes), `🧪️tests/🎚️config/🟦️.ts` (root, `passWithNoTests`), `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🎚️config/🟦️.ts` and `…/🧪️tests/📁️artifact-root/🟦️.ts` (`import.meta.dirname`), `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/🧪️tests/🎚️config/🟦️.ts`.
- Jack lsp: new `🔖️protocol/🟦️.ts`, `🧪️tests/🔖️protocol/{🟦️.ts,🔣️.json}`, `🧪️tests/📡️worker/🟦️.ts`; `🟦️.ts` and `📦️packages/🟦️typescript/🟦️.ts` re-pointed.
- Node strip-only / JSON attribute portability (the registry's lazy `createRequire` of discovery runs on native Node): `…/🔌️plugin/📇️registry/🧬️schema/🟦️.ts`, `🧰️framework/🔨️modules/🏃️process/📥️capture/🟦️.ts` (`CaptureInterrupted`), `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🟨️ecma/🟦️.ts` (`EcmaParser`), `…/🔍️discovery/🛣️ecma-command-routes/🟦️.ts` (`EcmaRouteScope`), 51 `with { type: "json" }` import sites listed in `C/vit-tools/json-attribute-files.txt` (the repo's existing convention; mtime-derived list, a few may belong to peers).
- Tests/fixtures: `…/🖱️ui/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts`, `…/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🧰️harness/🟦️.ts`, `…/🖱️ui/📱️device/🧪️tests/🔬️unit/🟦️.ts`, `…/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/⏱️retained-clock-deadlines/🟦️.ts`, `🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanecoverage/🟦️.ts`.
- Admission limits: `…/🧑‍💻dev/🎮️playground-session/🧬️schema/🔣️.json` (`maxRows` 512), `🏢️semio-tech/🎡️play/🟦️.tsx`, `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧪️multi-shell-harness/🟦️.tsx`.

## Verification

Per config: `python3 C/vit-tools/runvt.py <list> <out.jsonl> <budget> <cap>` (lists `vit-tools/list-27.txt`); native probe of the registry's require chain `node -e 'require("./…/📇️registry/🔎️discovery/🟦️.ts")'` -> exit 0 (loop `vit-tools/loop.py`, helpers `fixjson.py`, `fixpp.py`). Final plain run of all 27: table above (7 green, 20 red). Registry 90/90, styling 82/82, jack lsp 14/14, root exit 0.

## Record for deps / native (freeze applies)

- Packages: `brepjs` (store has 18.164.3 only in old generations) and `brepjs-opencascade` (in no generation), `@dnd-kit/sortable` + `@dnd-kit/core`, `color`, `tailwindcss`, `jstat`, `cockatiel`, `retry` (in no generation), `tree-sitter-wasms`, `http-cache-semantics` (in no generation). Several workspace packages' `node_modules` links point into older store generations (hub admin: `react-dom` from `02ad8f…`).
- Browser: Playwright expects chromium revision 1248, `~/Library/Caches/ms-playwright` has 1234. `bunx playwright install chromium` is a download and needs your permission; with a 1248->1234 symlink the 2d, registry composition and dev-os browser tests ran.
- Native/generated: `semio-os-mcp` binary (mcp, gis services), `@semio-tech/framework-os-node-graph-rs` wasm package, browser geometry session binding (dev cad), regenerated component descriptors (catalog audit: 13 pair issues).

## Open

20 configs red, see table. Not attempted because peers are mid-edit: wgpu/renderer source-reading laws. Not attempted because large: play descriptor-law rewrite (descriptors no longer committed), os dev (28 tests), catalog audit performance. The `[DEBUG]` convention: no debug logs were added to the repo.
