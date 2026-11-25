FINAL

# 🐧️ Linux Lane s1-site: Staffage Website End to End on g1-m04

State written 2026-11-25 11:40. Host: Linux x86-64, 12 cores, 15.8 GB RAM, 4.2 GB free on `/home`, swap full for most of the session. Scripts: `🧰️linux/s1/`. Logs: `🗑️generated/linux-s1/` (prefix `s1-`; the four final smoke folders `final-*` with screenshots and downloaded files are kept, the probe folders are deleted). Only runs of this session are counted. `R` = `/home/deck/semio`, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`.

## Where it stands

- **The site runs end to end on this machine from a release build.** Release build exit 0 in 728 s, 375 asset modules, 0 catalog findings, in `S/🌐️site/📦️packages/🟦️typescript/dist/release`. Served on loopback, smoked in Chromium 156 in four contexts: 62 of 62 checks passed, no console error, page error, failed request or response from 400 up (section 3).
- It did not run before: the release build was broken, the desktop overview drove the browser past 4 GB within a minute, an idle overview kept mounting pages, and the release policy blocked the style of the air flow layer. Four repairs with tests, all in the site and the React target of staffage (section 4, A to D).
- **Open, not solved:** the idle desktop overview still holds 2 to 3 GB in the renderer (all of Chromium up to 3.7 GB). The page count is bounded now, the memory level is not low; one of two 150 to 200 s idle runs was still ended by the memory guard at its 4096 MB cap (section 4, C).
- **`repo run` for staffage** starts no staffage task; the time goes into reading the project graph of the whole repository. Cause found as far as measured, not repaired, a run without the 5 minute limit was not made (section 1).
- Not done: no typecheck of the site or the React target; the full React suite was not run (only the cases named below); `catalog-loading`'s whole-archive case timed out at its own 300 s limit; nothing deployed or sent anywhere.

## 1. `repo run` for staffage

| # | Command (from `R`) | Limit | Exit | What happened |
| --- | --- | --- | --- | --- |
| 1 | `repo run @teaching/staffage-site:build --dry-run` | 120 s | 124 | 55 s waiting for the repo graph service, 39 s in the capacity queue for the graph slice (6339 MB asked, 3034 MB granted), then "Reading the project graph slice", killed in the file inventory |
| 2 | `repo run @teaching/staffage-site:build --verbose` | 300 s | 124 | cache collectors first (4.2 GiB free against 12 GiB wanted), 7 s queue (6038 MB granted), then the graph slice for 288 s: file inventory up to 126 652 source observations (144 s), "reading the owners (round 1)" 126 of 3600, "taken 20 of 64" when the limit hit |
| 3 | `repo doctor` | 170 s | 1 | "Graph code changed after generation 389: files of the graph implementation are 895 329 s newer than the published generation (923 523 s old)"; disk 0 % free; swap 8.7 GB |
| 4 | `repo test-file --dry-run S/🌐️site/🧪️tests/📄️host-document/🟦️.ts` | 100 s | 0 | 0.3 s, route `plain plain` |
| 5 | `SEMIO_TEST_LEVEL=long repo test-file S/🌐️site/🧪️tests/🧵️flow-worker/🟦️.ts` | 300 s | 1 | 2.1 s, runs `bun test` instead of Vitest: `factory.runIf is not a function` |

Cause, as far as measured:

- `repo run` does not hang in staffage. Before any task it reads a project graph slice of the whole repository. The last published graph generation on this host is 10.7 days old and older than the graph code, so each run starts a fresh read (3600 owners, 126 000 files), under a 6 GB memory request, on a host that swaps. In 300 s it got through about a third (20 of 64 taken). Whether it ends, and after how long, is not measured: a run without the 5 minute limit is still open.
- `repo test-file` knows three suite tables (repo build, caching, process: `R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔬️test-file/🔣️.json`). Every staffage test falls to the `plain` route and runs under `bun test`, although all staffage suites are Vitest. Not repaired (repo tool, outside staffage).
- Consequence: build, tests and dev server ran with the pinned bun through `repo capacity run`, with the scripts of `🧰️linux/s1/`.

## 2. Build

| Step | Command | Exit | Result |
| --- | --- | --- | --- |
| DWG encoder | `🧰️linux/s1/wasm.sh` | 0 | 5 s, `S/🔨️modules/📤️export/🖊️dwg/staffage_dwg.wasm` 186 229 B, validates, 6 exports. Built with rustc directly: cargo wants the dev-dependencies from the registry (`ezdwg-core` missing offline) |
| Release, first try | `build.sh release` | 1 | `Canonical bounded Vite compiler cache path required`: the cache path must lie in `.cache/b/p/m`; script corrected |
| Release, second try | `build.sh release` | 1 | 25 s: `"resolve" is not exported by "__vite-browser-external"` from `🗂️catalog/📥️files/🟦️.ts`, reached through the air flow worker. Site defect, repaired (section 4, A) |
| Release, third try | `build.sh release` | 124 | killed by my 900 s limit while checking the catalog (40 CPU minutes used) |
| Prototype | `build.sh prototype` (traced) | 0 | 946 s build, 18 min 37 s with queue; 375 asset modules. Slowest checks: fig-tree 62 s, ginkgo 32 s, bamboo-clump 25 s |
| Findings in the build | `node artifact-findings.mjs dist/prototype` | 0 | 375 modules of 375 entries, 0 missing, 0 with findings, 0 changed since the build |

The catalog check (`validateAsset` for 375 assets on 3 threads) is nearly all of the build time; a build under 16 minutes is not possible on this host at present.

## 3. Browser

Chromium 156.0.8078.4 of Playwright 1.64.1, headless, WebGL2 through SwiftShader, browser locale French so that the site has to ask for the language. Site served by the static stand-in `☀️17/…/serve.mjs` (exact files, `404.html` for misses) from `dist/release` of the build of 11:18. Command per context: `smoke.sh release final-<context> --only=<context>`.

| Context | Exit | Checks | Console and network |
| --- | --- | --- | --- |
| English desktop 1440 × 900 | 0 | 17 of 17 | none |
| German desktop 1440 × 900 | 0 | 17 of 17 | none |
| English phone 390 × 844 | 0 | 14 of 14 | none |
| German phone 320 × 640 | 0 | 14 of 14 | none |

What each context checks (the desktop ones all of it, the phone ones without the lines marked desktop):

| Area | Check |
| --- | --- |
| Language | gate offers English and Deutsch, the document has no `lang` before the choice and the chosen one after; kept after a reload |
| Catalog | overview links every one of the 9 categories and all assets (14 destinations on desktop, 13 on phone), no horizontal overflow; Humans opens from the overview (phone: 75 of 75 listed, 44 px touch target); all assets: phone lists 375 of 375; English desktop shows 375 cards with none busy and none marked unavailable when read |
| Detail | Walking Commuter / Pendler zu Fuß: side elevation drawn with marks, localized heading, clip plays |
| Three viewpoints | side elevation, front elevation and plan each by address with the selector showing it; from side elevation to front and plan through the selector; the three-view sheet with all three cells drawn |
| 3D view | model stage reaches `ready`, canvas shows 23 to 27 colours |
| Print | PDF export of the standing pose (22 462 B) and the browser's own print to PDF (49 842 B on German desktop) |
| Download | SVG 29 985 B; desktop: DXF 19 954 B, DWG 12 140 B (`AC1024`), GLB 7 903 352 B, animated FBX 1 022 012 B, WebM 324 720 B, each checked by its signature |
| Desktop only | one asset of every category draws (walking-commuter, labrador-dog, agave, park-bench, laptop, street-lamp, city-car, excavator, cantilever-balcony); collections, a two-pane comparison (line and thermal) and the playground open |

Limits of these checks: one asset per category was opened, not 375; the "375 cards" check reads the cards once and took 15 s, it does not prove that every preview was drawn; the phone is an emulated viewport with touch, not a device; no light theme, no playground placing.

Seen in the screenshots and not followed up: on the German desktop detail the labels of the segmented controls in the right panel overlap ("dunkle Strichzeichnung", "Thermikansicht", "Kraftansicht", "Momentenansicht", "Seitenansicht", "Vorderansicht"); `final-desktop-de/desktop-de-6-model.png`.

Earlier runs of this session, for the record: German phone on the prototype 12 of 14 (2 failures from a wrong label in my script); English desktop on the prototype failed twice because the memory guard ended Chromium at 4099 MB and 4245 MB (records `~/.cache/semio/host-capacity/warden/cancelled/2339126.*.json`, `2346232.*.json`, `reason cap`, `capMegabytes 4096`); German desktop on the release of 10:59: 9 of 17 (my script measured an SVG the playing figure had replaced), then 15 of 17 (the policy violation of D, and my script reading "Nicht verfügbar" in the laptop's dimensions table as an unavailable asset).

## 4. Changes (all files)

A. Release build broken since the air flow worker landed (2026-11-22):

- `S/🌐️site/🏗️builder/🌐️vite/🟦️.ts`: the stand-in for the Node cast exporter is now `staffageBrowserCastVitePlugin()` and is also in `worker.plugins`. Vite bundles a worker with `worker.plugins` alone, so the worker reached the Node exporter through the library barrel.
- New test `S/🌐️site/🧪️tests/🧵️flow-worker/` (`🥒️.feature`, `🟦️.ts`). Red before: 2 of 2 failed with the build's own error. Green after: `SEMIO_TEST_LEVEL=long site-test.sh flow-worker`, exit 0, 2 of 2.

B. Desktop overview used more than 4 GB within a minute. Measured in the page (`desktop-probe.mjs`, `desktop-profile.mjs`): 835 148 DOM nodes after 38 s with 31 000 attached; renderer 3 GB after 45 s; call tree `decodeGeometry < get < postMessage < retain < retainDrawing`. Three causes, all in `S/🎯️targets/⚛️react/🔨️modules`:

1. `🧮️pipeline/🏁️drawing/queue.ts` posted the decoded asset to the drawing worker. The structured clone reads every deferred geometry of every scale on the page and copies it. Now a catalog asset travels as the compressed payload it was shipped as. New `📚️catalog/🗜️payload/🟦️.ts` (`unpackAsset`, `packedAsset`); `📚️catalog/🟦️.ts` uses it; `🏁️drawing/🟦️.ts` has the request kind `packed` and `drawingBench`; `🏁️drawing/worker.ts` uses it. Same for the air flow worker: new `🌊️flow/🧵️server/wire.ts`, changed `🌊️flow/🧵️server/worker.ts` and `🌊️flow/📡️session/🟦️.ts`.
2. `🧮️pipeline/🧬️svg/🟦️.ts` kept up to 1024 parsed SVG templates whatever their size. Now bounded by weight: `SVG_TEMPLATE_BUDGET_CHARACTERS`, first 2 000 000, finally 8 000 000 (about 24 characters per node measured; reason for the final value under C).
3. `🧮️pipeline/🟦️.ts` kept up to 512 evaluated instants per asset. Measured with `state-weight.ts`: 330 kB per instant of the walking commuter, 600 kB of the sunflower. Now one budget for all assets: `STATE_BUDGET_POINTS` = 400 000 (about 150 MB).

New tests in `S/🎯️targets/⚛️react/🧪️tests/`: `📮️drawing-transport/`, `🧬️svg-templates/`, `⚖️state-budget/` (each `🥒️.feature` and `🟦️.tsx`). Runs:

- `react-test.sh drawing-transport svg-templates` before the change: exit 1 (missing functions).
- `react-test.sh state-budget clock-frame-budget drawing-transport svg-templates figure-render`: exit 0, 21 of 21.
- `react-test.sh drawing-transport svg-templates catalog-loading catalog-payload clock-frame-budget 🌊️flow/`: exit 1, 44 of 45. The failing case is `catalog-loading` "loads the authored archive as one ready row per manifest entry": timed out at its 300 s limit. It checks all 375 assets, which takes 16 minutes here; not shown to pass or fail.

Effect in the browser after B alone, before C, on the dev server (`dev-probe.sh desktop-probe.mjs`, logs `s1-desktop-probe8*`, 160 s on the idle English desktop overview):

- The browser lived through the 160 s; before the repairs the guard ended it after 60 to 100 s.
- DOM nodes swing between 76 000 and 765 000 (collected in between) instead of growing without end; the page's script heap stays at 80 to 190 MB.
- The renderer still grows: 1.0 GB at 15 s, 2.1 GB at 45 s, 2.0 GB at 90 s, 3.0 GB at 160 s (all Chromium processes 3.6 GB). It is not a plateau.
- Cause of the remaining growth, seen in the same log: the layered overview keeps opening more pages behind the glass while it idles. 1 page at 30 s, 3 at 46 s, 6 at 80 s, 10 at 128 s, 15 at 159 s (four categories and nine asset pages, each with live figures). This is what C repairs.

C. Pages mounted on an idle overview:

- Cause: not a fault of the framework element. `LayeredOverview` warms pages by design (`R/🧰️framework/🔨️modules/🖱️ui/🔨️modules/🥞️layered-overview-geometry/🟦️.ts`, `nextWarmBoot`: first page after 1.5 s, one more every 35 s, at most `budget` per overview). Staffage nests a `Level` in every page, and each nested `Level` ran its own warm queue with its own budget of 4, also while it lay behind the glass. So the bound was 4 per level times every nested level, not 4.
- Change, staffage only: `S/🎯️targets/⚛️react/🔨️modules/🥞️levels/🟦️.tsx` gives the element a budget of 0 for a grid that is neither the one the reader is on nor has an opened page. No framework file was edited.
- New test `S/🎯️targets/⚛️react/🧪️tests/🥞️level-warming/` (`🥒️.feature`, `🟦️.tsx`), five minutes of fake time. Red before: a grid behind the glass mounted 4 pages, nested grids reached 20 pages (2 of 3 failed). Green after: 3 of 3.
- `react-test.sh level-warming grid-scroll shared-borders mount-archive viewpoint-pages model-viewer-page` beside the running release build: exit 1, 46 of 47; the failing case was `model-viewer-page` "switches to the 3D model with one patch". Rerun alone: exit 0, 7 of 7. It failed once under load and passed alone; not explained further.
- The release build started at 10:37 was stopped by me at 10:39 (exit 143): it had bundled the code before this change.

Re-measurement of the idle English desktop overview on the dev server, same probe:

| Run | Template budget | Pages mounted | Renderer memory | End |
| --- | --- | --- | --- | --- |
| probe 8, before C | 2 M | 1 at 30 s, 15 at 159 s, rising | 1.0 GB at 15 s, 3.0 GB at 160 s | lived 160 s |
| probe 9, after C | 2 M | root and 3 categories at 123 s, no asset page | 2.3 GB at 40 s, 2.8 to 3.0 GB from 100 s to 150 s | guard ended the browser at 150 s: 4127 MB against the cap 4096 MB |
| probe 10, after C | 8 M | root and 4 categories at 157 s, the same at 192 s, no asset page | 2.1 to 2.8 GB from 40 s to 200 s, one sample 3.2 GB | lived 200 s, all Chromium at most 3.7 GB |

- The number of pages is bounded now: it stops at the budget of the root grid and no nested grid adds any.
- **The memory target is not met.** The renderer no longer follows the page count, but it stays at 2 to 3 GB and all of Chromium at 3.0 to 3.7 GB, which is just under the guard's cap; one of two runs was ended by it. This is a level, not a low plateau.
- What holds that level, as far as measured: the seven live figures of the overview itself. Each is about 4 200 SVG elements and is replaced whole on every frame of the 12 per second ambient clock; DOM nodes swing between 100 000 and 900 000 before they are collected. The warmed category pages add little: they do not load their assets while behind the glass (`useAssetLoading` in `🗃️browse`).
- Not changed, for the dev to decide: the previews ask for the 1:100 drawing (`RowPreview` default `scale = 100`, the frieze the same), also for a figure 40 px high. A coarser authored drawing for small previews would cut the elements several times over but changes how the overview looks.
- Template budget set to 8 000 000 characters after these two runs (one run each, so weak evidence): at 2 M the loops of the seven figures (3.5 to 10 M characters) did not fit and every frame was parsed again.
- `react-test.sh svg-templates level-warming state-budget drawing-transport` after the last change: exit 0, 13 of 13.

D. Release only: the air flow layer was unstyled and the console reported a policy violation.

- Found by the German desktop smoke on the release: `Applying inline style violates … style-src 'self' 'sha256-…'` on `#/compare?p=walking-commuter~dark-line&p=walking-commuter~thermal`. `S/🎯️targets/⚛️react/🔨️modules/🌊️flow/🖥️stage/🟦️.ts` made a `<style>` element at run time, which the release policy blocks.
- Change: `styleFlowLayer` in the same file adopts one constructable style sheet (as the 3D stage does), with the `<style>` element only where the platform has none.
- New test `S/🎯️targets/⚛️react/🧪️tests/🎨️flow-style/` (`🥒️.feature`, `🟦️.tsx`). Red before: 2 of 2 failed. `react-test.sh flow-style air-controls 🌊️flow/` after: exit 0, 27 of 27.

