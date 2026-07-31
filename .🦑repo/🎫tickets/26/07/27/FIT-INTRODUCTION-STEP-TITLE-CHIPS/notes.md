# Verification

- `bun nx run @semio-tech/ui-react:test-quick -- -t "introduction"`: 13 passed.
- `bun nx run @semio-tech/ui-react:typecheck`: blocked by existing unrelated generated `StyleSpec`, icon resolver, DOM `Node`, and stale introduction fixture diagnostics.
- `bun run dev:storybook:ui`: blocked by existing Storybook/Vite dependency optimizer failures involving `vite/internal`, native `.node` loaders, and `chromium-bidi`.
- `bun run dev:mit-bestand:aggregator`: blocked by the existing wasm build error in `vcs/rs/lib.rs:2190`, where `hash_bytes` is unavailable for `wasm32`.

The focused regression renders the Aggregator title `Willkommen bei Entwerfen mit Bestand` and verifies that the title chip drops the generic 12rem cap, ellipsis, fixed height, and hidden overflow; the window uses intrinsic width with a viewport maximum, the title wraps and breaks long words only when constrained, and body copy retains its readable maximum width.
