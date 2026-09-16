# Inspector panel tree migration audit

## Target pattern

- Body: `PanelTreeBuilder` → `Component::Tree`
- Rows: `treeItem` (controls as **children**, see procedural3d / VCS / Interpreter `collectTreeItemControls`)
- Read-only: `tree_item_desc`
- Verbs / picks: `tree_item_with_action`

## Migrated (no `ui::column` / `ui::field` in `✏️s/🔌️plugins/**/🔍️inspection/🦀️.rs`)

- fem2d, fem3d, energy model (large form inspectors)
- shooting, animate, block 2d/3d/5d
- dag, architect, lowpoly, space
- Already tree-native: vcs, cad, puzzle*, procedural3d, process3d, …

## Framework fixes (TreeWindow / typed catalog)

- `TreeItemProps` / `TreeSectionProps` `window` + `granularity` in `🧾️typed/🦀️.rs`
- `u32` in typed copy/compare/retire scalars for `TreeWindow`
- `TreeWindowRequest` import in `semio-framework-plugin`

## Verification

- `rg` over plugin inspection panels: zero `ui::column` / `ui::field`
- fem2d inspector unit tests updated for tree layout; full crate test run pending green compile
