# R2 Slice T-B: Engine And Windows

## Round 3 (newest): stale cells in the real dashboard (V-1 F5)

| Finding | Evidence |
| --- | --- |
| Root cause of the misaligned/stale-cell class: the engine measured text in `WidthMode::Cluster` (T-D's default) while every emulator V-1 used (vt100, pyte) and most terminals count scalars. Labels with VS16 emoji (`⌨️`, `🧰️`, all the taxonomy names in the launcher and window titles), ZWJ sequences and wide text then occupy a different number of cells in the engine than on the screen, so every later cell of the row lands in the wrong column and incremental patches leave fragments. | The property test extended with wide/emoji/ZWJ/combining text (navbar, footer, window titles, list rows, text nodes), tab status changes (`Shell::set_status`), overlays, zoom/remount, resize and ticks fails at step 0 in Cluster mode against the VT emulator (`cell (24,3) incremental '│' vs full ' '`) and passes 64 seeds x 80 steps in Scalar mode. |
| Fix | `Tui::new` now starts in `WidthMode::Scalar` (both buffers); `Tui::set_width_mode(Cluster)` is the explicit opt-in for a terminal that was probed to render clusters (T-A capability probe / the view). Test `the_engine_measures_scalars_by_default_so_emulators_that_count_scalars_stay_aligned`. Note for T-D: `text::display_width` is cluster-based, so hardware-cursor x for text with emoji can be off by the difference in Scalar mode. |
| Operations the property already covered (and still passes) | overlay open/close, zoom/restore, remount, tab strip changes, resize, appearance, tooltips, ticks; they were not the cause. Wide clusters and status changes are new. |
| Journey re-run | Built `semio` into `target-fleet-tb`, ran `the_incremental_screen_equals_a_full_repaint` through `r2-tb-jt.sh` (copy of `r2-v1-jt.sh` with my binary). Old binary: row 3 spinner glyph differs (animation between the two reads) and row 6 `workspace dashboard daemon ready at pid N`. New binary: only row 6. |
| Remaining difference is not the engine | Row 6 text is absent in the repainted screen and in the engine frame; it is a write that bypasses the engine: the detached daemon announces itself on the console shared with the view (F3, A-2: redirect the daemon's stdio to null/log when spawning detached, or never inherit the console). The engine cannot know about foreign writes; a resize forces the full repaint that clears it. |

Verification: `cargo test -p semio-framework-ui --features tui-terminal --lib -- tui::` = 357 passed, 0 failed; `cargo build -p semio-framework-repo-dashboard --bin semio` builds.

---

## Round 2b (newest)

API recorded for the view (no message sent to A-3):

| Item | API / behaviour |
| --- | --- |
| Glyph set (R24) | Window chrome draws close, maximize and the status glyph from `theme.glyphs` (`Glyph::Close`, `Glyph::Maximize`, restore `⤡`/`v`); layout stays theme-less because every glyph is one cell. The `+` new-tab control is ASCII in both sets. Tests: `controls_and_status_are_drawn_from_the_glyph_set_and_keep_their_columns`. |
| Tab status | `WindowStackTabState { label, corner, status: Option<theme::Status> }` with `with_status`; tab chip shows `Status::glyph(theme.glyphs, spin)` in `Status::role()` colour (active chip keeps `ActiveForeground`). `WindowState::{status, tab_ids, spin}`. `Shell::set_status(scene, window_id, Option<Status>)` sets the window's own status and mirrors it into every tab strip that lists it; remounts keep statuses. |
| Spinner | `Tui::tick(now_ms)` advances `WindowState.spin` (125 ms frames) for windows with a `Running` tab and ticks widgets (`Progress`); `Tui::deadline_ms()` includes the next animation frame while anything animates. Tests `the_engine_tick_animates_progress_widgets_and_running_tabs`, `shell_set_status_mirrors_into_every_tab_strip_of_the_stack`. |
| Repaint after mouse | Every routed `on_mouse` (all kinds) marks the node for repaint, so a wheel that returns `None` still repaints. Test `a_wheel_that_changes_widget_state_without_a_signal_still_repaints`. |
| Width mode | `Tui::set_width_mode(WidthMode)` / `width_mode()`: sets it on front and back buffers (kept across resize), marks layout stale and forces a full repaint. |

Verification: `cargo test -p semio-framework-ui --features tui-terminal --lib -- tui::` = 353 passed, 1 failed (`backend::native_windows::...an_unsignalled_wait_times_out...`, T-A's Windows input timing test; the earlier T-D failures are fixed). `cargo check -p semio-framework-ui --features tui-terminal`: clean, no TUI warnings. `cargo check -p semio-framework-repo-dashboard`: one error, `registry/🦀️.rs:1449 no method launch_json` (A-1's registry work, not TUI). Cargo note: the shared artifact-dir lock blocks for many minutes while other slices build; `CARGO_BUILD_BUILD_DIR=<target dir>/private-build` avoids it.

---

Executor T-B, 2026-10-08. TUI = `🧰️framework/🔨️modules/🖱️ui/⌨️tui`, EL = `🧰️framework/🔨️modules/🖱️ui/🧱️elements`.

## 1. Changes

| Area | Files | Change |
| --- | --- | --- |
| Dirty fix | `TUI/🎬️scene` | `mark_dirty` invariant restored: `clear_dirty` clears every node after layout/paint (old code cleared only the root, so later mutations stopped at stale flags). New `contains`, `try_node`, `lineage`, `shown`, `dirty_flags`, `hittable`, `tooltip`, `AxisState` content, second overlay root (hit first, painted last). `NodeMut::widget/chrome` mark layout and paint. |
| Engine | `TUI/⚙️engine` | Rewritten. Incremental `render()` (back buffer rebuilt per paint, patch = diff against front, front/back swapped), `dirty`, `render_due(now_ms)` (16 ms budget), `deadline_ms`, `dispatch_at`, fresh layout before every mouse hit test (D37), engine focus tree (`focus`, `set_focus`, `focus_ring`, `focus_next/prev`, `focus_window`, `window_of`, `focused_window`), `cursor` from the focused widget, `tick`, hover, `capture`, press state, click counts, wheel, right click, splitter and tab drags, overlays, `on_focus` notification, paste routing. |
| Widget plumbing | `TUI/🪀️widget` (my regions only) | `WidgetSignal`: `WindowMaximize(usize)`, `WindowNewTab(usize)`, new `Dismissed`. `TabsState::new` and `hover`. New variants `Menu Dialog Palette Tooltip`. New methods `is_overlay`, `overlay_size`, `claims_tab`. `on_mouse/set_hover/cursor/interactive` dispatch arms. Input and Palette `on_paste`. Label, Divider, Chip, Tooltip are no longer focusable. |
| Chrome | `TUI/🖥️chrome`, `EL/🪟️Window` | Window anatomy rewritten (section 3). `WindowHit`, `ChromeLabels`, `ChromeState::window_target/set_hover/tooltip/window_drop_index/set_drop_target`. `shell()`/`mount_window_layout` use layout titles, one gutter cell between stacks, `AxisState` nodes with layout paths, zoom hides every other window, `peers`/`zoomed` flags. |
| Layout | `TUI/📏️layout` | One solver (`axis_cells` over `distribute`, shared with the mount tree), `WINDOW_GAP`, `SPLIT_MIN_FRACTION`, `weight_of`, `WindowMeasure.title`, `stack_hosting`, `set_window_title`, `reorder_stack_tab`, `resize_split`. |
| Elements | `EL/{Tabs,Navbar,Footer,Label}` | Tabs: filled active tab, hover, click, double click, wheel. Navbar: free-band placement (R23). Footer: status keeps a third of the row, cut hints end in an ellipsis (R23). Label: ellipsis via `text::elide`. Chip, Divider: unchanged (nothing to fix). |
| New overlay elements | `EL/💬️Dialog`, `EL/🖱️ContextMenu`, `EL/⌨️Command`, `EL/💡️ChromeControlHint` (`🎯️targets/⌨️tui/🦀️.rs`), registered in `🎯️targets/⌨️tui/🦀️.rs` as `dialog menu palette tooltip` | Dialog (modal, wrapped body, buttons), Menu (separators, disabled rows, shortcuts, scrolling), Palette (incremental all-words filter, caret, paste, wheel), Tooltip (400 ms, `ChromeControlHint` delay). Surfaces `Menu`/`Dialog`, `BorderEmphasized`, `HoverInteractive` are now used. All strings come from the app. |
| Tests | `TUI/🧪️tests/{🔬️render-equivalence,🔬️engine,🔬️chrome,🔬️overlays,🔬️pointer-routing}`, `TUI/🧪️tests/🖱️pointer-routing/{🥒️.feature,🟦️.ts}`, `TUI/🧪️tests/🎞️render-equivalence/🥒️.feature`, `TUI/🧫️fixtures/🖱️pointer-routing/🔣️.json`; manifest `TUI/🦀️.rs` registers the five Rust modules. Existing chrome/engine tests in `🔬️unit` updated to the new behaviour (heavy line set, controls chip, gutter, `TabsState::new`, non-interactive labels, taller terminals). |

### Contract extension (recorded)

`WidgetSignal::WindowMaximize(usize)` and `WidgetSignal::WindowNewTab(usize)` now carry the stack tab index (the active tab of the emitting window); `WindowClose(usize)` carries the index of the pressed chip, not the active one. `WidgetSignal::Dismissed` was added. Chrome signals come from the window chrome node; `WindowFocus` is emitted from the window node when focus moves into it. All other §4.1 signatures are unchanged.

## 2. Ids

| Id | Status | Evidence |
| --- | --- | --- |
| D07 mouse | DONE (engine part) | Down/Up/Drag/Move/Scroll routed (`⚙️engine`); hover with `set_hover`; implicit capture on press plus explicit `capture`; click counts 1/2/3 (500 ms, same cell, needs `tick` or `dispatch_at` clock); wheel to widget under pointer, never moves focus; right click `ContextMenu{pos,item}` (widget item or `None`); phantom Drag without press is a Move. Tests `engine_behaviour::*`, fixture cases. `?1003h`, `clicks` from the terminal, selection: T-A/T-C. |
| D08 dirty | DONE | `render_equivalence`: 64 seeds x 80 random mutations (scene, widget, chrome, reparent, focus, pointer, keys, resize, appearance, overlays, zoom, tick) must give identical VT screens and identical engine frames for incremental `render()` and `render_full()`, and identical signals. Mutation proof: clearing only the roots makes `incremental_render_patches_match_full_render_on_random_mutations` and `mutating_a_node_after_the_first_frame_yields_a_non_empty_patch` fail (run once by hand, reverted, diff against backup identical). |
| D09 | DONE | `WindowClose(i)` of the pressed chip, only `Down(Left)` fires, right press on a tab = `ContextMenu(.., Some(i))`, controls do not move focus. Fixture cases, `every_chrome_control_carries_its_own_tab_index...`. |
| D10 | DONE (engine side) | Engine is the single focus source; click focuses the containing focusable (window body focuses its first widget); `WindowFocus` signal; `focused` chrome flag synced by the engine. A-3 must drop `dash.focused` as a second source. |
| D19 | PARTIAL | Incremental patches are minimal (`a_small_mutation_repaints_only_the_changed_cells`: under a quarter of a full frame), `render_due`/`deadline_ms` give the pacing hook. `?2026` bracket is T-A. |
| D25 | DONE | Ring holds interactive visible widgets only (`focus_ring`), stale ids never panic (`focus()` validates, `sanitize` per dispatch), removal clears focus. |
| D32, R13 | DONE | Zoom: all other windows invisible (not hit, not in ring, not painted), restore glyph `⤡`, maximize hidden for a lone window (`WindowState::show_maximize`). Tests `hidden_widgets_leave_the_ring_and_zoom_hides...`, `the_maximize_control_needs_a_peer...`. |
| D37 | DONE | Layout is solved before every mouse hit test when dirty (`hit_testing_uses_the_layout_of_the_change_that_just_happened`). |
| R02 | DONE | Chip max 24 cells with `…`, strip uses full width minus the controls chip, active tab always visible, `‹N`/`N›` overflow chips (hit activate the next hidden tab), same geometry for paint and hit. Tests in `chrome_anatomy`. |
| R03 | PARTIAL | Titles on layout nodes: `WindowMeasure.title`, tab labels and window titles come from `WindowLayoutWindowNode.title`, `set_window_title`. The `TaskLabel`/status glyph naming and deleting the dashboard remount hack is A-3. |
| R06 | DONE | Tab index on every control, maximize only with peers, one `+` new-tab control only when `WindowState::new_tab`, per-tab `⧉` removed. |
| R07 | DONE | See D08. |
| R08 | DONE | Filled `ActiveBase`/`ActiveForeground` active tab, `HoverInteractive` hover and drop target, heavy line set for the focused window (focus visible without colour). |
| R14 | PARTIAL | Hover, tooltips (400 ms, window controls and elided tab titles, `Node.tooltip` anywhere), splitter gutter with hover fill and drag. Scroll bars: T-D `Scrollable`. |
| R16 | DONE | `WindowState.compact` (engine sets it below `COMPACT_ROWS` = 30 rows) and automatic fallback when a window is too short: one-row chrome on the top border; three-row windows can still be closed. |
| R17 | DONE | Side walls start under the body hairline when a chip exists (`no_wall_sticks_out_beside_the_chip...`). |
| R19 | DONE (my parts) | `HoverInteractive`, `BorderEmphasized`, `Surface::Menu/Dialog` wired; `Hovered`/`ContextMenu`/`WindowFocus`/`TabMoved`/`SplitterDragged` now produced (`Hovered` still unused: hover is state, not a signal); one solver; dead layout fields removed. `Capabilities` fields: T-A. |
| R23 | DONE | Navbar centre sits in the free band, never covers left or right items; footer status keeps up to a third of the row, hints end in `…`. |
| tick | DONE | `Tui::tick(now_ms)` ticks all visible widgets (hook `WidgetState::tick`, no widget is timed yet) and opens due tooltips; returns whether a repaint is due. |
| Coordinator relay (T-C) | DONE | `Event::Paste` raw text to `WidgetState::on_paste`; focus gained/lost to `on_focus` (also on every focus change: old gets `false`, new `true`; answers are queued and delivered with the next dispatch result); all mouse kinds with `clicks`/`mods` to the pressed or hit widget; press captures the node so Drag/Up arrive outside its rect; Scroll/Move never move focus. Tests `focus_changes_and_window_focus_events...`, `a_pressed_terminal_keeps_receiving...`. |

## 3. Window anatomy (as built)

Full chrome: 2-row raised chips on corners, hairline body row; controls chip at the right end of the top row (`⤢`/`⤡` when peers or zoomed, `+` when `new_tab`); chips per tab `label ✕`; heavy lines when focused. Compact (below 30 terminal rows or too short): `┌ dev ✕ │ tasks ✕ ─── ⤢ ┐` on the border row. Flat only when under 3 rows or 4 columns.

## 4. Verification

| Command | Result |
| --- | --- |
| `CARGO_TARGET_DIR=.../target-fleet-tb cargo check -p semio-framework-ui --features tui-terminal --message-format=short` | exit 0, no warnings in TUI/elements |
| `cargo test -p semio-framework-ui --features tui-terminal --lib -- tui::` | 323 passed, 3 failed: `tests::paint_log_shows_the_tail_when_following`, `tests::text_cell_width_unicode_goldens` (T-D territory: log scroll bar, grapheme model) and `tests::shell_window_wizard_body_paints_options_after_remount` (asserts an option on the Wizard's first row, which is the filter row; T-D's Wizard rewrite; UNVERIFIED whether it failed before this slice). All my modules pass: `render_equivalence` 3, `engine_behaviour` 20, `chrome_anatomy` 15, `overlays` 11, `pointer_routing` 3 (counts include everything matching the filters, 48 tests). |
| `cargo check -p semio-framework-repo-dashboard --message-format=short` | exit 0 at the end of the slice (A-3 already adapted to the tuple variants). |
| `bun x vitest run --config <scratch config including **/🟦️.ts> 🖱️pointer-routing` | 15 passed: contract, golden geometry, `string-width` row widths, `cli-truncate` ellipsis, independent routing model for all chrome cases. The file is not wired into an Nx target (not mine to edit `project.json`). |

Independent oracles: routing model and third-party `string-width`/`cli-truncate` (TS). Render equivalence uses the owned VT interpreter and the full repaint; no third-party terminal emulator exists in the repo (`@xterm/*` absent).

## 5. Notes for other slices

- `WidgetState::cursor` has inline arms for Input, Wizard, Terminal, Palette (hardware cursor shape/blink); T-D/T-C may replace the Input/Wizard/Terminal arms with element-local functions. The Input painter still paints a block caret: remove it now that the hardware cursor is shown.
- `text::elide_end` replaced my private `elide`; `theme.glyphs.ellipsis()` is used for painted text. Layout-time chip elision uses `…` (theme-less).
- Terminal and overlay widgets claim Tab (`claims_tab`), so Tab reaches a shell.
- `NodeMut::widget()` marks layout dirty (a PTY feed re-runs layout; negligible).

## 6. Requests

A-3 (view):
1. Handle `WidgetSignal::WindowMaximize(_)`, `WindowNewTab(_)` (already done in `🖥️terminal/⌨️controls`), `WindowFocus`, `TabMoved{from,to}` (call `layout::reorder_stack_tab(&mut layout, <window id of the emitting stack>, from, to)` then `shell.remount`), `SplitterDragged{path,delta}` (call `layout::resize_split(&mut layout, tui.scene.rect(shell.canvas), &path, delta)` then `shell.remount`), `ContextMenu{pos,item}` (open `tui.open_menu(pos, items)` and handle `Activated(i)`/`Dismissed` from the returned node id), `Dismissed`.
2. Set `WindowState.labels` (`ChromeLabels`) per locale and `new_tab = true` where a new tab is wanted; tooltips stay off while a label is empty.
3. Drive `tui.tick(now_ms)` every loop and `tui.render_due(now_ms)` instead of `render_full` per event; after each `layout()`/`render` use the widget rect for PTY size; read `tui.cursor()` for `present`; stop syncing `WindowState.focused` (the engine does) and read `tui.focus()`.
4. Windows now need `shell.remount` after `zoom_window`; zoom hides other windows by itself.

T-A: pass `tui.cursor()` to `present`, forward `now_ms`, send `?2026` around patches, report `MouseEvent.clicks` 1 (the engine counts) and `mods`.
T-D: update `🔬️unit` tests that still assume a focusable `Label` is gone; the `Wizard` painter test above.
