# Table parity fix (wgpu vs React): what changed, what is NOT verified

Nothing below was compiled or run. The shared store was saturated and the host build policy changed mid-task, so the integration agent compiles and tests everything. A first `cargo test` attempt was refused before the compiler: `semio-ui-wgpu-paint` is over its source boundary (4215/4000 active lines), caused by other streams, not by this work.

All paths below are relative to `/Users/ueli/Documents/semio/🧰️framework/`. `UI` = `🔨️modules/🖱️ui`, `ENG` = `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`.

## Verified audit claims (read, not run)

- The audit is right that the document Table split the width equally (`🧮️layout` `table_column_rect`). React uses `minmax(192px, 192px)` per column (`TABLE_COLUMN_WINDOW_WIDTH_PX = 192` in `🪟️tree-windows`) and an actions track of `n x size-medium + n x spacing-single`.
- `--size-medium` is 22.4px, the same as the wgpu `theme.control_height` (7 x 3.2).
- The surface table (`🧮️table-and-paint2d`) divided columns equally, drew a literal `"No rows"`, painted a text glyph for sorting and projected no `aria-sort`.
- The shared a11y-mirror node had no `sort` field.
- Correction to the audit: the document `Table` record has no sort props in either renderer. Sorting and `aria-sort` exist only on the OS surface table (`TableHost`).

## Shared model and vectors (new)

- Model: `UI/🎯️targets/🧊️wgpu/📊️table-model/🦀️.rs`, exported as `semio_ui_wgpu_layout::table` from `🧱️layout-stack/🦀️.rs`. It holds:
  - fixed `TableTrack` min/max tracks and `table_column_template`, which is byte-identical to the TS template;
  - actions track, content width, max horizontal scroll, `table_scroll_left_for_column` and `table_scroll_top_for_row`;
  - row and column keys, `table_key` (focus, Shift range, Ctrl/Meta toggle, activate, Escape) and `table_merge_for_modifiers`;
  - sort next, `aria-sort`, the `ui.common.noData` label in en/de, and `TableAvatar` initials.
- Vectors: `UI/🧪️tests/🧫️fixtures/📊️table-column-model/🔣️.json`.
- Rust test: `UI/🧪️tests/📊️table-column-model/🦀️.rs`, registered in the companion `🧱️layout-stack/🧪️tests/🥣️companion/🦀️.rs`.
- TS test: `UI/🧪️tests/📊️table-column-model/🟦️.ts`. It exercises the existing `tableWindow*` and `tableSort*` functions against the same vectors.

## Wired into live paths

1. **Document-table fixed columns and scroll.**
   - `UI/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs`: `table_column_rect(row_width, row_height, column, metrics)` is now fixed 192px with a gap inset. `table_actions_width` is now `slots x (control_height + gap)`.
   - `📐️flex` `LayoutNodeKind::TableRow` gained `content`. Cells are fixed width (`192 - 2 x gap`, no grow or shrink, `gap_main = 2 x gap`) and the row has `min_width = content`.
   - `📌️mounted_layout` `tree_row_kind` computes `content`.
   - `🖌️paint/📝️retained-text-steps`: the header is laid out over the content width and shifted by `scroll_offset.0` of the table node.
   - `🌳️retained/🌳️tree/🔀️reconcile/🌳️ui-tree-reconcile`: a `Component::Table` node is `SCROLLABLE` and `CLIPS_CHILDREN`. Rows, editable cells, hit tests and action slots shift with the arena walk origin.
   - `⚡️events/✍️drag-scroll-and-edit-api` `route_scroll`: it now bubbles to the next scrollable ancestor when a scrollable has no range on the axes of the delta. A table therefore scrolls horizontally without swallowing vertical wheel events.
2. **Surface-table keyboard and selection.**
   - `ENG/🧱️elements/🎞️Scenes/.../🧮️table-and-paint2d`: `table_row_key` emits the same actions as React (`interactionSelect` with merge `range` or `invertive`, else `selectRow`). `table_row_id_at` and a focused-row focus ring were added.
   - `🗃️scene-state`: `SceneSurfaceState.table_focused_row`, with retirement handling, and accessors in `📷️scene-camera-dispatch`.
   - `ENG/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/✍️text-and-table-focus`: `FOCUSED_TABLE_ROW`, `focus_table_row`, `apply_focused_table_row_key`, `blur_focused_table_row_for_pointer`.
   - `🖱️scene-interaction`: a press focuses the row.
   - `ENG/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🎮️interaction-state`: the key lane runs after the stepper lane, and a pointer press blurs the row.
3. **Sort and `aria-sort`.**
   - The header glyph is now `↑`/`↓` as in React, and the next direction goes through the shared model.
   - `ui_contract::AccessibilityProjectionNode.sort: Option<String>` was added. All explicit struct literals got `sort: None`, in `🧬️contract/♿️accessibility`, the `🔬️introspection` files and `🪀️view-panels` / `🗨️view-overlays` `theme-file-import-and-accessibility`.
   - The introspection builder `append_table_button_accessibility_nodes` now also emits one `columnheader` node per table column, with `sort`. This needed a new dependency `semio-renderer-wgpu-scene-tables` in the `🔎️ui-engine-dump` `Cargo.toml`, and `TableColumn.sortable` became `pub`.
   - The TS mirror (`ENG/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts`) has `sort` on the node type and sets `aria-sort`.
4. **Localized empty state.**
   - `TableChromeLabels { no_data }` on `SceneChromeLabels`. `render_table(.., labels)` draws it centered; `"No rows"` is gone.
   - `shell_chrome_string("common.noData")` has en "No data" and de "Keine Daten", the same text as React's `ui.common.noData`.

## Second pass (code only, still uncompiled)

