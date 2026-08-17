# Get Map Working End to End

## Problem
1. `gis-plugin` failed to compile: `UiInspectorFieldGroup` missing required `presence` field (two sites in 2D inspector).
2. After compile fix, playground booted with wrong app id: registry had `app = "gis2d"` / `"gis3d"` but plugin registers `gis2d-play` / `gis3d-play` → `appId "gis2d" does not resolve to any app in the loaded plugin manifest`.

## Fix
- Added `presence: UiPresence::default()` to both missing `UiInspectorFieldGroup` initializers in `gis/plugin/rs/lib.rs`.
- Aligned `gis/plugin/rs/Cargo.toml` playground + asset `app` fields to `gis2d-play` / `gis3d-play`, regenerated plugin registry.

## Verify
- `cargo build -p gis-plugin --target wasm32-wasip2 --release` ok
- `SEMIO_RENDERER=react bun run dev:gis:2d` → http://127.0.0.1:6040/
- Playwright: title `semio · gis · 2d`, 1 canvas, Document/Catalogue/Inspection/Map chrome, Reuse Map example

# GIS 2D map render error

## Symptoms
- Window showed `Render error: Cannot read properties of undefined (reading 'map')`
- Console also logged `setActiveExample` failing: app actions rejected unless sent through typed command channel

## Root causes
1. `TiledMapHost` called `mapTiledContextMenu(contextMenu.specs)` while state was typed/initialized with `items: []` (no `specs`) — every render hit `specs.map` on `undefined`.
2. After B1, `VcsDocumentApp::dispatch_action` only accepted framework-reserved verbs; React still sends `{action,args}` for app verbs like `setActiveExample`.

## Fixes
1. React `TiledMapHost`: state uses `specs`, destructure `requestContextMenu`, map with `specs ?? []`; harden `mapContextMenuSpecs` against null/undefined.
2. Framework `DocumentApp::command_from_action` bridge + `dispatch_action` else-arm calls it; GIS implements full action→`Gis2dCommand` mapping (incl. `camera`→`camera_json`, `hover`→`hover_json`).
3. Rebuilt `gis` plugin wasm so the browser loads the bridge.

## Verified
- Playwright: no Render error, Map canvas mounted, Reuse Map example, OSM/VT tile requests, no action failures
- `cargo test -p semio-s-app-gis-2d-ui --lib set_active_example` — 2 ok
