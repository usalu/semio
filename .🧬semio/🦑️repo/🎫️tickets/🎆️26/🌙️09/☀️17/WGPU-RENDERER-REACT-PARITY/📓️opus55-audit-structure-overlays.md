# WGPU renderer vs React parity audit: structure, surfaces, overlays, window layout

Read-only audit. No source file was edited and no git command was run. Only this report was written.

## Path key

All paths are under `/Users/ueli/Documents/semio/`. Short prefixes used in the tables:

| Prefix | Expands to |
|---|---|
| `RUI/🧱️elements/` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/` (React elements) |
| `RUI/🎯️targets/🧊️wgpu/` | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/` (wgpu UI engine) |
| `RUI/🧬️contract/🖼️presentation/` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/` (Rust contract) |
| `RUI/📜️declaration/📐️layout/` | `🧰️framework/🔨️modules/🖱️ui/📜️declaration/📐️layout/` (declaration layer used by wgpu shell) |
| `OS/🧱️elements/🗣️Interpreter/` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/` (React document interpreter) |
| `OS/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/` | the same engine root, `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/` (wgpu surface kinds) |
| `OS/🎯️targets/🧊️wgpu/` | the same engine root, `🎯️targets/🧊️wgpu/` (OS wgpu target) |
| `OS/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/` | shell staging used by the OS chrome overlays and dock |

Emoji in path names are kept exactly as they appear on disk. Line numbers were taken from grep or sed output at the time of the audit.

## Method and limits

- Read React Tree, Table, Surface, ContextMenu, Popover, Dialog, Select, Tabs, Resizable, Scrollable, Window (partly), the document interpreter (`TableView`, surface bridge, `ExtensionView` case), the Rust contract files for surfaces, overlays and window layout, and the wgpu tree, table, surface, overlay, keyboard, dock and accessibility-mirror code paths.
- Absence claims ("no X in wgpu") come from grep over the named directories. They are stated as "not located" and are not proof of absence. Anything I could not check directly is marked **unverified**.
- No runtime comparison (screenshots, probes, the React or wgpu hosts running) was done.
- Severity: **High** = visible or behavioural mismatch that breaks a documented contract or an accessibility rule. **Medium** = visual or behavioural drift with a clear fix. **Low** = cosmetic, duplication, or already matching.

## Key structural findings (read these first)

1. **Two wgpu tree paths.** The document Tree/TreeItem/Table nodes that the shared UI document produces are painted by `RUI/🎯️targets/🧊️wgpu/🖌️paint/🌲️retained-tree-node-painting/🦀️.rs`. The older immediate widget path (`RUI/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs` and `RUI/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs`) still exists with different constants. Gaps below are marked by which path they apply to.
2. **Two Rust window-layout models, neither matching React exactly.**
   - Contract `WindowLayoutNode` (`RUI/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:417`, variants `Window`, `Stack`, `Split { axis: Axis }`) has **no renderer consumer**. A grep for `WindowLayoutNode` finds only its own file, the schema projection, and a comment in `OS/🎯️targets/🧊️wgpu/🛰️dock/🦀️.rs:1`.
   - The live wgpu shell and dock use `semio_ui_declaration::layout::WindowLayout` (`RUI/📜️declaration/📐️layout/🧭️layout-contract/🪟️window-layout-tree/🦀️.rs:120-177`, `WindowLayoutRoot::{Axis, Stack}`, string `kind` "row"/"column").
   - React uses a third shape (`🧰️framework/🔨️modules/🖱️ui/🔨️modules/🗺️window-layout/🟦️.ts:6-27`: `row`/`column`, `id`, `activeId`).
3. **The shared overlay surface is not the chrome the shell uses.** Shell overlays (`OS/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs:31-36`) use `push_glass`. The generic overlay path used for Dialog/Popover/Tooltip (`RUI/🎯️targets/🧊️wgpu/🖌️paint/🖼️ui-image-sources-skeleton-and-overlays/🦀️.rs:346-384`) paints an opaque surface with a hardcoded scrim. React uses glass and tokenised veil for all of them.
4. **Document Table layout is not React's.** React sizes document table columns to fixed pixels (`OS/🧱️elements/🗣️Interpreter/📊️table-window/🟦️.ts:62-65`). wgpu divides the width equally (`RUI/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:383-389`).
5. **Keyboard navigation for Tree and Table is absent in wgpu.** React implements WAI-ARIA tree and grid keys. wgpu only has a DOM accessibility mirror that answers Enter and Space on focusable actionable rows.

---

## 1. Tree / TreeSection / TreeItem

Primary React: `RUI/🧱️elements/🌳️Tree/🟦️.tsx`, `🎨️chrome/🟦️.tsx`, `📏️row-alignment/🟦️.tsx`, `🌱️section/🟦️.tsx`, `🍃️item/🟦️.tsx`, `⌨️roving/🟦️.ts`, `🎚️selection/🟦️.ts`, `🧲️drag-drop/🟦️.tsx`, `🔭️virtual-window/🟦️.tsx`. Interpreter: `OS/🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx`.

Primary wgpu: `RUI/🎯️targets/🧊️wgpu/🖌️paint/🌲️retained-tree-node-painting/🦀️.rs` (document path), `RUI/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs` (`TreeRowMetrics`), `RUI/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs` and `RUI/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs` (legacy widget path), `OS/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts` (DOM mirror).

