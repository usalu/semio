# Window layout Rust migration: wgpu / renderer / plugin-runtime side

## Decisions
- DockNode stays the wgpu f32 view type. `dock_from_window_layout(&WindowLayoutNode)` and `dock_node_to_layout_root(&DockNode) -> WindowLayoutNode` (Row=Horizontal split, Column=Vertical split, sizes fractions, normalised exactly as before) live in `🛰️Dock/🎯️targets/🧊️wgpu/📐️layout-conversion-and-drop-zones`. A bare window root is read as a one-window stack.
- `window_layout_instance_titles` now delegates to contract `collect_instance_titles` (re-exported from `semio_ui_declaration::layout`/`semio_ui_layout::layout`, added to the re-export list).
- Dock `even_layout` stays DockNode-based and is built from contract `even_layout`.
- `semio_plugin` re-exports: dropped `create_tab_stack_layout`, `WindowLayoutAxisNode/Child/Root/StackNode/WindowNode`; added `Axis`, `WindowLayoutNode`.

## Edited
Dock (lib import, conversion, tab-chrome even_layout, tests wgpu-unit + layout-diffs + window-instances, app-mode-layouts axis names horizontal/vertical, concrete-window-instances.json `activeWindowId`), Shell (`dock_mode_layout_identity` now one recursive projection, dock keyboard-and-palette `collect_window_kind_ids`, tests: window-scope-laws parses the fixture as contract `WindowLayoutNode`, safe-area json split, navbar-footer, window-pane-chrome fractions 0.3/0.3/0.4, fixtures/shell 0.33333/0.66667), plugin builder (`collect_window_kind_ids(&layout.root)`), plugin app-builder tests (typed `Axis`, stack test rewritten), manifest app-label test, hub space manifest (`Axis::Horizontal`, fractions).
NOT touched: `active_window_kind_id`/`activeWindowKindId` on ViewModel/view-state (a different model, not the layout).

## Check / test list (integration agent)
- cargo check: semio-renderer-wgpu-dock, semio-renderer-wgpu-shell-* crates (state, session, windows, dock), semio-plugin, plugin builder crate, semio-framework (manifest), semio-hub-space.
- tests: `semio-renderer-wgpu-dock-laws` (now also runs the contract corpus `window_layout_corpus_tests`), shell chrome-parity (window-scope-laws, safe-area-and-incoming-layout, maximize-resync), navbar-footer parity, wgpu-window-pane-chrome (`locale_roundtrip_preserves_instance_titles...`), app-mode-layouts, plugin app-app-builder, manifest app-label.
- Watch: `Axis` now exported from `semio_plugin` (globs importing another `Axis` may clash on use); `even_layout` is also exported from the layout module (Dock aliases it as `even_window_layout`).
- `🐚️shell/🤖️generated/🟦️.js` not located under the renderer tree; regenerate with the shell generator the integrator knows.
