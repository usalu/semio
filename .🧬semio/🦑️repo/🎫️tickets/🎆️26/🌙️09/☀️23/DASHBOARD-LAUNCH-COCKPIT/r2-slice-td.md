# R2 Slice T-D: Text And Lists

## 00. Round 3 (newest): every width follows the active width mode

Problem: `Tui::new` defaults to `WidthMode::Scalar`, but `display_width`, truncation, ellipsis, caret columns and hit tests counted clusters, so the hardware cursor and cuts were off for text such as `🖥️platform` (10 cells as a cluster, 9 as scalars).

Design (`⌨️tui/📝️text`, `⌨️tui/🔲️cell`):
- Explicit, pure variants take the mode: `display_width_in`, `cluster_cells_in`, `truncate_to_in`, `elide_in`, `window_cells_in`, `boundary_at_cell_in`. In `Scalar` mode the counted unit is one scalar (sum of `char_cells`); in `Cluster` mode it is one grapheme cluster. Cuts never split a unit of the mode.
- The names every other module already calls (`display_width`, `cluster_cells`, `truncate_to`, `elide`, `elide_end/start/middle`, `window_cells`, `boundary_at_cell`) now use the thread's active mode, so chrome, dialog, menu, palette, tabs, footer, navbar and all my elements follow it without edits.
- `text::active_width_mode()` / `set_active_width_mode()` hold the active mode in a thread local (default `Cluster`). `CellBuffer::set_width_mode` sets it, so `Tui::new` / `Tui::set_width_mode` (which configure `front` and `back`) switch the whole thread; painting, cursor column and pointer hit tests then count exactly as the buffer paints. Thread local keeps parallel tests and multiple hosts independent.
- Caret and Backspace movement stay cluster-wise in both modes (editing semantics); only columns are mode-aware.

Tests (both modes, repo emoji paths, oracle kept):
- `📏️text-width`: scalar widths of all 90 fixture cases equal the `unicode-width` per-char sum, cluster widths still equal the string oracle; `🖥️platform` is 10/9 cells, the full repo path 38/36; cuts, ellipsis and windows never exceed the budget in either mode for every fixture text and budget; a scalar buffer paints exactly the predicted cells for all repo segments; active-mode switching via the buffer.
- `🔬️text-elements`: input cursor column and click mapping, filter cursor of Tree and Wizard, and List/Wizard/Tree elision of repo paths stay inside the rect in both modes.
- Result: `cargo test -p semio-framework-ui --features tui-terminal --lib tui::` 365 passed, 0 failed (with `CARGO_BUILD_BUILD_DIR=<target>/private-build`); `cargo check -p semio-framework-repo-dashboard` fails only on a peer change (`Key::Keypad` not covered in `dashboard/terminal/controls` line 75), not on this work.


## 0. Round 2b (newest): per-row status on `ListState`