| Aspect | React file:line | wgpu file:line | Gap | Severity |
|---|---|---|---|---|
| Row pitch and presentation (standard / compact / touch) | `RUI/🧱️elements/🌳️Tree/🟦️.tsx:45-46` (touch 44 px), `:535-541` (row-height vars per presentation), `📏️row-alignment/🟦️.tsx:29` (`treeRowUiSpacing`) | `RUI/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:156`, `:168-191` (`TreeRowMetrics::for_presentation`, touch clamps to 44 px), `retained-tree-node-painting/🦀️.rs:23-24` | Document path matches. Widget path ignores presentation: `RUI/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:21-24` and `RUI/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs:127-147` use one `TREE_ROW_HEIGHT`. | Low |
| Keyboard: Up/Down, Home/End, Right/Left expand-collapse, Enter | `⌨️roving/🟦️.ts:7-27` (`treeNavigateV1`), `:52-68` (`handleTreeKeyDownV1`), root `onKeyDown` at `🟦️.tsx:547` | No tree key handler. Only Enter/Space on actionable rows via `OS/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:170`. Engine-owned keys only for slider/spinbutton at `:46-54`. | Whole tree key model missing | High |
| Typeahead on tree rows | not located in `⌨️roving/🟦️.ts` (React Select has typeahead, `🔽️Select/🟦️.tsx:460`) | none | Not in React tree either | Low (parity note) |
| Roving tabindex and ARIA tree state | `🟦️.tsx:502-516` (roving tabindex sync via MutationObserver), `aria-level`/`aria-expanded`/`aria-selected` set on rows | mirror sets `aria-selected`, `aria-expanded`, `aria-level` (`accessibility-mirror/🟦️.ts:127-136`). No roving tabindex. | Only one Tab stop in React; wgpu rows are each tab-reachable | High |
| Section header fold chevron | `🌱️section/🟦️.tsx:198` (`treeFoldChevronIcon`), `:231` (gutter slot), `🎨️chrome/🟦️.tsx:42` (`treeSectionChevronClassName`) | `retained-tree-node-painting/🦀️.rs:57-69` draws the folder icon only; `:75-84` draws the label. No chevron. | Section chevron missing on document path | High |
| Section label typography | `🎨️chrome/🟦️.tsx:41` (`text-xs font-semibold uppercase leading-none tracking-wide`) | `retained-tree-node-painting/🦀️.rs:80` (`theme.font_size_small`, no weight, no uppercase) | Not uppercase, not semibold, no tracking | Medium |
| Item label typography | `🎨️chrome/🟦️.tsx:39` (`text-xs font-normal leading-none`), `:38` (`--tree-label-size` var) | `retained-tree-node-painting/🦀️.rs:189-195` (`theme.font_size_body`, compact uses `SIZE_TINY`) | Token mapping unverified (`text-xs` vs `font_size_body`) | Medium (unverified) |
| Indent per level and toggle slot | `📏️row-alignment/🟦️.tsx:24-26`, `:30` (`treeToggleUiSpacing`) | `retained-tree-node-painting/🦀️.rs:143` and `:184` use `TREE_INDENT_PER_LEVEL` and `TREE_TOGGLE_WIDTH` from the same `dom` tokens (`RUI/🎯️targets/🧊️wgpu/🖌️paint/🔤️retained-glyph-and-node-cursors/🦀️.rs:6-10`) | Tokens match | Low |
| Indent guide lines (branch elbows, parent stems) | `📏️row-alignment/🟦️.tsx:58` (guide stroke), `:63-77` (`IndentationLines`), `:88-130` (`TreeDocumentGutter` elbows and stems), `🎨️chrome/🟦️.tsx:63-77` | `tree_draw_guides` is referenced only in `RUI/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs` and `RUI/🎯️targets/🧊️wgpu/🖌️paint/🔬️reference/🦀️.rs`. The document painter `retained-tree-node-painting/🦀️.rs` has no guide call. | Guides absent on the document Tree | High |
| Chevron direction (collapsed/expanded, up/down, RTL) | `🎨️chrome/🟦️.tsx:14-19` (`treeFoldChevronIcon`) | `retained-tree-node-painting/🦀️.rs:143-160` (same mirror rule: `chevron-right`/`chevron-left`, `chevron-down`/`chevron-up`) | Logic matches | Low |
| Selected, highlighted and hover fills; text emphasis | `🎨️chrome/🟦️.tsx:104-130` (selected `interactiveActiveFillClass`, highlighted `bg-hover-interactive-fill`, hover `groupHoverExcludingHandleBgFillClass`; selected text `text-emphasized`) | `retained-tree-node-painting/🦀️.rs:126-135` (`theme.selected`, `theme.row_hover`, `foreground_on_fill`), `:189`, `:225` | Token equivalence unverified. Fills are applied to the content rect only in both (ok). | Medium (unverified) |
| Selection gestures: click, Shift range, Ctrl/Meta additive, anchor | `🟦️.tsx:210-233` (`handleSelectItem`, `getTreeNextSelectionState`, `interactionMergeFromModifiers`), `🎚️selection/🟦️.ts` | Selection comes from `presence.selected` (`retained-tree-node-painting/🦀️.rs:126`). No modifier range or anchor logic located for tree rows. | Modifier selection not located | High (unverified) |
| Double-click | `🟦️.tsx:235-237` (`handleDoubleClickItem` → `item.onDoubleClick`) | none located in `RUI/🎯️targets/🧊️wgpu/⚡️events/` (only slider readout uses "double") | Missing | Medium |
| Drag reorder with drop preview | `🟦️.tsx:180-208` (`handleDragOverItem`, `TreeReorderDropPreview`), `:239-300` (drag start, drop), `🧲️drag-drop/🟦️.tsx` | Drag handle icon only: `retained-tree-node-painting/🦀️.rs:238-247`. No drop-preview indicator located. | Reorder preview not located | High (unverified) |
| Palette pointer drag (item to canvas, with 6 px threshold) | `🟦️.tsx:307-392` (`buildPalettePointerProps`, 36 px² threshold at `:357`) | not located | Not located | Medium (unverified) |
| Section reorder | `🟦️.tsx:408-447` (`handleSectionDragStart`, `handleSectionDrop`, `TREE_SECTION_REORDER_MIME`) | not located | Missing | Medium (unverified) |
| Row actions and menu placement | `🧾️row-actions` (React), `⚡️actions/🟦️.tsx` (267 lines) | `retained-tree-node-painting/🦀️.rs:250-270` draws actions right-to-left; `:257` skips `UiTreeActionPlacement::Menu`; hover label drawn at `:` action step | Menu-placement behaviour and label reveal not compared (unverified) | Medium (unverified) |
| Inline toolbar and detail band | contract `RUI/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs:146-151` (`inline_toolbar`, `detail`). React element sources for the detail band: `🌳️Tree/📄️row/🟦️.tsx` (unverified). | `retained-tree-node-painting/🦀️.rs:192`, `:222` reserve control width for `inline_toolbar`. Detail band painting not located (`mounted_layout/🌲️tree-metrics-and-window-measures/🦀️.rs` mentions `.detail` for measurement only). | Detail band paint not located | Medium (unverified) |
| Windowed / virtual rows (spacers, streaming) | `🔭️virtual-window/🟦️.tsx` (360 lines), `🪟️tree-windows/🟦️.ts`, `🧱️node-tree/🟦️.tsx` tree-window observer | spacers `retained-tree-node-painting/🦀️.rs:97`, `:138`, and `tree_window_spacer_px` in `🧮️layout/🦀️.rs` | Spacer model matches; scroll anchoring and request cadence unverified | Medium (unverified) |
| Section header spacing | `🎨️chrome/🟦️.tsx:33-36` (`treeSectionContentPaddingTopPx = 0`, sibling gaps 0) | document path adds none. Widget path adds `TREE_SECTION_GAP = 8.0` at `RUI/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:24`, used at `RUI/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs:144` | Widget path only | Low |
| Tree direction / RTL | `RUI/🧱️elements/🌳️Tree/🟦️.tsx:521` forces `<FlowProvider inline="ltr">`; `direction="up"` reverses sections (`:518`) | document path mirrors with flow (`retained-tree-node-painting/🦀️.rs:23`, `:64`, `:78`, `:143-157`) | React Tree never mirrors inline; wgpu does. Needs a decision (see W-13) | Medium |
| Empty state slot | `RUI/🧱️elements/🌳️Tree/🟦️.tsx:562-566` (`emptyState`) | not located | Missing | Low |
| Constants duplicated | n/a | `TREE_INDENT_PER_LEVEL`, `TREE_TOGGLE_WIDTH`, `TREE_ICON_SIZE` defined in both `RUI/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:21-23` and `🖌️paint/🔤️retained-glyph-and-node-cursors/🦀️.rs:6-10`. `TREE_ROW_HEIGHT` is in `RUI/🔨️…/🎨️styling/🎭️theme/📏️metrics/🦀️.rs:53`. | Duplicated constants | Low |

