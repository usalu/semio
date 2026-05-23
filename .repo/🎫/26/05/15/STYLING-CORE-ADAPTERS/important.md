# Styling core + adapters

- **Core (framework-neutral):** `elements/core/styling/tokens.json` — colors, spacing, fonts, `board_vello_canvas`. Discover adapters via `elements/core/styling/adapters.manifest.json`.
- **Nx:** `nx run @elements/styling-core:generate` (or `bun ./elements/core/styling/script.ts generate`).
- **JS / Tailwind:** `elements/core/styling/js/tailwind/generate.ts` → `elements/client/lib/styling/generated/*.css`; `@elements/styling` (`palette.css`) imports generated files only.
- **Rust (board Vello):** `elements/core/styling/rs/board_vello_build.inc.rs` — included from `elements/client/lib/board/rs/build.rs`; emits `OUT_DIR/elements_styling_board.rs`.
- **.NET:** `Elements.Styling/Generated/Palette.g.cs` (+ `BoardVelloCanvas`) under `semio/client/lib/net/Elements.Styling` — referenced from `Semio`, `Semio.Grasshopper`, `Semio.Rhino`.
- **Monorepo.sln:** project paths under `semio/client/...`; Grasshopper uses `Elements.Styling` via `ProjectReference` (no duplicate palette in GH UI code).