Release builds:

| Step | Exit | Result |
| --- | --- | --- |
| `build.sh release` with A, B, C (10:59) | 0 | 734 s, 375 asset modules, 96 MB |
| `build.sh release` with A to D (11:18) | 0 | 728 s, 375 asset modules; this is the build that was smoked |
| `artifact-findings.mjs dist/release` | 0 | 375 of 375 modules, 0 missing, 0 with findings, 0 changed since the build |
| `☀️17/…/check-release-artifact.mjs dist/release --require-current` (on the 10:59 build) | 1 | the checker itself fails at its line 47 (`JSON.parse(payload.json)`): it predates the compressed payload; nothing learned about the release |

Coordinator's ruling: B and C are site and React-target work and stay. D is in the same target.

Also written: `S/🔨️modules/📤️export/🖊️dwg/staffage_dwg.wasm` (git-ignored build output at its declared place), `dist/prototype` and partial `dist/release` of the site package, `R/.cache/b/p/m/vite-staffage-s1`.

## 5. Open

1. Memory of the idle desktop overview (section 4, C): 2 to 3 GB in the renderer. The lever that is left is the weight of the live previews (1:100 drawings of about 4 200 SVG elements, replaced whole 12 times a second); that is a decision about how the overview looks.
2. `repo run` for staffage: whether the graph read ends without the 5 minute limit was not measured. `repo test-file` runs staffage's Vitest cases under `bun test`.
3. `☀️17/…/check-release-artifact.mjs` no longer matches the shipped payload.
4. `catalog-loading`'s whole-archive case cannot end inside its 300 s limit on this host (the check of 375 assets takes 12 to 16 minutes); `model-viewer-page` failed once beside a build and passed alone.
5. Typecheck and the full React and site suites were not run.
6. German desktop: overlapping labels in the detail's right panel.
7. `🗑️generated/linux-s1` (25 MB) is left for the coordinator to read and then delete with the ticket.