## 2. Table / TableRow

Primary React: `RUI/🧱️elements/📊️Table/🟦️.tsx` (immediate table, used by legacy and by the Table row UI), `OS/🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx:318-414` (`TableView`, the document Table), `OS/🧱️elements/🗣️Interpreter/📊️table-window/🟦️.ts:62-68`, `OS/🧱️elements/🗣️Interpreter/🧾️row-actions/🟦️.tsx`, `RUI/🧱️elements/📻️TableAvatar/🟦️.tsx`.

Primary wgpu: document path `RUI/🎯️targets/🧊️wgpu/🖌️paint/📝️retained-text-steps/🦀️.rs:279-345` (`retained_table_header_step`, `retained_table_cells_step`), layout `RUI/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:372-389`. Surface table: `OS/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧮️table-and-paint2d/🦀️.rs` (`table_metrics` `:283-292`, `render_table` `:726-816`, `render_table_cell` `:235-272`, hit `:679-716`).

| Aspect | React file:line | wgpu file:line | Gap | Severity |
|---|---|---|---|---|
| Column widths (document Table) | `OS/🧱️elements/🗣️Interpreter/📊️table-window/🟦️.ts:62-65` (`tableWindowColumnTemplate`: each column a fixed `TABLE_COLUMN_WINDOW_WIDTH_PX` `minmax`, actions column `size-medium` each). Constant import at `📊️table-window/🟦️.ts:6`; its definition was not located by grep. | `RUI/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:383-389` (`table_column_rect` shares the row width equally between columns; doc comment at `:380-382` says so). Actions width `:372-378` uses `drag_handle_extent + action_gap`, not `size-medium`. | Fixed-width columns with horizontal scroll (React) vs shrink-to-fit (wgpu) | High |
| Column `width` prop (legacy Table) | `RUI/🧱️elements/📊️Table/🟦️.tsx:66`, `:453`, `:565` (`style={{ width: column.width }}`) | `surface Table` columns ignore any width: `table_metrics` `:289` divides evenly | Legacy React Table honours width; wgpu never does | Medium |
| Keyboard: Up/Down, PageUp/PageDown, Home/End over the logical extent | `OS/🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx:315-317` (doc comment), `TableView` handler from `:318` | none. `RUI/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:160-164` define the keys, but no table consumer | Missing | High |
| Row activation Enter/Space | `RUI/🧱️elements/📊️Table/🟦️.tsx:48-51` (`rowActivationKeyV1`), `:225-226`, `:291-292`; `TableView` Enter/Space in `node-tree/🟦️.tsx:315-317` | Only the mirror's generic Enter/Space on actionable rows (`accessibility-mirror/🟦️.ts:170`) | Activation exists via a11y mirror; no column-level Right/Left walk, no Escape return | High |
| Cell focus: Right/Left walk actions, Escape returns to row | `node-tree/🟦️.tsx:314-317`, `:414` (`ArrowLeft` at position 0 focuses the row) | none | Missing | High |
| Sorting: click cycle asc/desc | `RUI/🧱️elements/📊️Table/🟦️.tsx:44-46` (`tableSortNextDirectionV1`), `:456` (header button) | sort hit on header: `table-and-paint2d/🦀️.rs:750-760` (`sortTable` action); next direction shared logic (`table_next_sort_direction`, `:346`) | Logic matches | Low |
| Sort indicator and `aria-sort` | `RUI/🧱️elements/📊️Table/🟦️.tsx:53-57` (`tableSortAriaV1` → `aria-sort`), `:451`, `:456-458` (button and icon) | sort indicator is a text glyph appended to the label (`▲` or `▼`, `table-and-paint2d/🦀️.rs` around `:743-748`). No `aria-sort` anywhere in `RUI/🎯️targets/🧊️wgpu` or `OS/🎯️targets/🧊️wgpu` (grep). | Screen readers get no sort state; visual indicator differs | Medium |
| Header typography and height | `RUI/🧱️elements/📊️Table/🟦️.tsx:565` (`text-sm font-medium h-large p-single`) | `table_metrics` `:284` (`header_h = control_height * 1.33`), label drawn with `font_size_small` at `:747` in `text`/`text_muted` (no weight) | Weight, height token and label colour differ | Medium |
| Row height (rowHeight preset) | `RUI/🧱️elements/📊️Table/🟦️.tsx:387-391`: `compact`, `normal`, `comfortable` all map to `h-medium` | `table_metrics` `:287` `row_h = theme.control_height` | Presets identical in React; wgpu matches only if `control_height` equals `size-medium` (unverified) | Low (unverified) |
| Row separator | `RUI/🧱️elements/📊️Table/🟦️.tsx:33` (`tableBandBorder`, shared border bottom) | `table-and-paint2d/🦀️.rs` row loop draws a 1 px line at the row bottom with `theme.separator` | Shared-border join with selected outline lost | Low |
| Selected / hover fills and text | `RUI/🧱️elements/📊️Table/🟦️.tsx:30` (`interactiveHoverClass`), `:36` (`interactiveActiveFillClass`), `:267` (`bg-hover-interactive-fill` for drop target) | `table-and-paint2d/🦀️.rs` row loop: `theme.selected` / `theme.row_hover`, `active_foreground` text | Token equivalence unverified; drop-target fill not present | Low |
| Row context menu trigger | `RUI/🧱️elements/📊️Table/🟦️.tsx:21-22` (`DragHandle`, `ContextMenuTriggerButton`), `:236`, `:302` (kebab button in last cell when `onRowContextMenu`) | right-click resolves through `table_context_menu_target` (`scene-render-entry/🦀️.rs:48`); no visible trigger button drawn | Visible affordance missing | Medium |
| Drag: activation delay, tolerance, drag handle | `RUI/🧱️elements/📊️Table/🟦️.tsx:89-102` (`DragDropConfig`: 2 delay/tolerance/distance fields), `:21` (`DragHandle`) | row drag uses a move icon handle (`table-and-paint2d/🦀️.rs`, `draggable && driver_drag == Handle`), hit with `drag_data`; no activation delay logic located | Delay/tolerance not matched (unverified) | Medium (unverified) |
| Row double-click | `RUI/🧱️elements/📊️Table/🟦️.tsx:112` (`onRowDoubleClick`), TableRow `onDoubleClick` | none located | Missing | Medium |
| Empty state text | `RUI/🧱️elements/📊️Table/🟦️.tsx:117`, `:345` (`emptyMessage` ?? `noDataLabel`, translated) | `table-and-paint2d/🦀️.rs:770` `let message = "No rows";` (literal English) | Not localised; not the React label | Medium |
| Range / row count label | `OS/🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx:344` (`useLabel("ui.host.tableRowRange", …)`), `:350-351` (`rowLabel`, `columnLabel`) | not located in wgpu table paint | Missing | Low |
| Virtualisation of rows | `node-tree/🟦️.tsx:318-360` (tree-window observer, viewport cap via `tableWindowViewportCapRowsV1`), `📊️table-window/🟦️.ts` | surface table: row cull by y and `list_visible_row_capacity` (`table-and-paint2d/🦀️.rs`, `reserve_list_rows`), document table: spacers `retained-tree-node-painting` | Cull model similar; request cadence and scroll anchor unverified | Medium (unverified) |
| Action column width | `OS/🧱️elements/🗣️Interpreter/📊️table-window/🟦️.ts:65` (`actions × size-medium + actions × spacing-single`) | `RUI/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:372-378` (`gap + slots × (drag_handle_extent + action_gap)`) | Formula differs | Medium |
| Avatars | `RUI/🧱️elements/📻️TableAvatar/🟦️.tsx:18-21` (`size-workbench`, `text-xs`). Used by `HistoryTable`, `PresenceBar`, `VirtualFileSystem`, not by Table. `TableRowProps.cells` is `UiText` only (`RUI/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs:47-49`). | `TableCellPayload` variants are Text, Number, EditableText, Stepper, Buttons (`table-and-paint2d/🦀️.rs:144`, `render_table_cell` `:235-272`). No avatar variant. | Document Table has no avatars in either renderer. Avatar parity belongs to VFS, PresenceBar, HistoryTable (unverified) | Low |
| Editable text, stepper, button cells | `RUI/🧱️elements/📊️Table/🟦️.tsx` cell content; document cells in `node-tree/🟦️.tsx` | `render_table_cell` `:244-266` (inputs with `commit: table-cell`, stepper `−`/`+` buttons, buttons); document path via `retained_table_cells_step` | Present on both | Low |
| Drag-over row target | `RUI/🧱️elements/📊️Table/🟦️.tsx:267` (`isOver` fill) | none | Drop-target highlight missing | Low |