API in `🧱️elements/📃️List/🎯️targets/⌨️tui/🦀️.rs`:
- `ListState::statuses: Vec<Option<theme::Status>>`, a public vector parallel to `items` and `marks` (rows past its end have no status). `ListState::set_status(position, Option<Status>)` grows it as needed.
- Painting: a row with a status draws `Status::glyph(theme.glyphs, frame)` in `theme.role(status.role())` right after the mark column (a focused selected row keeps the active foreground so it stays legible). As soon as any row has a status every row reserves the two-cell column, so labels stay aligned; a list without statuses keeps the old layout.
- Animation: `WidgetState::tick(now_ms)` on a `List` returns true only when the list holds a `Status::Running` row and the 125 ms frame (same `SPIN_FRAME_MS` as the engine) changed; `Tui::tick` already marks such a node paint-dirty. Idle lists never request repaints.
- Test: `list_rows_show_their_status_glyph_in_the_role_colour_and_the_spinner_ticks` in `⌨️tui/🧪️tests/🔬️text-elements`.

Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT`, executor slice T-D. Scope: `fleet-plan.md` §4.2 T-D (D11 D12 D21 D28 D29 D33 D34, R05 R10 R11 R18 R20 R21 R22 R24).

## 1. Verification (all run in this session)

| Command | Result |
| --- | --- |
| `cargo check -p semio-framework-ui --features tui-terminal` | exit 0 |
| `cargo check -p semio-framework-ui --features tui` (wasm-safe set) | exit 0 |
| `cargo check -p semio-framework-repo-dashboard` | exit 0, no change needed in the dashboard |
| `cargo test -p semio-framework-ui --features tui-terminal --lib tui::` | 348 passed, 0 failed (includes the peers' suites) |
| Isolated harness (text, cell, theme, rows only; scratchpad crate) in `--release` | `keystrokes_stay_fast_at_fifty_thousand_rows` passes under 5 ms |

Not run (UNVERIFIED): a `--release` build of the whole `semio-framework-ui` test target (fleet rule: no release builds); any real-terminal rendering of clusters; Bun/Node oracle adapter for the width fixture (only the Rust oracles exist).
The element-level 50k tests (`wizard_filters_fifty_thousand_options_keystroke_by_keystroke`, `tree_filters_fifty_thousand_nodes_quickly`) assert 200 ms / 80 ms per keystroke including a repaint in the dev profile and 5 ms / 8 ms in release; only the dev profile ran.

## 2. Defects

| Id | State | Where |
| --- | --- | --- |
| D11 | fixed | `📜️rows` `FilterIndex` (lowercase keys once, token AND, 32-entry history so extending and backspacing reuse the previous answer, `scanned()` counter) + `ListModel` (key-stable selection, `set_labels` keeps the selected label). Tests count scanned rows and time keystrokes at 50 000 rows. |
| D12 | fixed | `Rows` persistent viewport that follows the selection and survives wheel scrolling; List, Wizard, Tree, Table: press selects (`SelectionChanged`), double press or Enter activates, right press `ContextMenu{item}`, wheel, hover row (`set_hover`), PgUp/PgDn/Home/End, scroll bar. Log, Scrollable, Select have PgUp/PgDn/Home/End too. |
| D21 | fixed | Input cursor is always on a grapheme boundary; Ctrl/Alt characters do not insert; Home/End/Delete/Ctrl+A/E/U/K/W/word moves; a mid-character cursor is repaired. |
| D28 | fixed | Table re-derives the selected row from the visible rows on every key, mouse and paint. |
| D29 | fixed | `LogScroll::At(n)` now means first visible line; Home shows a full page; first PageUp from Follow moves a whole page; wheel. |
| D33 | fixed | Backspace on an empty wizard filter does nothing; navigating back is Esc on an empty filter or Alt+Backspace / Alt+Left. |
| D34 | fixed | Every list signal (`SelectionChanged`, `Activated`, `ContextMenu.item`) carries the option or item index in the unfiltered data, never a screen position. |
| R05 | fixed in the model | Extended grapheme clusters (UAX #29 incl. GB9c and GB11), widths per cluster (EAW, VS15/VS16, emoji modifier and ZWJ sequences, flags, keycaps, Hangul, Indic), Unicode 17 tables. `CellBuffer` stores whole clusters. `WidthMode::Scalar` is the wcwidth view for terminals without clustering; nobody selects it yet (see §5). |
| R10 | fixed | `text::{elide, elide_end, elide_start, elide_middle, Elision, ELLIPSIS}`; all my painters use them. The registry still has its own `elide_*`. |
| R11 | fixed in theme | `Role::{Success,Warning,Danger,Info}` (contrast 4.5:1 on base, window, pane and panel in both appearances), `Status`, `Status::glyph(set, tick)` with a four-frame spinner. Adoption by tabs and the Tasks list is the dashboard's and chrome's step. |
| R18 | fixed | `CellBuffer::put` blanks orphaned halves on both sides, wide lead at the last column becomes a blank, `diff` compares cluster tails and returns per-row runs on a size change. |
| R20 | partly | No framework painter owns an English string any more: `empty` fields on List, Wizard, Table, Tree (default empty, the application supplies the text). Dashboard literals remain A-3's. |
| R21 | fixed | Grapheme cursor, horizontal scroll that keeps the caret visible, `cursor()` for Input, Wizard filter row and Tree filter row. No painted caret (coordinator request); the painter never hides a glyph. |
| R22 | fixed | Persistent offset; unfocused selection is bold, focused selection uses the active fill, hover uses `HoverInteractive`. |
| R24 | partly | `Theme::glyphs: GlyphSet {Unicode, Ascii}`, `Glyph` repertoire (close, maximize, new tab, check, cross, fault, ellipsis, markers, bar, scroll, toggle), every glyph one cell. My painters use it; the window chrome (T-B) still draws its own `✕ ⤢ ⧉` and should read `theme.glyphs`. |

## 3. API (new or changed)

Core (`⌨️tui`):
- `text`: `clusters(&str)`, `cluster_cells`, `display_width`, `truncate_to`, `elide*`, `window_cells(s, start, width) -> (&str, lead_blanks)`, `boundary_at_or_before`, `previous_boundary`, `next_boundary`, `boundary_at_cell`, `WidthMode`. `char_cells` is unchanged for the VT.
- `cell`: `CellBuffer::{put_cluster, push_glyph(x,y,&mut String), tail_id, row_text(y), restyle(rect, f), width_mode, set_width_mode}`. `Cell` literal is unchanged (cluster tails live beside the cells, interned, capped at 65 536 distinct tails). `attr` bits unchanged (0x40 and 0x80 stay free for the VT).
- `theme`: `Role::{Success,Warning,Danger,Info}`, `Status`, `Glyph`, `GlyphSet`, `Theme::{glyphs, set_glyphs}`, `luminance`, `contrast`.
- `rows` (new module `📜️rows`): `FilterIndex`, `ListModel`, `Rows`, `Pointer`, `follow_top`, `navigate_to`, `row_under(_with)`, `row_style`, `paint_scroll_bar`.

Elements (state types are defined in the element files and re-exported by `widget`):
- `ListState` keeps `items, selected, offset, marks` public; adds `empty`, `select(pos)`, `hover()`.
- `WizardState { steps, list: ListModel, empty }` with `options()`, `set_options()`, `filter()`, `selected_option()`. Signals carry option indices.
- `TableState` adds `empty`; `LogState` keeps its API; `InputState` keeps its three public fields and gains `new`, `set_value`; `SelectState` unchanged.
- NEW `TreeState` / `TreeItem { id, label, depth }` (flat pre-order forest), the launcher element for A-3:
  - `TreeState::new(items)`, `expand_to_depth(d)`, `set_expanded(item, bool)`, `select_item(item)` (opens ancestors), `set_query(q)` (matches the whole path of a node, keeps ancestors, selects the first real match; clearing keeps the selected item revealed), `query()`, `count()`, `row_item(pos)`, `position_of(item)`, `selected_item()`, `item(i)`, `is_folder`, `is_expanded`, `parent_of`, fields `view: Rows`, `empty`, `filterable`.
  - Keys: arrows, PgUp/PgDn, Home/End, Right (open, then enter), Left (close, then parent), Enter (folder toggles, leaf activates), printable keys edit the filter, Backspace, Esc and Ctrl+U clear it. Signals: `Toggled(open)`, `Activated(item)`, `SelectionChanged(item)`, `ValueChanged(query)`, `ContextMenu{item}`.
  - Mouse: press on the marker toggles, press elsewhere selects, double press toggles a folder or activates a leaf, right press context menu, wheel, hover.
  - `cursor()` sits on the filter row. 50 000 nodes filter per keystroke within the test budget.
- NEW `ScrollableState` (`new(lines)`, `set_lines`, pub `top`, `left`; keys, wheel, scroll bar click, horizontal), `ProgressState` (`determinate(label, f32)`, `indeterminate(label)`, `set_value`, ticks through `WidgetState::tick`, not focusable), `ToggleState` (`new(label, on)`; Space, Enter, click; `Toggled(bool)`). New `WidgetState` variants `Tree`, `Scrollable`, `Progress`, `Toggle`.

## 4. Files

Created: `⌨️tui/📜️rows/🦀️.rs`, `⌨️tui/📝️text/🔤️tables/🦀️.rs` (generated Unicode 17 data), elements `🌳️Tree`, `📜️Scrollable`, `📶️Progress`, `🔀️Toggle` under `🧱️elements/<name>/🎯️targets/⌨️tui/🦀️.rs`; tests `⌨️tui/🧪️tests/{📏️text-width,📜️rows,🚦️status-roles,🔬️text-elements}` (each with `🥒️.feature`, Rust adapter), fixtures `⌨️tui/🧫️fixtures/{📏️text-width,📜️rows}/🔣️.json`.
Rewritten: `📝️text`, `🔲️cell`, `🎨️theme`, elements List, Wizard, Table, Log, Input, Select.
Edited outside my slice, minimal and necessary: `⌨️tui/🪀️widget/🦀️.rs` (state structs of my elements moved out and re-exported; new variants and dispatch arms; T-B's inline paste and cursor code for Input/Wizard replaced by my element hooks), `⌨️tui/🔡️ansi/🦀️.rs` (one line: `emit_runs` writes `next.push_glyph(x, y, ..)` so clusters keep their tails), `⌨️tui/🦀️.rs` (modules `rows`, test module), `🎯️targets/⌨️tui/🦀️.rs` (four element modules), `📦️packages/🦀️rust/Cargo.toml` (dev-dependencies `unicode-width 0.2.2`, `unicode-segmentation 1.13.2`; `Cargo.lock` gains the two edges), `⌨️tui/🧪️tests/🔬️unit/🦀️.rs` (tests that asserted the defects: Wizard, Input, Log, cell orphan, diff, goldens, list literal).
Oracles: `unicode-width` and `unicode-segmentation` (dev only) agree with the model on every scalar's width, on segmentation of every scalar in 4 to 14 contexts, on a 38 000 sequence sweep and on the 90 case fixture (repo emoji path segments included). One documented deviation: a skin tone modifier that starts a ZWJ sequence (UAX #29 breaks before the pictograph; the string oracle joins).
Ticket helpers: `r2-td-unicode-tables.py`, `r2-td-unicode-width-probe.rs`, `r2-td-width-fixture-probe.rs`, `r2-td-fixture-segments.py` (how the table module and fixture were produced).

## 5. Required follow-ups for other slices

1. Capability wiring (T-A/T-B/host): call `buffer.set_width_mode(WidthMode::Scalar)` (on `back` and `front`) when `Capabilities::unicode` is below full or the terminal fails the cluster probe; call `theme.set_glyphs(GlyphSet::Ascii)` from the `ui.glyphs` preference.
2. Chrome (T-B): read `theme.glyphs` for `✕ ⤢ ⧉` and tab status (`Status::glyph`, `Status::role`); `Role::Danger` etc. now exist.
3. Registry (A-1): replace its private `elide_middle/elide_end/take_cells` (char-wise) by `ui_tui::tui::text::{elide_middle, elide_end}` (cluster-wise, same budget semantics: tail = min(6, budget - 2)).
4. Dashboard (A-3): launcher should use `TreeState` (`TreeItem::new(id, label, depth)`; `Toggled/Activated` carry item indices), set `empty` texts from its locale catalogue, and drive spinners with `WidgetState::tick`. `ListState` and `InputState` field use keeps compiling.
5. Engine (T-B): a wheel over a list/log/scrollable changes widget state and returns `None`; repaint the hit node after any `on_mouse`. `Progress` needs `Tui::tick` to forward to `WidgetState::tick` with a clock.
6. Real terminals: UNVERIFIED how Windows Terminal, conhost and iTerm2 draw VS16 and non-fully-qualified ZWJ names such as `🧑️‍💻️contributor`; the model follows Unicode and the oracle (such a name counts 4 cells), so check on target terminals and use `WidthMode::Scalar` where they disagree.
