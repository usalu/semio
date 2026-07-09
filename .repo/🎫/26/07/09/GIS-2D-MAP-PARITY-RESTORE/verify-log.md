# GIS 2D Map Parity Restore — Verify Log

## Rust

- `cargo build -p gis-plugin --target wasm32-wasip2 --release` — **pass**
- `cargo test -p gis-plugin` — **blocked** (pre-existing: `plugin_exports!` macro requires `wasm32` + `p2` env; native test harness cannot link `component_export_anchor`)

## WASM packages

- `bun nx run @semio-tech/gis-2d-rs:wasm` — **pass** (`gis_2d_bg.wasm` 4.31 MiB)

## React

- Added `GisMapHost` for `componentKind: gis2d-map`
- Wired lazy import in `ui-interpreter.tsx`
- Added `@semio-tech/gis-2d-rs` workspace dependency

## Window measures

- `Gis2dPlayApp::window_measures()` returns render mode, vector style, LOD, selection method, layer toggles, layer weight sliders for `gis2d-main`

## Dev server (`bun run dev:gis:2d`)

- **pass** — Vite starts on `http://127.0.0.1:6040/` (no missing `./pkg/gis_2d.js` export error)
- **pass** — `/@fs/.../gis/2d/rs/pkg/gis_2d.js` + `gis_2d_bg.wasm` served (200)
- **pass** — tile proxy `/osm/0/0/0.png` + `/vt/0/0/0.pbf` return 200

## Fixes for E2E boot

- `gis/2d/rs/package.json` — export `./pkg/gis_2d.js` subpath (matches `@semio-tech/framework-graph-rs`)
- `framework/product/os/dev/js/vite.config.ts` — `gisMapTilesVitePlugins` when `SEMIO_PLUGIN=gis2d`, wasm `assetsInclude`, wasm optimizeDeps exclude
- `framework/product/os/dev/script.ts` — build `gis-2d-rs` wasm in `buildEngineWasm` for `gis2d`

## Playwright smoke (headless, 2026-07-09)

- **pass** — navbar/footer visible, app name `semio · gis · 2d`, no actionable console errors
- **pass** — canvas present, side panel tabs populated
- **pass** — Inspection panel shows MAP VIEW controls (render mode, vector style, LOD, selection method, layer weights)
- **pass** — Document tree lists map layers (Raster, Water, Land, Roads, …)
- **note** — headless Chromium reports `gpuReady: false`; tile `fetch()` skipped until WebGPU adapter available (real browser required for raster/vector tile paint verification)

## Browser smoke (manual, real GPU)

Open GIS 2D app and confirm:
- Raster + vector tiles visible
- Pan (middle button), wheel zoom, marquee/lasso selection
- Hover tooltip on positions
- Right-click context menu
- Window options rail populated (window measures overlay on main window)
