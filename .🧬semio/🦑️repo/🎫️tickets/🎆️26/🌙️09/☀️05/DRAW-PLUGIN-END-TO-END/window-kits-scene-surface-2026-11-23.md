# Window-kits scene_surface migration

## Kit change (semio-plugin-window-kits)
- `WindowKit` trait keeps only `KIND_ID`, `window_kind`, `editable_window_kind`; `render` moved to an inherent method per kit (needed because Text/Mesh/Table/Document now take a locale, Tree/Image/Media do not).
- Names: text editor "Text editor"/"Texteditor", read-only text "Text"/"Text", mesh "3D view"/"3D-Ansicht", table "Table"/"Tabelle", document page "Document page"/"Dokumentseite".
- New signatures (locale last): `TextWindowKit::{render, render_read_only, render_draft}`, `MeshWindowKit::render`, `TableWindowKit::{render, render_editable, render_editable_cells}`, `DocumentWindowKit::{render, render_windowed}`.

## Framework tests
surface-carriers, reconcile-budget, tool-run (uses `view_state.locale`), app-window-kits updated.

## Plugin callers
About 100 plugin files plus their app impls/tests threaded with `view_state.locale` (stdio `mesh_scene` helper takes `locale`; procedural `is_de` replaced by `Locale`).

## Check
`repo cargo check -p semio-plugin-window-kits --target wasm32-wasip2` passed (no errors).
Plugin crates only partially verified: unrelated in-progress errors elsewhere (semio-os-store-one-item fixture, onig_sys wasm toolchain, procedural-3d-testkit shift overflow).