## 3. Surface (15 surface kinds)

Contract: `RUI/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs:20-69` (`SurfaceKind`, 15 kebab-case tags), `:86-95` (`SurfaceProps`). React surface container `RUI/🧱️elements/🌈️Surface/🟦️.tsx:16-122` is a level/fill wrapper, not a kind host. The kind host map is `OS/🧱️elements/🗣️Interpreter/🔌️surface-bridge/🟦️.tsx:37-51`.

| Aspect | React file:line | wgpu file:line | Gap | Severity |
|---|---|---|---|---|
| All 15 kinds routed | `surface-bridge/🟦️.tsx:37-51` (kind → host), `:66-73` (scene lanes for text-editor, node-graph, world-3d, canvas-2d, board-2d, tiled-map, paint-2d) | `OS/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🎨️scene-render-entry/🦀️.rs:246-249` (World3d, Canvas2d, InkCanvas, IconRender), `:441-447` (Table, VirtualFileSystem, GraphTimeline, BlockList, DiffView, EventFeed), `:300-410` (phases 4-9: TextEditor, Paint2d, Board2d, NodeGraph, TiledMap through engine texture attach) | All kinds have a path | Low |
| Unexpected kind reaching list step | n/a | `scene-render-entry/🦀️.rs:448` (`_ => Fault`) | Fault, not silent | Low |
| Missing-payload fallback | not located for React host fallbacks (unverified) | `OS/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🖱️canvas-gestures-input-kit-and-chrome/🦀️.rs:194-197` `render_placeholder` draws `"{kind} host"` text. Called from `table-and-paint2d/🦀️.rs:730`, `block-list-and-diff/🦀️.rs:400`, `:661`, `canvas2d` shape-fills `:179`, `ink-canvas-render/🦀️.rs:302`, `event-feed-and-timeline/🦀️.rs:505`, `:850`, `board2d-and-vfs/🦀️.rs:599`. | Non-localised, developer-looking text on a missing payload | Medium (unverified React behaviour) |
| Context-menu surface id (camelCase) | `surface-bridge` menu vocabulary (`openSurfaceContextMenu`) | `scene-render-entry/🦀️.rs:197-214` (`context_menu_surface_kind_id` matches the camelCase list) | Matches | Low |
| Canvas-2d empty state text | `RUI/🧱️elements/📐️Canvas2dHost/🟦️.tsx` (`"Empty canvas"`, per wgpu doc comment at `canvas2d/🔷️shape-fills-and-render/🦀️.rs:172-175`) | `canvas2d/🔷️shape-fills-and-render/🦀️.rs` (doc only; render path not compared) | Unverified | Low (unverified) |
| Table surface row/column visuals | see section 2 | see section 2 | Same gaps | High (see section 2) |
| Surface hit targets and pointer routing | `🖼️surface-views/🟦️.tsx` (scene host triggers) | scene hit routing `scene-render-entry/🦀️.rs` (`scene_context_menu_target`, `:37-75`) | Not compared in depth (unverified) | Low (unverified) |

## 4. Extension

| Aspect | React file:line | wgpu file:line | Gap | Severity |
|---|---|---|---|---|
| `Component::Extension` rendering | `OS/🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx:582-583` (`case "extension": ExtensionView`), host contract `OS/🧱️elements/🗣️Interpreter/🧩️host-contract/🟦️.ts:9` (props and node identity) | No arm for `Component::Extension` in `RUI/🎯️targets/🧊️wgpu/🖌️paint/`, `RUI/🎯️targets/🧊️wgpu/📌️mounted_layout/`, or `RUI/🎯️targets/🧊️wgpu/🧮️layout/` (grep). Dynamic extension admission exists at `OS/🎯️targets/🧊️wgpu/🧩️dynamic-extension/🟦️.ts:41` and plugin invoke at `OS/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:91` but is not wired to document nodes. | Extension nodes are not painted on the wgpu document path; host contract not bridged | High (unverified: no generic fallthrough confirmed) |
| Unknown component fallback | `node-tree/🟦️.tsx:589` (`UnknownComponentView`) | none located | No visible fallback | Low |
| Contract | `RUI/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs:86-89` (`ExtensionProps { extension, props }`) | n/a | n/a | n/a |

## 5. Overlays (`OverlayKind`: SelectPopup, ContextMenu, Tooltip, Popover, Dialog, CommandPalette)

