# Play Release Verification Path (2026-10-10)

Read-only audit of how `@semio-tech/semio-tech-play` release output is verified after `build-fresh`. No source file, build, server or test was run. Evidence comes from file reads and the existing logs and status files under `.🧬semio/🦑️repo/⚡️cache/play-fleet/coordinator/`.

## 1. Targets and what they run

| Nx target (project.json) | Runs | Serves | Asserts | Tests | Browser |
|---|---|---|---|---|---|
| `test` | `bun ./📜️script.ts test` → vitest (`🧪️tests/🎚️config/🟦️.ts`, `environment: node`, includeSource of 7 modules incl. `📄pages/🟦️.ts` and `🆕️fresh-build/🟦️.ts`) | Nothing | Unit and source-embedded tests: pane catalog, descriptors, runtime, publication, fresh-build environment | 69 run at the 2026-10-07 prefinal check (61 pass, 8 fail, all from a stale generated catalog) | No |
| `test-release-routing` | `bun ./🔨️modules/🧪️e2e/📜️script.ts test-release-routing` → `bun test 🧪️tests/🧪️playrelease/🟦️.ts` | Four `Bun.serve` listeners on `127.0.0.1:0` over `dist/pages/{play,map,media,modules}` (only inside the test) | `releaseAssetPath` (9 path rows, no SPA fallback), `releaseRequestUrl` (3 origin rows), real HTTP status, MIME, body, `Cache-Control`, `Access-Control-Allow-Origin` compared against Playwright's API client; missing file must not return HTML | Fixture-driven: 9 + 3 + 1 registrations in the file. The 2026-10-07 log reports 14 pass; I did not reconcile the 14 | No |
| `test-release` | `bun ./🔨️modules/🧪️e2e/📜️script.ts test-release` (forwards extra args to Playwright) | Four independent static listeners (`servePublishedPlay`), one per page, `dist/pages`; no Vite | The same acceptance spec as `test-e2e` (see below), run against the emitted pages; CDN origins are rerouted by `installPublishedPlayRoutes` | **149** (1 overview + 148 pane boots) | Yes, chromium project |
| `test-e2e` | `bun ./🔨️modules/🧪️e2e/📜️script.ts test` → Playwright with the same config | `serve-e2e` → `prepare-e2e` → `activate-dev` (DEV profile, Vite dev server on `127.0.0.1:0`, frozen) | Same acceptance spec | 149 | Yes |
| `build` | `📦️site/📜️script.ts build` | Nothing | Vite build of `dist/site` (SEMIO_BUILD_MODE=ship, SEMIO_RENDERER=react, GIS_MAP_TILE_SERVE_MODE=bundle), then `publishPlayPages` | Its own throws only | No |
| `build-fresh` | `📦️site/📜️script.ts build-fresh` → new `play-fleet/release-*` generation, then Nx `run @semio-tech/semio-tech-play:build --skip-nx-cache --skip-remote-cache --parallel=4 --output-style=stream` | Nothing | Whole inner graph must exit 0 (`build` depends on `prepare-release` → `catalog-release` → 28 release composition preparations) | Its own contract test: `test` on `🆕️fresh-build/🟦️.ts` (5 tests per the file) | No |
| `catalog-release` / `prepare-release` | Canonical catalog refresh with `renderCatalogFiles(..., "refuse")` then `GenerateScript`; then runtime `prepare release` | Nothing | Each release component marker and runtime closure | None beyond their own throws | No |

Notes on the acceptance spec (`🧪️tests/🎭️acceptance/🟦️.ts`), which both `test-release` and `test-e2e` run:

- Overview test: lists one card per pane (148), `N apps` count, no page errors, no console errors, no HTTP ≥400, no SPA fallback for asset paths.
- Per pane (`test.setTimeout` 240 s):
  - `page.goto("./#<variant>")`, wait up to 120 s for `data-shell-ready` on that shell only; `networkidle` then a 2 s settle.
  - Zero pageerror, console.error, HTTP ≥400, `requestfailed`, refused input (`input #N … refused:`) and SPA-fallback asset.
  - The example label in the chrome must equal the catalog `exampleLabel` (121 panes have one; 26 stdio/PDF/Office/binary panes have none and skip the example step).
  - Paint witness (not pixels): 3D windows must publish at least one mesh or instance, raster windows must have a visible layer whose asset is at least 8 px per side, DOM windows must have text or at least 10 elements in `pane-host-root`. Polled for up to 4 s.
  - Dismiss first-use introduction (`ui.introduction.skip`).
  - Example picker: clear to the blank option, paint witness not required there, then restore the curated example and require paint.
  - Role round trip: viewer, then editor, each through `playground.navbar.roles.*`, each waiting for `aria-busy` to clear, shell ready, no error boundary, `data-app-id` dialect check, dismissal, and paint witness. This is interaction, not only boot.
  - Overview button returns to the overview.
- Contract fixture: `🧫️fixtures/🔀️roles/🔣️.json` (Ajv-validated).
- This is editor-role coverage: viewer and editor for every pane, with painted content. It does not perform document edits.

`test-release` runs the spec unfiltered only if no `-g` or path argument is forwarded. Any filter reduces coverage.

## 2. Browsers, server and prerequisites

- Playwright config: `🔨️modules/🧪️e2e/🎚️config/🟦️.ts`. `testMatch` is `🎭️acceptance/🟦️.ts`, `workers: 1`, `fullyParallel: false`, `retries: 0` (2 when `CI` is set), reporter `list`, `trace: on-first-retry`, one project `chromium` with Desktop Chrome.
- Launch args: `--use-angle=metal` on darwin, `--use-angle=gl` otherwise, both with `--enable-gpu --ignore-gpu-blocklist --enable-unsafe-webgpu`. `PLAY_E2E_GPU=swiftshader` switches to software ANGLE.
- Browser location: the e2e script sets `PLAYWRIGHT_BROWSERS_PATH = repoCacheDirectory(repoRoot, "tools", "ms-playwright")`, which is `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`. The same path is in the native dependency fixture and the ticket launch entries.
- `workspace:deps-browsers` (root project.json): `bun 🧰️framework/…/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts sync browsers`, `cache: false`. It is a dependency of `test-release`, of `prepare-e2e` (and so of `test-e2e` and `serve-e2e`), and of the framework's own targets. I did not run it or inspect what it downloads.
- `test-release` and `test-e2e` do not depend on `build-fresh`. `test-release` checks only that `CNAME` exists in all four directories and `index.html` exists in `play`. It will therefore run against whatever is in `dist/pages`, including a stale one. Freshness must come from running `build-fresh` first.
- Browser prerequisites are verified by `browser-capabilities` (ticket `📜️script.ts`): it launches chromium with the acceptance launch args and requires `WebAssembly.Suspending`, `promising` (JSPI) and a WebGPU adapter. Recorded on this host: HeadlessChrome 151.0.7922.34, jspi true, gpu true, adapter true (2026-10-07).
- Timeouts: `playwrightTestTimeoutMs()` returns the active test level budget, which defaults to `fundamental` = **15 s** unless `SEMIO_TEST_LEVEL` is set. The e2e launch entries and project.json do not set it. Boot tests override this with 240 s, but the overview test does not. Its `expect` timeout is 15 s too.

## 3. What `dist/pages` contains and how it is deployed

Produced by `publishPlayPages` (`🔨️modules/📦️site/📄pages/🟦️.ts`) from `dist/site`, which is removed afterwards. Each page is a directory with `CNAME`, `.nojekyll` and `_headers`. Each page must be under 1 000 000 000 bytes.

