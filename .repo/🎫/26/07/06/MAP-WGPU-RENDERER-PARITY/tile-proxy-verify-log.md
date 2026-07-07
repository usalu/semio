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