Contract: `RUI/🧬️contract/🖼️presentation/🪟️overlay/🦀️.rs` (placement `:6-150` incl. `resolve_anchored_placement`, `OverlayKind` `:200-270`, dismiss policy, `TOOLTIP_DWELL_SECONDS = 0.4` at `:220`). React: `RUI/🧱️elements/🗨️Popover/🟦️.tsx`, `💬️Dialog/🟦️.tsx`, `🖱️ContextMenu/🟦️.tsx`, `🖱️ContextMenu/🧭️navigation/🟦️.tsx`, `🔽️Select/🟦️.tsx`, `💡️ChromeControlHint/🟦️.tsx`, `🎨️styling/🖌️ui/🎨️.css` (`ui-glass`, `ui-veil`), `🎨️styling/🔣️.json:457-458` (`veilAlpha` 0.4).

wgpu: `RUI/🎯️targets/🧊️wgpu/⚡️events/🧲️overlays-drag-and-commands/🦀️.rs:91-130` (placement wrappers), `⚡️events/⏱️tooltip-clock-and-overlay-api/🦀️.rs` (open/close, dismiss, tooltip clock), `⚡️events/🖱️accessibility-and-pointer-dispatch/🦀️.rs:432-466` (Escape, Tab), `RUI/🎯️targets/🧊️wgpu/🖌️paint/🖼️ui-image-sources-skeleton-and-overlays/🦀️.rs:346-407` (backdrop and surface paint), `RUI/🎨️styling/🎭️theme/🧩️component/🦀️.rs:137, 286, 414-434` (`overlay_shadow`, `glass`, `veil`, `veil_blur_px`).

| Aspect | React file:line | wgpu file:line | Gap | Severity |
|---|---|---|---|---|
| Placement: side, align, flip, viewport clamp | `RUI/🧱️elements/🗨️Popover/🟦️.tsx:240-271` (`resolvePopoverPlacement`), `:288-293` (defaults 4 / 8) | contract `RUI/🧬️contract/🖼️presentation/🪟️overlay/🦀️.rs` (`resolve_anchored_placement`, `AnchoredPlacement::POPOVER/TOOLTIP/MENU`) used by `⚡️events/🧲️overlays-drag-and-commands/🦀️.rs:91-130` | Same contract on both sides | Low |
| Select positioning (inline start/end, RTL) | `RUI/🧱️elements/🔽️Select/🟦️.tsx:253-266` | `resolve_select_inline_left` in contract `overlay/🦀️.rs` | Parity | Low |
| Glass surface (translucent + blur) for floating surfaces | `RUI/🧱️elements/🗨️Popover/🟦️.tsx:442` (`glassClass`), `🔽️Select` (`glassClass`), `💡️ChromeControlHint` (`glassClass`, `level="menu"`), `🖌️ui/🎨️.css:7182-7186` (`ui-glass`) | `ui-image-sources-skeleton-and-overlays/🦀️.rs:358-363` (`paint_overlay_surface`): opaque `theme.surface(Level::Menu)`, no blur, no alpha | Opaque instead of glass for the shared overlay path. Shell overlays do use `push_glass` (`OS/…/📥️staging/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs:31-36`), so the two paths disagree | High |
| Scrim colour and alpha | `🖌️ui/🎨️.css:7188-7192` (`ui-veil`: `color-mix(surface-bg, veil-alpha)` + `backdrop-filter: blur(veil-blur)`), token `🎨️styling/🔣️.json:458` (`veilAlpha` 0.4), `RUI/🧱️elements/💬️Dialog/🟦️.tsx:417` | `ui-image-sources-skeleton-and-overlays/🦀️.rs:351` hardcodes `OVERLAY_BACKDROP_ALPHA = 0.55` over `theme.background`, no blur. `theme.veil(level)` and `veil_blur_px()` exist (`🎭️theme/🧩️component/🦀️.rs:427-434`) but the shared paint does not call them | Hardcoded, not tokenised, no blur | High |
| Surface shadow | `💡️ChromeControlHint/🟦️.tsx` uses `shadow-lg` (tooltip) | `paint_overlay_surface` draws a 1 px / 2 px offset quad in `theme.overlay_shadow`, whose default is `Rgba::TRANSPARENT` (`🎭️theme/🧩️component/🦀️.rs:286`), so no shadow shows unless a theme sets it | Shadow invisible by default | Medium |
| Corner radius | `Select` uses `rounded-sm`, glass `ui-glass` surfaces are rounded through the level tokens | `paint_overlay_surface` uses square `push_chrome_border(..., true, true, true, true, ...)` with no radius, although `theme.border_radius` exists (`🎭️theme/🧩️component/🦀️.rs:~291`) | Square corners | Medium |
| Dialog open and close animation (fade, zoom 95) | `RUI/🧱️elements/💬️Dialog/🟦️.tsx:417`, `:554` (`animate-in`, `zoom-in-95`, `fade-in-0`) | none located | No animation on wgpu | Low (unverified: wgpu may have a frame-based transition elsewhere) |
| Focus return on close | `💬️Dialog/🟦️.tsx:434-456` (stores `restoreFocusRef`, returns focus to trigger or previous element), `🗨️Popover/🟦️.tsx:359-372` (`onCloseAutoFocus`), `🔽️Select` (`restoreFocusRef`, `:336-339`) | `⚡️events/⏱️tooltip-clock-and-overlay-api/🦀️.rs:244-280` (`finish_close`): for non-Select overlays it calls `clear_focus`. No saved trigger, no restore (grep for `restore` in `⚡️events` finds nothing) | Focus is dropped to none on Dialog, Popover and ContextMenu close | High |
| Focus trap (Tab stays inside a modal) | `💬️Dialog/🟦️.tsx:146-209` (siblings `aria-hidden` and `inert` while open), `:492-495` (Escape) | `⚡️events/🖱️accessibility-and-pointer-dispatch/🦀️.rs:459-461` (Tab scoped to topmost `focus_trap` root). Sibling `inert` / `aria-hidden` not located. `focus_trap` set only for Dialog and CommandPalette (`⏱️tooltip-clock…/🦀️.rs:190`) | Tab trap present; background is not made inert (unverified) | Medium |
| Escape closes the topmost overlay | `RUI/🧱️elements/💬️Dialog/🟦️.tsx:492-495`, Popover `onEscapeKeyDown`, Select | `accessibility-and-pointer-dispatch/🦀️.rs:432-440` closes topmost via `close_topmost_overlay` for every kind (`escape_closes` is true for all kinds in the contract) | Matches for the shared path | Low |
| Outside press closes and is swallowed | `RUI/🧱️elements/💬️Dialog/🟦️.tsx` `onPointerDownOutside`, Popover | `⏱️tooltip-clock…/🦀️.rs:280-291` (`dismiss_topmost_if_outside_press`, `outside_press_swallow`) | Matches | Low |
| Tooltip dwell (400 ms) and hover-out (400 ms) | `RUI/🧱️elements/💡️ChromeControlHint/🟦️.tsx:21` (`CHROME_CONTROL_TOOLTIP_DELAY_MS = 400`), `:51`, `:163` | contract `overlay/🦀️.rs:218-226` (`TOOLTIP_DWELL_SECONDS = 0.4`, `TOOLTIP_HOVER_OUT_SECONDS = 0.4`), used at `⏱️tooltip-clock…/🦀️.rs:90`, `:172`, `:305` | Matches | Low |
| Tooltip revealed by keyboard focus or press | `💡️ChromeControlHint/🟦️.tsx:135` (reveals on keyboard focus, click, tap, Enter, Space) | `⏱️tooltip-clock…/🦀️.rs` reveals on hover only (focus appears only for editable text) | Keyboard-focus reveal missing (unverified beyond grep) | Medium |
| Tooltip surface | `💡️ChromeControlHint/🟦️.tsx` (`p-single text-xs`, glass, `shadow-lg`) | `ui-image-sources-skeleton-and-overlays/🦀️.rs:~399-407` (`paint_tooltip`, `p-single` padding, `paint_overlay_surface`) | Same geometry, same opaque-surface issue as above | Medium |
| ContextMenu: arrows, Enter, Escape, submenu open/close | `RUI/🧱️elements/🖱️ContextMenu/🧭️navigation/🟦️.tsx:105-109` (ArrowLeft/Right), `:162-168` (`contextMenuOpenSubmenuPath`) | `OS/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/⌨️keyboard-and-palette/🦀️.rs:185-230` (`context_menu_handle_key`: Escape, Up, Down, Left, Right), children model in `🕹️input/🔖️identity-and-types/🦀️.rs:26-33` | Keyboard model matches; visual row layout unverified | Low |
| Select popup: typeahead, Tab, arrows | `RUI/🧱️elements/🔽️Select/🟦️.tsx:452-460` | `RUI/🎯️targets/🧊️wgpu/⚡️events/⌨️widget-keyboard-and-commit/🦀️.rs:69-146` (`route_select_key`, typeahead `:147`) | Matches | Low |
| Dialog close control (X) | `💬️Dialog/🟦️.tsx:572` (`DialogClose`, top-right, `opacity-70` → `100` on hover) | not located | Unverified | Low (unverified) |
| Command palette and modal flags | contract `OverlayKind::CommandPalette` `has_backdrop` true | `⚙️engine/📣️dispatch-and-overlay-api/🦀️.rs:341` uses `overlay.kind.has_backdrop()`, so the backdrop follows the kind | Parity | Low |

