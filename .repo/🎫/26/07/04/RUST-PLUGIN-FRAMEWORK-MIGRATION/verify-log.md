# Rust Plugin Framework Migration — Verification Log

## Rust tests

- `cargo test -p semio-framework-core --lib` — 2 passed
- `cargo test -p semio-framework-os --lib` — 1 passed
- `cargo test -p draw-plugin` — 1 passed
- `bun nx run @semio-tech/framework-renderer-react:test` — 1 passed

## Plugin build

- All 25 plugins built to `framework/product/os/dev/public/plugins/*/`
- wasm-bindgen 0.2.126 glue generated per plugin

## Browser boot

- OS dev server started on port 6164 with `SEMIO_PLUGIN=draw`
- `curl http://localhost:6164/` — 200
- `curl http://localhost:6164/plugins/draw/draw_plugin.js` — 200

## Hot-swap

- `framework/product/os/dev/script.ts plugin watch` rebuilds plugin crates on change
- Plugin host in `semio-framework-os` bumps instance generation on `hot_swap_plugin`
