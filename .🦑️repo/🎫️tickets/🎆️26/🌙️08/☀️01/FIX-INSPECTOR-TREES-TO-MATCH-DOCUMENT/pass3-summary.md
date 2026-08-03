# Pass 3 — History/Inspector trees match Settings/Theme

## Root cause
Settings/Theme on React are hand-built `TreePanelConfig`s (labeled sections, label+control rows).
History/Inspector plugin bodies that were not recognized as trees (or older stacks) went through
`uiNodeToTreePanelConfig`'s fallback: empty-label item hosting `interpretUiNode` — which rendered as a
lone document/`file-text` icon above nested content.

wgpu Settings/Theme were still `Stack`s, so they could not share the same Tree chrome as History.

## Fixes
1. React `uiNodeToTreePanelConfig`: real declarative→tree conversion; no empty-label wrapper.
   Tree nodes only enable section sorting/DnD when `dropAction` is set (like Settings).
2. wgpu `build_settings_general_ui` / `build_settings_theme_ui` → `UiNode::Tree` with label+control rows.
3. History action rows use Button controls (Settings shape); filter stays Select control.
4. Inspector Field→tree no longer mirrors input value as `description` (value lives in control only).

## Verify
- `cargo test -p semio-framework-plugin --lib ui_history_panel` — pass
- `cargo check -p semio-framework-os-renderer-wgpu` — pass
- `cargo check -p semio-s-app-cad-ui` — pass (prior)
