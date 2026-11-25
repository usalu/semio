# OS renderer suite handoff

Baseline `/private/tmp/claude-501/oa.txt`: 39 failed, 3562 passed, 19 files (plus 1 unhandled download rejection). After: 7 failed, 3594 passed, 3 files failed, all environmental (full run, `lockf -k /private/tmp/claude-501/semio-vitest.lock sh "$T/os-vitest.sh"` with `PLAYWRIGHT_BROWSERS_PATH` set). The two suites that cannot load add no counted tests. ui spot run (`compositor-vitest.sh Layout shared-border-dock prepared-readback text-flow`): 69 pass.

## Chromium launch (root cause for 14 tests)
The legacy headless shell (`chromium_headless_shell-1248`) on this Mac never finishes launching or delivering input: probe with `headless: true` alone timed out at launch (20 s); `channel: "chromium"` (new headless) launched in 0.8 to 3.9 s, `navigator.gpu.requestAdapter()` succeeded (Apple metal-3 with the metal args, Google swiftshader with `--use-angle=swiftshader`), `mouse.wheel` returned.
- One helper: `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌐️chromium-launch/🟦️.ts` (`launchChromium(options)`, forces `channel: "chromium"`, lazy playwright import; `chromiumProvisioned()` for skips). 56 call sites in 41 ui and os renderer test files now use it (literal `chromium.launch` is gone from ui and from os `📺️renderer`, including the 4 `channel: "chromium"` sites of the ui Layout tests).
- Not migrated (open): `🧑‍💻dev` journeys and e2e harness (they load playwright through `PLAYWRIGHT_MODULE_SPECIFIER`/`loadPlaywrightV1` and `browserLaunchOptionsV1()`, not through vitest), `◻️2d` text alignment, `🖌️raster` svg-export, `🧊️3d` viewer gpu, `📓️print` chart-mutations. They can import the same helper when their lanes run on this Mac.

## Before and after per test
| Test (file) | Before | After | Root cause |
| --- | --- | --- | --- |
| time-travel geometry, phone, tablet, desktop (3) | rows `[navbar, region, bleed, footer, subfooter]` | pass | stale (law 4), see below |
| time-travel reprojection-status, stepped load | `[1,false,true]` | pass | stale source scan, see below |
| chrome-palette (2) | 30 s timeout | pass | headless shell |
| icon-svg-lighting WebGPU | timeout | pass | headless shell |
| ui prepared-readback WebGPU | timeout | pass | headless shell |
| IconRenderHost gpu-export, png-export, download (+ unhandled rejection), frame-presentation (2) | timeout | pass | headless shell |
| diff-view-produced-surface | timeout | pass | headless shell, then a real bug: split rows lost `gap-x-single` in `3cde99d423d`, so panes measured 196.3125 against the contract 193.125 (`🔺️DiffViewHost`, restored; fixture `diff-view-presentation` and the Rust `wgpu-diff-view` test agree) |
| MediaTransportHost lifecycle | esbuild "css without output path" | pass | test bundle imports `@xyflow/react/dist/style.css` through the ui index; `loader: { ".css": "empty" }` (same as the dev test config) |
| shell-option-locks, empty document picker | `Bun is not defined` | pass | runner is node; the scan reads the file with `node:fs` |
| editable-controls FEM fields x4 | no intent | pass | stale: number fields are continuous scrub presses on `onInput` (design 13.1), the test fired `change`; it types with a real `input` event |
| procedural-descriptors setLodMode | owners `[procedural-main]` | pass | stale: plugin fixture `generate-mode-interactions.json` (and the descriptor generated from it) declares it on generate-preview, preview and main |
| turn-patch-batch, ack batch | 60 s timeout | pass | stale: a reactor turn serves one event slot (`🎟️context/🌉️call/🎭️exports` refuses batches), `submitInstanceUiAcknowledgements` is one crossing per `patch-ack` in order; the test now answers N crossings and asserts N crossings, one event each, memoized repeat adds none |
| embedded-mount (9) | worker counts 2/4 vs 1/2, `undefined.locale` | pass | stale: the page host now also opens a local-catalog store Worker per mount (`createWgpuLocalCatalog`); the stub separates frame and shard workers (string URL) from store workers (URL object) and asserts one store Worker per mount that retires with its owner |
| embedded-mount, unique DOM ids | not reached | pass | real bug: every mount's tutorial `<video>` had id `ui.tutorial.video`; the id is now scoped by the mount root id (`🎞️tutorial-video`) |
| package-integration, document archive bridge | `Uint8Array` vs array | pass | stale: the archive decoder returns zero-copy `Uint8Array` views by contract; the test compares their array form |

