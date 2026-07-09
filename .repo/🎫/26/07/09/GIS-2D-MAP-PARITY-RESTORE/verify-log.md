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

- **pass** — `framework/core/js/index.ts` parse fix: both `withSerializedPluginWasmHandle({…})` returns close with `});` (not `};`)
- **pass** — `mathematical/graph/port/directed/dag/rs/lib.rs` `slider_track_bounds(&node)` fixes graph wasm build during dev boot
- **pass** — Vite starts, `gis_2d.js` resolves, tile proxy `/osm/` + `/vt/` return 200
- **pass** — Playwright smoke: app `semio · gis · 2d`, canvas renders, no actionable console errors

## Browser smoke (manual, real GPU)

Open GIS 2D app and confirm:
- Raster + vector tiles visible
- Pan (middle button), wheel zoom, marquee/lasso selection
- Hover tooltip on positions
- Right-click context menu
- Window options rail populated (window measures overlay on main window)
