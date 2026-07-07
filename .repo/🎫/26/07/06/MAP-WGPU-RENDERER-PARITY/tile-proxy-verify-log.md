# GIS map tile proxy verify log

## Standalone tile proxy (`startGisMapTileProxyServer`)

```
[DEBUG] osm 200 image/png
[DEBUG] vt 200 application/x-protobuf
```

Direct fetch to `http://127.0.0.1:6141/osm/0/0/0.png` and `/vt/0/0/0.pbf` both return 200 with correct content types.

## Rust tests (`gis2d-plugin`)

```
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Includes new `render_canvas_uses_absolute_tile_urls_when_env_set`.

## Trunk proxy config

`framework/renderer/wgpu/Trunk.toml` forwards `/osm/` and `/vt/` to `http://127.0.0.1:6141/`.

## Tile fetch regression fix (2026-07-07)

**Root cause:** On native, `spawn_app_task` uses `pollster::block_on` inside `frame()` while the runtime `RefCell` is already borrowed. The asset-poll task's `try_borrow()` failed immediately, `asset_poll_pending` stayed `true`, and no map tiles were ever fetched/applied.

**Fix:** `poll_pending_assets()` collects pending tiles synchronously in `frame()` and fetches them with blocking HTTP on native; wasm keeps async `fetch()` with a `Drop` guard that always clears `asset_poll_pending`. Relative `/osm/…` URLs resolve via `SEMIO_GIS_MAP_TILE_BASE_URL` (default `http://127.0.0.1:6141`).