## 6. Window layout nodes (splits, tabs, stacks, scrolling, clipping)

Contract: `RUI/🧬️contract/🖼️presentation/📐️layout/🦀️.rs` (`LayoutSpec` and `ScrollLayout`, `OverlayLayout`, `WindowLayoutNode` `:417-440`, `Axis`, `FlowInline`/`FlowBlock` `:~300-330`, `for_anchor` mirrors React `flowFromAnchor`). Declaration: `RUI/📜️declaration/📐️layout/🧭️layout-contract/🪟️window-layout-tree/🦀️.rs:21-177`. React model: `RUI/🔨️modules/🗺️window-layout/🟦️.ts:6-27`. React Mode and dock: `RUI/🧱️elements/🎨️Canvas/⚓️dock/🟦️.tsx:395` (`minSize={8}`), `RUI/🧱️elements/↔️Resizable/🟦️.tsx:19-22`. React Tabs: `RUI/🧱️elements/📑️Tabs/🟦️.tsx:167-176`. React Scrollable: `RUI/🧱️elements/📜️Scrollable/🟦️.tsx:~20-35`.

wgpu: dock `OS/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🧮️axis-solving-and-stack-rendering/🦀️.rs:100-102` (`SPLIT_VIS_PX = 6.0`, `SPLIT_HIT_MIN_PX = 20.0`), `…/🧱️dock-types-and-window-state/🦀️.rs:9` (`SPLIT_MIN_FRACTION = 0.08`), `…/🏷️tab-chrome-and-layout-edits/🦀️.rs:9-19` (tab max width 192 px; mobile 767 px, tablet 1023 px), shell tree build `OS/🎯️targets/🧊️wgpu/🐚️shell/🦀️.rs:198-225`, tabs `RUI/🧱️elements/📑️Tabs/🎯️targets/🧊️wgpu/🦀️.rs:77-110`.

| Aspect | React file:line | wgpu file:line | Gap | Severity |
|---|---|---|---|---|
| Node vocabulary and names | React `window-layout/🟦️.ts:6-27`: `kind` = `row`/`column` axis, `stack`, `window`; fields `id`, `activeId`, `size`, `corner`. | Declaration `window-layout-tree/🦀️.rs:32-75`: `kind` string, `window_kind_id`, `active_window_kind_id`, `size`, `corner`. Contract `📐️layout/🦀️.rs:417-440`: `Window`/`Stack`/`Split { axis }` with `windowKindId`, `activeWindowKindId`, `Axis::{Horizontal,Vertical}`. | Three shapes; contract unused | High |
| Consumers of the contract `WindowLayoutNode` | n/a | none (grep): only its file, `schema-rows-ui-contract-violation-to-window-stack-corner/🦀️.rs:242-258`, and a comment in `OS/…/🛰️dock/🦀️.rs:1` | Dead contract with a live schema projection | Medium |
| Splits: resize hit target | `↔️Resizable/🟦️.tsx:19` (fine 20 px), `:22` (coarse 28 px), `:24` (corner grab) | `axis-solving-and-stack-rendering/🦀️.rs:102` (`SPLIT_HIT_MIN_PX = 20.0`), no coarse variant | Touch splitter hit area is 20 px, React 28 px | Medium |
| Splits: visible divider thickness | React divider width not verified (`↔️Resizable/🟦️.tsx` hit size only) | `axis-solving…/🦀️.rs:100` (`SPLIT_VIS_PX = 6.0`) | Unverified | Low (unverified) |
| Splits: minimum size | `RUI/🧱️elements/🎨️Canvas/⚓️dock/🟦️.tsx:395` (`minSize={8}`) | `dock-types…/🦀️.rs:9` (`SPLIT_MIN_FRACTION = 0.08`) | Matches | Low |
| Splits: delta epsilon | React ignores deltas under 0.001 % (`Canvas/🟦️.tsx:594`, cited in wgpu comment) | `dock-types…/🦀️.rs:13` (`SPLIT_DELTA_EPSILON = 0.00001`) | Matches | Low |
| Splits: flow mirroring for dock anchors | contract `FlowInline`/`FlowBlock::for_anchor` (`📐️layout/🦀️.rs`, `flowFromAnchor` reference) | same contract consumed by wgpu dock | Matches in intent | Low |
| Stack tab chrome | React `PanelTabBar`/Tabs; `modeDockTabClassName` max width 12 × 16 px (wgpu cites it) | `tab-chrome…/🦀️.rs:9` (`MODE_DOCK_TAB_MAX_WIDTH_PX = 192.0`) | Matches on width cap; tab visuals unverified | Low |
| Breakpoints mobile / tablet | React `🧰️framework/🔨️modules/🖱️ui/📱️device/🟦️.ts` (cited) | `tab-chrome…/🦀️.rs:14-19` (767 / 1023) with a test that reads the file | Matches | Low |
| Tabs keyboard (arrows, Home/End, RTL, disabled skipped) | `RUI/🧱️elements/📑️Tabs/🟦️.tsx:167-176` | `RUI/🧱️elements/📑️Tabs/🎯️targets/🧊️wgpu/🦀️.rs` `tabs_step_keys` `:79-84` and `tabs_key` `:91-110` | Matches (incl. RTL and Home/End) | Low |
| Tabs activation (automatic / manual) | React `🟦️.tsx` `activationMode` (unverified) | `tabs_key` manual Enter/Space `:95-97` | Matches in code; visuals unverified | Low (unverified) |
| Flow of Tree (RTL) | `RUI/🧱️elements/🌳️Tree/🟦️.tsx:521` forces LTR | document tree mirrors with flow | React Tree never mirrors; wgpu mirrors | Medium |
| Scrolling: wheel, scrollbar drawing | `RUI/🧱️elements/📜️Scrollable/🟦️.tsx` uses native `overflow-*-auto` (native scrollbars) | Scroll region hits exist (`table-and-paint2d/🦀️.rs` `HitKind::ScrollRegion`, `scroll_offset`), but no scrollbar thumb painter was located (grep for `scrollbar` in `🖌️paint`, `🖍️draw`, `🧊️renderer` finds nothing) | Scrollbars not drawn on wgpu (unverified outside the grep scope) | High (unverified) |
| Clipping | React `overflow-hidden` on tree root (`🟦️.tsx:533`), Scrollable `overflow-*` | wgpu `push_scissor` around table body (`table-and-paint2d/🦀️.rs`), retained tree (`retained-tree-node-painting/🦀️.rs:27`) and list rows | Matches for tables and trees | Low |
| Window search Escape routing | React `RUI/🧱️elements/🪟️Window/🟦️.tsx:263` (`routeWindowSearchEscape`) | not located | Unverified | Low (unverified) |
| Window controls and header actions | React `🪟️Window/🟦️.tsx:353`, `:395` | OS dock chrome `RUI/🎯️…/🛰️dock/…` (not compared in depth) | Unverified | Low (unverified) |
| Splitter keyboard | React `↔️Resizable/🟦️.tsx` has no keyboard handler (grep) | none | Parity (both missing) | Low |

