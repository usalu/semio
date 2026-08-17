# Wgpu Dock Corner Chips Report

## Summary

Refactored the wgpu `Dock` renderer and `Shell` handlers for four-corner tab groups with per-tab inline actions (focus / new / close), matching the React mode-dock model.

## Data model

- Added `DockStackTab { window_id, corner }` with default corner `TopLeft`.
- `DockNode::Stack.windows` is now `Vec<DockStackTab>`.
- `DockDropZone::Tab` now carries `corner: WindowStackCorner`.
- Layout round-trip preserves corner via `stack_from_node` / `layout_window_node`.
- `insert_tab(s)_at_corner` map corner-local indices onto the flat tab list.
- `stack_windows_at_path` still returns `Vec<String>`; `stack_tabs_at_path` returns full tabs.
- `close_window_in_stack(path, window_id)` closes a specific tab; `close_active_in_stack` delegates to it.

## Paint / silhouette

- `render_stack` paints up to four corner chip groups and registers:
  - `dock.tab.{path}.{window_id}`
  - `dock.tab.{path}.{window_id}.focus`
  - `dock.tab.{path}.{window_id}.new`
  - `dock.tab.{path}.{window_id}.close`
- Removed stack-level `dock.focus.{path}` / `dock.close.{path}` and `render_cap_action_group`.
- `WindowSilhouette::from_measured_edges(bounds, top_spans, bottom_spans, top_depth, bottom_depth)` added; `from_measured_top` kept as a thin wrapper.
- `layout_stack_cap` returns per-corner groups + top/bottom silhouette spans.

## Drop zones

- `compute_dock_drop_zone` / `drop_zone_indicator_rect` take `(path, corner, rect, widths)`.
- `DockState::stack_corner_tab_bar_rects` feeds Shell.
- `ShellState::dock_tab_bars_for_drop` uses the corner-aware API; `dock_drop_tab_bars` field type updated.

## Shell handlers

- Replaced `dock.focus.*` / `dock.close.*` with `dock.tab.*.focus` / `dock.tab.*.close`.
- Added `parse_dock_tab_action_id`.
- Updated `shell_command_for_control` + unit assertions.
- Tab drag start / pending-click parsers strip `.focus` / `.close` / `.new` suffixes.

## Tests

- Stack constructors use `DockStackTab` helpers (`tab` / `tabs` / `stack_tabs`).
- Drop-zone fixtures include `corner`.
- Hit-id assertions expect per-tab `.focus` / `.close` and reject stack-level `dock.focus.` / `dock.close.`.
- Added `apply_drop_tab_moves_window_to_target_corner`.

## Supporting fixes (compile unblock)

- Re-exported `WindowStackCorner` from `ui_wgpu` glue.
- Added `corner: None` to `WindowLayoutWindowNode` struct literals across the tree that still omitted the new field after the schema change.

## Compile / test status

`cargo test -p semio-framework-os-renderer-wgpu dock::` did **not** finish: dependency `semio-s-plugin-stdio` still fails with unrelated errors:

- `unresolved import semio_framework_plugin::ArtifactKindId`
- missing `BinaryMutation` / `BinarySnapshot` in a binary artifact schema path

No Dock/Shell-specific rustc errors were observed before the dependency failure. Re-run dock tests once stdio compiles.

## Known follow-ups

- Wire `dock.tab.*.new` to mint an extra window instance (React `onWindowOpenInNewWindow`); hit target is registered, Shell handler not yet implemented.
- Empty-corner drop pads during an active drag (React renders pads; wgpu currently still exposes empty-corner bar rects via `stack_corner_tab_bar_rects`).
- Confirm runtime hit routing with `[DEBUG]` logs once the package builds.