| Page dir | CNAME host | Contents | `_headers` |
|---|---|---|---|
| `play` | `play.semio-tech.com` | App HTML, bundled JS and CSS, same-origin copy of `shard-worker.js` | `/*` `Cache-Control: no-cache` |
| `map` | `map.assets.semio-tech.com` | `osm`, `vt`, `dem` (map tiles) | no-cache plus `Access-Control-Allow-Origin: *` |
| `media` | `media.assets.semio-tech.com` | `mesh`, `cad-assets`, `infinite-assets`, `SEMIO_ASSET_DIRECTORY` | same |
| `modules` | `modules.assets.semio-tech.com` | `🔌️plugin-modules`, `🧩️extension-modules`, support shims, fonts, `shard-worker.js` | same |

- Host names come from `playPageHost(name, PLAY_HOST)`. `PLAY_HOST` is `catalog.host` from `🔨️modules/🧩️runtime/🔣️.json`, which is `play.semio-tech.com` (re-exported by `🔨️modules/🧩️runtime/🟦️.ts`, line 8).
- Absolute CDN origins are baked into the app JS through `SEMIO_PLAY_PAGE_ORIGINS` (vite `define`) and into CSS URLs. The shard worker prelude relocates fetches to the satellites.
- Deployment: there is no upload step in the repository. `📓️release-deployment-contract-audit.md` (2026-10-07) inventoried provider configs (`netlify.toml`, `wrangler.*`, `vercel.json`, `firebase.json`, `hosting.json`) and `.github/workflows`, and found none for Play. The only workflow is the unrelated architecture quiz. All four directories must be published together to their hosts. Real HTTPS, MIME, CORS and cache behaviour can be verified only after upload at the provider.
- No launch.json entry publishes or uploads Play pages. The `📇️catalog🏢️semio-tech🎡️play📦️release` and build entries only produce them.
- Map tiles: `build` first runs `prefetchPlayMapTiles` (`🔨️modules/📦️site/🗺️map-tiles/🟦️.ts`), which downloads raster and vector tiles for the default viewport up to zoom 10 into `.🧬semio/🗺️map/osm-tiles` and `.🧬semio/🗺️map/openfreemap-vt`. It uses `skipExisting` and fails on any failed tile. A cold map cache therefore needs network. These directories are outside the fresh-build cache clear list.

## 4. Ordered command list after a fresh build

The AGENTS.md rules say developers run tasks from `.vscode/launch.json` and never the CLI, and the coordinator owns builds. Use the launch.json names below. Equivalent `bun nx run` commands follow each name.

1. Fresh build (must exit 0, produces `dist/pages`):
   - `📦️build🏢️semio-tech🎡️play🆕️fresh` → `bun nx run @semio-tech/semio-tech-play:build-fresh`
   - Covers: build only. Boot coverage: none.
2. Canonical unit gate on the final catalog (non-browser, must be re-run after step 1; the catalog must not be stale):
   - `⚖️gate🏢️semio-tech🎡️playprefinal-complete-unit` → `bun nx run @semio-tech/semio-tech-play:test --excludeTaskDependencies --skip-nx-cache --skip-remote-cache --output-style=stream`
   - Covers: pane catalog, descriptors, runtime, publication code. Boot coverage: none.
3. Feature closure (Cargo resolution per owner, no compile):
   - `🔎️diagnostic🏢️semio-tech🎡️play🧩️features` → `bun nx exec ... -- bun …/feature-audit/📜️script.ts`
   - Covers: 43 owners, 148 panes. Boot coverage: none.
4. Browser prerequisites:
   - `🔎️diagnostic🏢️semio-tech🎡️play🌐️browser` → `…/📜️script.ts browser-capabilities <workspace>`
   - Covers: JSPI and WebGPU adapter only. Boot coverage: none.
5. Release routing contracts (no browser):
   - `⚖️gate🏢️semio-tech🎡️play🛣️routing` → `bun nx run @semio-tech/semio-tech-play:test-release-routing`
   - Covers: static path and origin rules, HTTP semantics against the emitted pages.
