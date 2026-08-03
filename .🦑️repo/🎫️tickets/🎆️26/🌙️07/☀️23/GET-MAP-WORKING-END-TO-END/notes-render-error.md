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
