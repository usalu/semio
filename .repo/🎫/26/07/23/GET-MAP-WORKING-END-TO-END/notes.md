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
