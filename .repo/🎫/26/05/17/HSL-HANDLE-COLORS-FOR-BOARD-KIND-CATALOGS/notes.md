Root cause: `nakagin-capsule-tower.board.json` meta `handleKinds` use CSS Color Level 4 `hsl(H S% L%)`; `BoardHost::set_board_kind_catalogs_from_json` only called `parse_css_hex_color`, so WASM rejected catalogs and `BoardCanvas` never finished init (frozen).

Fix: `parse_css_color` tries hex then `parse_css_hsl_color` (hsl/hsla, comma or space, optional `/` alpha). Per-handle `color` in scene descriptor uses the same parser.

Tests: `cargo test board_host_kind_catalog_accepts_modern_hsl`, `bun ./rs/scripts/build-wasm.script.ts`, `bunx vitest run` in board (67 passed).
