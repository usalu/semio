# Fix Inspector Trees To Match Document

## Problem
Inspection side-panel bodies in several apps returned `ui_stack_vertical` of `Field` nodes.
That violates the side-panel law (Panel → Tabs → Tree → Sections → Items) and rendered as a
malformed gray card with a stray document icon, unlike the clean Document trees built with
`PanelTreeBuilder`.

## Fix
- Convert always-stack inspectors to `ui_inspector_groups_to_tree` (block 2d/3d/5d, vcs, imperative).
- Convert empty/no-selection inspector fallbacks from stacks/`ui_text` to
  `ui_declarative_sections_to_tree` (forms, layout, shooting, puzzle 3d/5d, lowpoly, procedural 2d/3d,
  process 3d, trinity rewrite).
- Stop defaulting leaf tree items without `icon_id` to `file-text` in the widget tree painter so
  inspector field rows stay as clean as Document rows (explicit icons only).

## Verification
- `cargo test -p semio-s-app-block-3d-ui --lib` — pass (incl. tree-root assertion)
- `cargo test -p semio-s-app-block-2d-ui --lib` — pass
- `cargo test -p semio-s-app-block-5d-ui --lib` — pass
- `cargo test -p semio-framework-ui-wgpu --lib` — compiles after icon default change

## Note
Repo MCP auth was skipped in this session; ticket folder was created locally.
