# Pass 2 — History + CAD Inspector trees

## Root causes
1. History `ui_history_panel` still built `ui_stack_vertical(header, filter, tree)` — Stack root nested a Tree (orphaned icons / nested chrome).
2. Prior rewrite mangled the signature (`&stpub fn…`) and ate the following `Emit` docstring.
3. CAD Inspection empty selection still returned `ui_stack_vertical` of `ui_text` (schema/utility/objects).

## Fixes
- `ui_history_panel` → pure `UiNode::Tree` (Actions + Commands), same shape as Document/Catalogue.
- Restored HistoryPanel endregion + Emit docstring; removed dead `ui_stack_horizontal_tight`.
- CAD empty `build_properties_panel` → `ui_inspector_groups_to_tree` with readonly fields.
- History render test updated for Tree root.

## Verification
- `cargo test -p semio-framework-plugin --lib ui_history_panel` — pass
- `cargo test -p semio-framework-plugin --lib rendering_the_history_body` — pass
- `cargo check -p semio-s-app-cad-ui` — pass