6. Worker routing probe:
   - `🔎️diagnostic🏢️semio-tech🎡️play🛠️worker-routing` → `…/🔭️routing/📜️script.ts probe`
   - Covers: worker fetch routing. Boot coverage: none.
7. Publication completeness audit (static; writes `📓️published-page-completeness.md` to the ticket root):
   - `🔎️diagnostic🏢️semio-tech🎡️play📦️publication` → `…/🔍️publication/📜️script.ts audit`
   - Covers: CNAME, `.nojekyll`, CORS, byte budget, worker equality, missing module references. Boot coverage: none.
8. Release browser acceptance (the only full coverage gate):
   - `⚖️gate🏢️semio-tech🎡️play📦️release` → `bun nx run @semio-tech/semio-tech-play:test-release`
   - Covers: 149 Chromium cases against the emitted pages with no dev server. Every pane (148) is booted, painted, has its curated example cleared and restored (121 panes), and is switched editor → viewer → editor with paint witnesses at each step. Boot-only: none. Document edits: not covered.

Optional, not release proof:
- `⚖️gate🏢️semio-tech🎡️play🎭️e2e` → `test-e2e`. Same spec against the DEV profile through `activate-dev`. It is not a release check.
- `🔎️diagnostic🏢️semio-tech🎡️play🎭️acceptance` runs the ticket's `acceptance-oracle/📜️script.ts`. It checks the helper functions (collector, settlement, paint witness) on synthetic pages. It does not run the 149 pane tests. Its name is misleading.
- `🆕️fresh-contract` (`⚖️gate🏢️semio-tech🎡️play🆕️fresh-contract`) checks the fresh launcher's environment and argument contract. Not a build.
- `📦️build🏢️semio-tech🎡️play📦️release` runs a plain `build` without fresh isolation. Do not use it as the redeployability proof.

Boot-only versus full coverage: no Nx gate is boot-only. The overview test and the unit, routing and audit gates do not boot a pane. Every per-pane boot in the acceptance spec also runs the paint, example and role round trip.

Expected runtime: no full run has been recorded. The known numbers are component compile times from the notes: Flow optimized 4m12s, native descriptor release 7m37s, Surface wasm-release 3m52s, AEC component-dev 28m45s under shared queues. The acceptance run is serial, 149 tests. Boot tests allow 240 s each; in practice this is unmeasured.

## 5. Previous runs and known failures (newest first)

Ticket state today (2026-10-10 09:34):
- No `dist/` directory exists in `🏢️semio-tech/🎡️play` (cleared at session start).
- Coordinator fresh attempt 1: `fresh-1.status` `start 09:29:45`, `exit 1 09:30:20`. The log ends with `Script invocation must be an object` and an `OwnedCommandFailure` (process:owner-command failed (1)). It failed in about 2 s, before any compile.
- Coordinator fresh attempt 2: `fresh-2.status` `start 09:32:50`, `exit 1 09:33:33`. Log: `Error: Undeclared Nx child selection refused`, from the Nx bootstrap authority. It failed in about 7 s.
- Both failures happen in the launcher handoff, not in compilation. The likely cause is the direct `bun nx run` from `🔁️run-fresh-2026-10-10.sh`, rather than the launch.json environment. I did not verify that cause.

Earlier evidence (2026-10-07 and 2026-10-08):