## Layout law 4 for the time-travel geometry test
Layout (non scroll) now has the canvas as `layout-canvas-bleed`, absolute, beneath the in-flow bands; bands are `data-layout-band` rows at `z-[1]`. Static markup runs no effects, so the test sets `--layout-clearance-top`, `--layout-clearance-bottom`, `--layout-subfooter-height` with the same sums as `useBandClearance`.
Kept: one in-flow row each in order (navbar, region, footer, bands row), rows stack inside the viewport, docks that are open, every band inside its row and viewport, over no other row, opaque, no two bands overlap, every control on top at its centre and 24 x 24, English and German, every stage of the corpus.
Changed to the new law: the bleed layer is excluded from the row stack and asserted on its own (absolute, from the top edge beneath navbar and footer, stops above the bands row, display none on a phone with an open panel), the bands also lie over the bleed in no place, and the navbar and footer bands win hit testing over it. The old "dock pulls its cap below the region into the footer band" (silhouette cap) became "dock lies between navbar and footer bands and keeps a height", matching Panels between the bands. The history-editing bands (subfooter row, below the bleed) behave as before.
reprojection-status: not caused by Layout or chrome. It scans `applyLoadDocumentEffect`; the in-flight document-load-gate work changed `showLoad(status)` to `showLoad(documentLoadRunningV1(status.state) && !stopping ? status : null)`. The regex follows that (every running step is folded in, a stopping one ends it).

## Environmental, left red with evidence (7 tests, 2 suites)
- `hidden-tab-surfaces`, `node-graph-hosts` (suites do not load): `🌊️flow/🎁️browser-package/🕸️bindings/` holds only `package.json`; `flow_core.js` and the wasm are not built, building needs cargo.
- package-integration, 5 tests: `spawnSync bun ENOENT`, `which bun` finds nothing; `packageManager` pins `bun@1.4.2`.
- package-integration, 2 tests: "Current WGPU package catalog digest drift" and `reviewed-catalog-bytes` false. The reviewed catalog digest needs the pinned Bun regeneration; present before any edit of this lane.

## Files changed
New helper above. Tests: time-travel `📐️geometry`, `📡️reprojection-status`; os `🧪️tests` `🆚️diff-view-produced-surface`, `♿️editable-controls`, `📖️procedural-descriptors`, `🔒️shell-option-locks/📚️example-offers-and-switching`, `🧺️turn-patch-batch`, `🪆️embedded-mount`, `🧩️package-integration`, `🎬️MediaTransportHost` lifecycle, `⌨️text-input-oracle` (type import), `🎬️wgpu-browser-media`, ui `🎨️Canvas/shared-border-dock`, ui `📐️Layout` component tests, plus the launch swap in the 41 files. Product: `🔺️DiffViewHost/🟦️.tsx` (`gap-x-single`), `🎬️media/🌐️browser/🎞️tutorial-video/🟦️.ts` (scoped id).
Commands: `export PLAYWRIGHT_BROWSERS_PATH="$R/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"; lockf -k /private/tmp/claude-501/semio-vitest.lock sh "$T/os-vitest.sh" <filters>`. Logs removed from `🗑️generated`.
