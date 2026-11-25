# Browser E2E v12: empty 3D viewport after the Viewer to Editor switch

## Status: narrowed, NOT fixed, NOT verified end to end
No product change was made and no hub rebuild was run (so the coordinator's cargo gate notice about `RUSTUP_TOOLCHAIN` did not affect anything: no gated build failed). The three scenarios were not rerun to exit 0. Reason below.

## What was reproduced (stock hub, Vite on 6022, harness with `wasm_optimizingjit=false`, one content process)
| Case | Result |
|---|---|
| Demo, first load | canvas 1366x572, 42689 non-background pixels: grid, gizmo and workpiece render. Same with and without the light Firefox prefs, so the lighter config does not disable the 3D scene. |
| Demo then pick Concrete Forest (no role switch) | combobox "Concrete Forest", canvas drawn, 42689 px (`e2e-process-v12-forest.png`). |
| Demo, Editor to Viewer | canvas present, drawn, 42689 px (`e2e-process-v12-viewer.png`). Viewer switch completes about 60 s after the click. |
| Viewer back to Editor | Firefox parent is SIGKILLed 3 to 5 s after the click, every time (about 15 attempts), with no console output after the click and no kernel jetsam record, no Firefox crash dump. |

So the regression is confined to the Viewer to Editor switch (the second role switch in one page life). v11's empty-viewport screenshot is that step completing under memory stress; here it does not complete at all.

## Evidence on cause (memory of the content process, `footprint -p`, Demo)
- after first load: about 1.7 GB
- after Editor to Viewer: about 2.7 GB, and it does not fall again in the following 60 s (the predecessor's memory is not given back)
- Viewer to Editor click: 3.1, 3.4, then 4.5 GB within 7 s, then the process dies

Each role switch costs about +1 GB that stays resident, so the second switch reaches the 4 GB class and the browser dies (macOS was also swapping heavily because other agents' rustc/ld ran at 5 to 7 GB each). The wasm guests run in shard workers (a worker plus four `shard-worker.js`), so the memory is not observable from the page (`WebAssembly.Memory` tracking in the window found nothing). A dead or memory-starved page explains the v11 symptom (R3F canvas present, nothing drawn, grid and gizmo missing, because the WebGL canvas is created or fed while the process is starved). I did not prove that last link; the v11 screenshot run is not reproducible here.

Not the cause (checked): WebGPU (the viewport is WebGL via R3F, and renders under the light prefs), Demo or Forest first-load scene publication, Concrete Forest picker path, the light Firefox config.

## Suggested fix direction (not implemented)
Release the retired predecessor's guest memory before or at publication of the successor (verify `destroyApp` in `switch-plugin-app` retirement actually frees the shard instance and the captured archive bytes held by `prepareDocumentSurfaceV1` and the Tasks row), then re-measure footprint across two switches; target is steady state near 1.7 GB.

## Harness: regression check added
`verify-process-firefox-bidi.ts`
- `nonBackgroundPixels`: dependency-free PNG decoder (zlib plus unfilter). The harness hides everything except `<canvas>` with a temporary style, screenshots each canvas rectangle and counts pixels that differ from the dominant colour.
- Exit 1 if no canvas, or fewer than `MIN_CANVAS_PIXELS` (default 200) differing pixels. `CANVAS_CHECK=0` disables. Report fields `canvasRects`, `canvasPixels`.
- `LIVE_LOG` (appends non-info console lines and each step result as they happen, so a killed browser still leaves a trace; `LIVE_ALL=1` for all levels).
- Firefox now starts with a private crash-data directory and copies it out if the browser dies.

## Changed files (all under the ticket folder)
`verify-process-firefox-bidi.ts`, `run-forest-render.sh`, `run-render-retry.sh`, `run-when-quiet.sh`, `light-prefs.js`, `wasm-mem-preload.js`, scenarios `process-forest-render.json`, `process-demo-viewer-editor.json`, `process-demo-viewer-editor-probed.json`, `process-demo-viewer.json`, `process-wasm-mem.json`, `footprint-demo-role-switches.txt`, screenshots `e2e-process-v12-forest.png`, `e2e-process-v12-viewer.png`. No source files outside the ticket were changed. Temporary outputs in `🗑️generated` were removed; no `[DEBUG]` lines were added to product code.

## Next
1. Quiet the machine (other agents' multi-GB rustc/ld runs), rerun `process-demo-viewer-editor-probed.json` via `run-render-retry.sh` (`LIGHT=1`).
2. Implement the memory release above, rebuild through `T/bin/cargo` (now with `RUSTUP_TOOLCHAIN`), run `rebuild-and-boot.sh`, then the three scenarios.