- `📓️2026-10-07-acceptance-discovery.md`: `test --list` passes, 149 tests in one file. This is discovery only.
- `📓️2026-10-07-role-settlement-and-network-oracle.md`: red then green on the settlement and paint helpers. Fixed a real ordering gap (paint was checked before the role group settled). Found and fixed a DOM paint defect: a measures overlay in an empty window body counted as content (`painted=true` with 8 characters and 4 elements). Synthetic probes were green, but no real pane run was done.
- `📓️2026-10-07-editor-interaction-readiness.md`: an earlier unit run had 47 pass and 15 fail. Remaining failures were 8 stale-registry gates (catalog has draw and puzzle only) and 7 CDN routing gates under development. HTTP 4xx/5xx and console errors now fail acceptance; the generic 404 suppression was removed.
- `📓️play-prefinal-source-integrity.md` (last edited 2026-10-08 01:02): complete Play unit 69 run, 61 pass, 8 fail. Release routing 14 pass, 0 fail. Feature closure 43 owners, exit 0. Digest oracle exit 0. The 8 Play failures need a complete generated plugin catalog (`cad` missing: `Unknown runtime component cad`). The required step is a catalog refresh and rerun after descriptors are materialized.
- `📓️2026-10-07-redeployment-readiness.md`: "Production browser acceptance: Pending; 149 Chromium tests discovered". The earlier `dist/pages` was incomplete and "cannot substantiate current release readiness". Two cold builds were cancelled to protect shared caches (exit 130 after a Cad lock failure, then `release-6Z7Lxu`). No successful fresh release is recorded.
- `📓️release-assets-second-audit.md` (2026-10-07 02:18, read-only): defect in `relocatePublishedRequestUrl` (`🧰️…/📇️registry/📦️deployment/🟦️.ts`) and the copied shard worker prelude. It compared a decoded emoji path to the percent-encoded `URL.pathname`, so an absolute same-origin emoji URL was not relocated to the satellite. The readiness note later records encoded emoji fetches returning 200 through the adapter, so this is likely fixed. I did not find a closing note for the next item.
- Same audit, second item (open as far as I can tell): `retainSameOriginShardWorker` copies only `shard-worker.js`. The diagnostic `armGuestRuntimeDiagnostics` dynamically imports `…/preview2-shim/cli.js` relative to that copy, which lives only on the modules satellite. Fetch interception does not cover dynamic imports, so the import can 404 and the catch silently disables guest diagnostics.
- `📓️published-page-completeness.md` (2026-10-07 00:31, earlier partial tree): 11 failures. Missing or wrong CNAME and `.nojekyll`, missing satellite CORS, missing same-origin shard worker, no play `index.html` or JS, and `Unknown runtime component cad`. This was an incomplete tree, not a final audit.
- `📓️release-deployment-contract-audit.md` (2026-10-07): no Play upload step exists. Test-release is independent of build-fresh and accepts filtered runs. The readiness order is: build-fresh, unit, routing and browser probe, publication audit, unfiltered test-release, then an authorized upload.
- `📓️fresh-build-pipeline.md` and `📓️final-release-current-generation-plan.md` (2026-10-07 and 2026-10-08): the final cold graph is 321 tasks. Earlier attempts failed on the Cad Cargo lock under `--locked`, on Cad artifact compile errors, and on a Demonstrator component retry (exit 1). Flow and AEC component producers did complete. No successful whole-graph exit is recorded.

Related report that is not a release test: `📓️2026-10-07-all-pane-interaction-contracts.md` (148 panes, 43 owners, 121 pinned examples, 26 without). Its statement that "the browser run has not passed" still stands.

## 6. Auditor findings and risks

1. `test-release` does not require a fresh build. It passes on any `dist/pages` with a `CNAME` and `play/index.html`. Run step 1 before step 8.
2. The overview test in the acceptance spec has no `setTimeout` override, so it uses the 15 s default budget (fundamental level). Set `SEMIO_TEST_LEVEL` for the release gate, or give the overview test an explicit timeout.
3. A filtered `test-release` run (`-g`, a path, `forwardAllArgs`) can pass without covering all 149 cases. Only the unfiltered run is release proof.
4. `dist/pages` is deleted and rewritten by `publishPlayPages`. The audit and the browser gate read whatever is there, with no record of which generation produced it.
5. The two coordinator fresh attempts failed in the launcher, not the build. The build has not yet run with the current source. No release evidence exists for the current source.
6. The diagnostic dynamic-import gap in the shard worker (section 5) is not covered by any test.
7. Map tile prefetch needs network access on a cold map cache.
