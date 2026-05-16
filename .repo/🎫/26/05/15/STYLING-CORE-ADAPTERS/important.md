# Styling core + adapters

- **Core:** `elements/core/styling/tokens.json` (colors, spacing, fonts, `board_vello_canvas`).
- **JS/Tailwind:** `bun ./elements/client/lib/styling/script.ts generate` → `elements/client/lib/styling/generated/*.css`; `palette.css` imports generated files only.
- **Rust:** `elements/client/lib/board/rs/build.rs` → `OUT_DIR/elements_styling_board.rs`; `lib.rs` uses `board_palette::*`.
- **.NET:** `semio/client/lib/net/Elements.Styling` + generated `Palette.g.cs` / `BoardVelloCanvas`; referenced from `Semio`, `Semio.Grasshopper`, `Semio.Rhino`.
- **Monorepo.sln:** project paths aligned under `semio/client/...`; solution folder renamed to avoid duplicate `semio` name conflict.
- **Other:** Rhino `Semio.Rhino.cs` template placeholders (`__DOT_ID_UPPER__`, etc.) repaired; full solution still hits Rhino API / GH test issues unrelated to tokens.