---

## Prioritised fix work items

Order is by user-visible impact and accessibility risk. Each item names the files to change and the React reference to copy from.

**P1 (High)**

1. **Overlay surface and scrim use the shared tokens and glass.** Change `RUI/🎯️targets/🧊️wgpu/🖌️paint/🖼️ui-image-sources-skeleton-and-overlays/🦀️.rs`: replace `OVERLAY_BACKDROP_ALPHA` and `theme.background` with `theme.veil(Level::Dialog)` plus `theme.veil_blur_px()`; in `paint_overlay_surface` use `theme.glass(Level::Menu)` and `theme.border_radius`. Also set a non-transparent default for `overlay_shadow` in `RUI/🎨️styling/🎭️theme/🧩️component/🦀️.rs:286`. Reference: `RUI/🎨️styling/🖌️ui/🎨️.css:7182-7192` (`ui-glass`, `ui-veil`), `RUI/🎨️styling/🔣️.json:457-458` (`veilAlpha`), `RUI/🧱️elements/💬️Dialog/🟦️.tsx:417`, `🗨️Popover/🟦️.tsx:442`.

2. **Restore focus on overlay close.** Add a saved focus owner to `OpenOverlay` (struct at `RUI/🎯️targets/🧊️wgpu/⚡️events/🧲️overlays-drag-and-commands/🦀️.rs:30-40`), capture it in the open path (`⚡️events/⏱️tooltip-clock-and-overlay-api/🦀️.rs:~185-192`), and in `finish_close` (`⏱️…/🦀️.rs:244-280`) restore it instead of `clear_focus` for Dialog, Popover and ContextMenu. Reference: `RUI/🧱️elements/💬️Dialog/🟦️.tsx:434-456` (`restoreFocusRef`, trigger-first), `🗨️Popover/🟦️.tsx:359-372`.

3. **Tree keyboard model.** Add treeitem handling for ArrowUp/ArrowDown, Home/End, ArrowLeft/ArrowRight (expand/collapse and parent jump), Enter, and roving focus. Either in the engine (`RUI/🎯️targets/🧊️wgpu/⚡️events/⌨️widget-keyboard-and-commit/🦀️.rs`, with a row-order walk over `retained-tree`) or in the DOM mirror (`OS/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts`, extending `accessibilityMirrorEngineKeys` at `:46-54` and the keydown block at `:170`). Reference: `RUI/🧱️elements/🌳️Tree/⌨️roving/🟦️.ts:7-27` and `:52-68`; `RUI/🧱️elements/🌳️Tree/🟦️.tsx:502-516`.

4. **Document Table column layout.** Replace the equal split in `RUI/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:383-389` (`table_column_rect`) with fixed per-column width and a horizontal scroll region, and change the actions width in `:372-378` to `size-medium` per action. Reference: `OS/🧱️elements/🗣️Interpreter/📊️table-window/🟦️.ts:62-65`. Update `RUI/🎯️targets/🧊️wgpu/🖌️paint/📝️retained-text-steps/🦀️.rs:279-345` to read the widths.

5. **Document Table keyboard.** Add Up/Down, PageUp/PageDown, Home/End, Enter/Space, Left/Right across actions and cells, and Escape back to the row. Put it next to the existing focus helpers in `RUI/🎯️targets/🧊️wgpu/⚡️events/✍️drag-scroll-and-edit-api/` or in the mirror at `OS/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts`. Reference: `OS/🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx:315-317` and `:318-414` (`TableView`), `RUI/🧱️elements/📊️Table/🟦️.tsx:48-51` (`rowActivationKeyV1`).

6. **Extension node painting.** Add a `Component::Extension` arm in the retained painter (`RUI/🎯️targets/🧊️wgpu/🖌️paint/🎨️node-painting-with-driver/` or `retained-tree-node-painting`) that hands off to the dynamic-extension host (`OS/🎯️targets/🧊️wgpu/🧩️dynamic-extension/🟦️.ts:41`). Reference: `OS/🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx:582-583`, `🧩️host-contract/🟦️.ts:9`. First confirm what the painter does with an unknown component (unverified).

7. **Tree section header chevron and label style.** In `RUI/🎯️targets/🧊️wgpu/🖌️paint/🌲️retained-tree-node-painting/🦀️.rs:57-84` draw the fold chevron in the gutter (same mirror logic as items at `:143-160`) and render the label uppercase with semibold weight if the font stack supports it. Reference: `RUI/🧱️elements/🌳️Tree/🌱️section/🟦️.tsx:198,231` and `RUI/🧱️elements/🌳️Tree/🎨️chrome/🟦️.tsx:41-42`.

