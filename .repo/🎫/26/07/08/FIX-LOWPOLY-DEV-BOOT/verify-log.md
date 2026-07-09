# Verify Log — Fix Lowpoly Dev Boot

## Root cause
- Stale `semio-framework-plugin` rlib referenced `_SEMIO_PLUGIN_INIT_HOOK` while plugins export `semio_plugin_install_bundle` via `plugin_exports!(bundle)`.
- Plugin WASM linked against old hook → `ensure_plugin_initialized()` never installed bundle → manifest was `pluginId: empty` with 0 apps, or stale artifacts lacked `surfaceKind`.

## Fix
- Rebuilt `semio-framework-plugin` and `lowpoly-plugin` for `wasm32-wasip2`.
- Re-transpiled plugin artifacts to `framework/product/os/dev/plugin-modules/lowpoly/`.

## Manifest check (node)
```
pluginId: lowpoly apps: 1 len: 41855
lowpoly-main surfaceKind: world-3d
lowpoly-uv surfaceKind: canvas-2d
```

## Dev server
- `bun run dev:lowpoly` serving at http://127.0.0.1:6178/