- **Surface-table columns:** React's `TableHost` passes no column widths, so its `<table class="w-full">` auto layout shares the width equally. The equal split already matched, and now goes through the shared model (`table_fill_track_px`, vector `fillTrack`, used by `table_metrics`). The fixed 192px model applies only to the document table, as in React.
- **Action slot pitch:** `TreeRowMetrics.table` (set by `retained_tree_row_metrics` for document tables) makes `tree_row_action_rect` return React's slot. That is a `size-medium` (22.4) button with `spacing-single` (3.2) gap, pitch 25.6, which is the same track the actions column reserves. Paint draws the 12px icon centered in the button. The `table-row-grid` fixture `actionSlots` and `clicks` were updated (slot 1 x 454.4, slot 2 x 428.8, y 0.8, size 22.4), and the slot test uses `.with_table(true)`.
- **Document-table keyboard:** `EventRouter::focused_table_row_key` (`⚡️events/⌨️widget-keyboard-and-commit`) is called from the KeyDown chain before the disclosure activation. It uses `table::table_key`: Up/Down/PageUp/PageDown/Home/End move focus over the table's materialised sibling rows, and Enter/Space fire the row's `Trigger::Activate` through `dispatch_accessibility(Activate)`, the same path as React's `activateRow`. Escape and keys with Ctrl/Meta/Alt are left to the generic routes.
- **Double-click and context menu, checked against React and not added:**
  - React's document `TableView` activates a row on double-click (`onDoubleClick` calls `activateRow`) and on Enter/Space. A single click only focuses (`moveTo`).
  - React's document table has no row context menu or trigger button. Its row actions are all rendered as buttons (`TableRowActionButton`). wgpu already announces every action, Row and Menu placed, as a mirror button.
  - wgpu still fires the row's activation on a single press, as the fixture clicks encode. I did not change that, because it would change product behaviour. The mismatch with React is a decision for the integration agent: either drop click-activation and add double-click, or keep it. The engine has no double-click event and no row context-menu command yet.
  - The surface table's right-click already resolves through `table_context_menu_target`, like React's `onRowContextMenu`.

## Not done, or only partly done

- **Avatars:** only `table_avatar_initials` (model plus vector). Nothing is painted, because table cells are text only and React's `Table` renders no avatars (`TableAvatar` is used by HistoryTable, PresenceBar and VirtualFileSystem).
- **Document-table keyboard:** the cell and action walk (Right/Left over actions, Ctrl+arrows over cells, Escape back to the row) is not wired, because actions and cells are not focusable arena nodes. Rows beyond the materialised window are not reachable by keyboard.
- **Selection:** the document table has none in React, so none was added. Shift/Ctrl selection exists only on the surface-table key lane.
- **Context menu key and Shift+F10:** not routed for table rows.
- **Actions placement:** with a viewport wider than the content, actions sit at the row's inline end, not right after the last column as in React.
- **`aria-sort`:** header nodes have no rect and are not actionable.
- **Document-table scroll:** the effect of `CLIPS_CHILDREN` on hit-testing and the paint scissor after a horizontal shift is untested.

## Tests to run (framework workspace unless noted)

Existing fixtures changed: `UI/🧪️tests/🧫️fixtures/📊️table-row-grid/🔣️.json`. Columns are now `x 3.2 / 195.2` with `width 185.6`, `actionsWidth 51.2`, plus new `contentWidth` and `narrowViewportWidth`.

1. `repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p semio-ui-wgpu-layout-laws`
   - `table_column_model_tests::*` (new).
   - `table_row_grid_tests::*` (action slots now 22.4 buttons at pitch 25.6, columns 185.6 wide), including the new `a_viewport_narrower_than_the_columns_keeps_every_row_content_wide_and_scrolls` and the metrics check `size-medium is the control height`.
2. `... test -p semio-ui-wgpu-events-laws`: `route_scroll` changed and `focused_table_row_key` was added. Add a test that focuses a row of the `table-row-grid` fixture and sends ArrowDown/End/Enter. Expect focus to move to the next row, then `open-space` to fire on Enter. Overlay/drag/scroll laws must still pass: offset `(0,30)`, clamp at zero, clamp at the extent.
3. `... test -p semio-ui-wgpu-mounted-layout-laws` and `-p semio-ui-wgpu-paint-laws`: `TableRow` kind, the changed `table_column_rect` signature, header and cells painting.
4. `... test -p semio-ui-retained-tree` (or its laws crate): table node flags `SCROLLABLE`/`CLIPS_CHILDREN`, and the `tree-row-rects` and reconcile laws.
5. `repo cargo ... -- test -p semio-renderer-wgpu-scene-tables` (root workspace) and the `🎞️Scenes/🧪️tests/🔬️wgpu-table/🦀️.rs` suite: `render_table` takes `labels`. Check header hits, sort and the empty label.
6. Compile-only, root workspace: `semio-renderer-wgpu-ui-engine-focus`, `semio-renderer-wgpu-ui-engine-dump`, the Shell crate that owns `scene_chrome_labels` and the shell vocab crate. Also `semio-ui-contract` and every crate with an `AccessibilityProjectionNode` literal. Expect possible errors in `-engine-dump` (new `scene-tables` dependency or a layering rule), `scene-interaction` and `interaction-state`.
7. TS: `bun test` / vitest on `UI/🧪️tests/📊️table-column-model/🟦️.ts` and `UI/🧪️tests/📊️table-sort-header/🟦️.ts`. The mirror type change needs the accessibility-mirror TS check.
8. Not yet written (follow-ups): a test for `table_row_key` and `apply_focused_table_row_key` (add to the `🔬️wgpu-table` suite, next to the stepper-keyboard fixture), a header `aria-sort` projection test, and an en/de `common.noData` assertion in `🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n`.
