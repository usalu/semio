# Layout Technology Playground — verification log

## Tests (2026-07-02)

- `@semio-tech/layout-core:test` — 8 passed
- `@semio-tech/layout-react:test` — 2 passed
- `@semio-tech/layout-play:test` — 4 passed + `cargo test -p layout_rs` — 3 passed

## Build

- `layout/play` direct `bun ./script.ts build` — success, includes `layout_rs_bg.wasm` (~2.9 MiB)

## Dev server

- `LAYOUT_PLAY_PORT=6079 bun ./script.ts dev` — Vite ready on http://127.0.0.1:6079/

## Preflight (seeded fixture)

Default document includes deliberate issues verified in core tests:

- `asset.missing` (link-missing)
- `text.below_minimum_size` (frame-small-text on page-2)

## Export

Rust unit tests cover SVG serializer; PNG/PDF/package APIs exposed on `LayoutSession` wasm-bindgen surface.