8. **Tree indent guides on the document path.** Port `tree_draw_guides` (currently in `RUI/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs`) into `retained-tree-node-painting/🦀️.rs` for each row, with the elbow and stem rules. Reference: `RUI/🧱️elements/🌳️Tree/📏️row-alignment/🟦️.tsx:58-130` (`TreeDocumentGutter`) and `🎨️chrome/🟦️.tsx:63-77` (`IndentationLines`).

9. **Scrollbar drawing (verify first).** Confirm whether scrollbars are drawn anywhere on the wgpu path. If not, add a thumb painter to the scroll region used by the table body (`table-and-paint2d/🦀️.rs`) and by Scrollable hosts. Reference: `RUI/🧱️elements/📜️Scrollable/🟦️.tsx` (native overflow model) and the native scrollbar token set in `🎨️styling/`.

10. **Selection modifiers and double-click on tree and table rows.** Add Shift-range, Ctrl/Meta-additive selection with an anchor, and row double-click dispatch in the document path and the surface table (`table_hit` already re-derives modifiers, `table-and-paint2d/🦀️.rs:~693-716`). Reference: `RUI/🧱️elements/🌳️Tree/🟦️.tsx:210-237`, `RUI/🧱️elements/🌳️Tree/🎚️selection/🟦️.ts`, `RUI/🧱️elements/📊️Table/🟦️.tsx:112`.

11. **Extension and missing-payload fallbacks.** Replace `render_placeholder`'s `"{kind} host"` text (`OS/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🖱️canvas-gestures-input-kit-and-chrome/🦀️.rs:194-197`) with a localised label or nothing, and match React's fallback once it is confirmed (unverified).

**P2 (Medium)**

12. **Pick one window-layout model.** Either make `RUI/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:417-440` (`WindowLayoutNode`) the source and project the declaration `WindowLayout` from it, or delete the unused contract enum and keep `RUI/📜️declaration/📐️layout/🧭️layout-contract/🪟️window-layout-tree/🦀️.rs`. Align the React type `RUI/🔨️modules/🗺️window-layout/🟦️.ts:6-27` (`row`/`column`, `id`, `activeId`) with whichever wins. Update `RUI/🧬️contract/🧬️schema/🟦️projection/🧮️schema-rows-ui-contract-violation-to-window-stack-corner/🦀️.rs:242-258` and the dock comment `OS/…/🛰️dock/🦀️.rs:1`.

13. **Coarse-pointer splitter hit target.** Add a 28 px coarse hit in `OS/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🧮️axis-solving-and-stack-rendering/🦀️.rs:102` next to `SPLIT_HIT_MIN_PX`, chosen by the touch input path. Reference: `RUI/🧱️elements/↔️Resizable/🟦️.tsx:19-22`.

14. **Tree inline direction.** Decide whether the document Tree should mirror (wgpu now does) or stay LTR (React `RUI/🧱️elements/🌳️Tree/🟦️.tsx:521`). Then either remove the flow argument in `retained-tree-node-painting/🦀️.rs:23` or change the React root. This needs a product decision.

15. **Overlay tooltips on keyboard focus.** Reveal tooltips on keyboard focus and press in `RUI/🎯️targets/🧊️wgpu/⚡️events/⏱️tooltip-clock-and-overlay-api/🦀️.rs`. Reference: `RUI/🧱️elements/💡️ChromeControlHint/🟦️.tsx:135`, `:163`.

16. **Overlay corner radius and shadow.** In `paint_overlay_surface` use `theme.border_radius` and draw a visible shadow from the theme (see item 1). Reference: `RUI/🧱️elements/🔽️Select/🟦️.tsx` (`rounded-sm`) and `💡️ChromeControlHint/🟦️.tsx` (`shadow-lg`).

17. **Table header semantics and typography.** Add `aria-sort` through the existing accessibility staging for table headers (`table-and-paint2d/🦀️.rs`, `stage_table_button_accessibility_cells`). Change header font weight and height to match the `font-medium` `h-large` template. Reference: `RUI/🧱️elements/📊️Table/🟦️.tsx:53-57`, `:565`.

18. **Table empty state, range label and row-action trigger.** Replace the literal `"No rows"` (`table-and-paint2d/🦀️.rs:770`) with a localised label (`emptyMessage`, `noDataLabel`) and draw the per-row trigger button in the last cell when a context menu is bound. Reference: `RUI/🧱️elements/📊️Table/🟦️.tsx:117`, `:345`, `:236`, `:302`, `OS/🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx:344`.

19. **Table drag activation delay.** Add the `pointerActivationDelayMs`/`pointerActivationTolerancePx` behaviour from the React drag config to the table drag path. Reference: `RUI/🧱️elements/📊️Table/🟦️.tsx:89-96`.

20. **Tree roving tabindex and drop-preview.** Add a roving tab stop for tree rows and a reorder drop indicator (`RUI/🧱️elements/🌳️Tree/🧲️drag-drop/🟦️.tsx`, `TreeReorderDropPreview`) on the document path. Reference: `RUI/🧱️elements/🌳️Tree/🟦️.tsx:180-208`.

**P3 (Low)**

21. Dedupe the tree constants: keep one `TREE_INDENT_PER_LEVEL`/`TREE_TOGGLE_WIDTH`/`TREE_ICON_SIZE` owner (currently `RUI/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:21-23` and `🖌️paint/🔤️retained-glyph-and-node-cursors/🦀️.rs:6-10`).
22. Make the widget-path `render_tree_item` honour `TreePresentation` (`RUI/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs:167`), or retire the widget path if nothing uses it (unverified).
23. Dialog open and close transition (React `zoom-in-95` / `fade-in-0`, `💬️Dialog/🟦️.tsx:417`, `:554`) if the design requires it.
24. Table empty-state and drop-target fills: `RUI/🧱️elements/📊️Table/🟦️.tsx:267`.

## Unverified items (not confirmed in this audit)

- Scrollbar painting on wgpu (grep found no thumb painter in `🖌️paint`, `🖍️draw`, `🧊️renderer`; not traced through the scroll engine).
- Whether an unknown document component paints nothing or a fallback (affects item 6).
- Tree detail band painting, inline toolbar paint for tree rows, and row-action menu placement.
- Exact React tree and table token values against wgpu theme values (`text-xs` vs `font_size_body`, `h-medium` vs `control_height`, `h-large`).
- Source of `TABLE_COLUMN_WINDOW_WIDTH_PX` (imported from `🪟️tree-windows/🟦️.ts`, export not found by grep). Its numeric value is therefore not stated here.
- Window search Escape routing, window header controls and the dock resize visual thickness.
- Tabs activation-mode visuals and the Dialog close control.
- Whether shell chrome overlays (glass) are meant to differ from the shared overlay path; the audit treats the difference as a gap.
