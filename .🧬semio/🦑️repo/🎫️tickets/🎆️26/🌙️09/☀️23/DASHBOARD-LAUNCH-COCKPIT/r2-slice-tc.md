# R2 Slice T-C: Embedded Terminal

Executor slice T-C of the round-2 fleet (`r2-plan.md` §2, `fleet-plan.md` §4.2). Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT`, 2026-10-08.
TUI = `🧰️framework/🔨️modules/🖱️ui/⌨️tui`, UI = `🧰️framework/🔨️modules/🖱️ui`, VT = `TUI/📟️vt`.

## 1. What changed

The VT module is now a folder of modules (domain taxonomy, one file each) instead of one 893-line file. Everything lives under `TUI/📟️vt/`
except three small regions of `TUI/🪀️widget/🦀️.rs`.

| Module (`TUI/📟️vt/…`) | Content |
| --- | --- |
| `🦀️.rs` | module root, re-exports (`VtScreen`, `VtParser`, `Palette`, `color_256`, `CellPoint`, `Match`, `Modes`) |
| `🎨️palette` | default fg/bg flags, xterm 16 colours, `Palette::from_theme` (default fg/bg from Foreground and Window surface; black/white/greys follow the theme's ink and paper), `resolve` |
| `🎹️encode` | `InputModes`, `encode_key`, `encode_keypad`, `encode_paste`, `encode_focus`, `encode_mouse`, `encode_wheel_as_arrows` (pure functions of event + child modes) |
| `📜️history` | `Row` (with soft-wrap flag), `History` (absolute row ids, cap), `reflow` (join soft-wrapped rows, re-wrap, map tracked spots) |
| `🧱️screen` | `VtScreen`: grids of `Row`, history, viewport anchored by absolute row, pen with BCE, modes, tab stops, charsets, replies, selection text, word/line/URL spans, search |
| `🧬️parser` | `VtParser`: C0, ESC (+intermediates), CSI with `:` sub-parameters, OSC (title, cwd, 10/11 query, 52 clipboard), DCS/APC swallowing, UTF-8 with U+FFFD recovery, CAN/SUB/ESC abort |
| `🖥️pane` | `TerminalState` (the widget state), selection model, search, browse mode, mouse routing, cursor, paint, scrollbar data |
| `🧪️tests/🔬️unit` | 79 unit tests including the shared-fixture adapters and a 6000-round robustness fuzz |

`TUI/🪀️widget/🦀️.rs` (peer T-B edits the rest of it, I touched only terminal lines, preserved its CRLF endings):
the old terminal region (old `TerminalState`, `key_to_pty_bytes`, `terminal_on_key`, `paint_terminal`) is replaced by one
`pub use crate::tui::vt::pane::{Scrollbar, SelectionMode, TerminalSearch, TerminalSelection, TerminalState};`; the `WidgetState`
arms `on_key`, `on_mouse`, `on_paste` (before T-B's newline flattening, so a terminal gets the raw text), `cursor`, `paint` delegate
to `TerminalState`; new `WidgetState::on_focus(gained)`.

`TUI/🧪️tests/🔬️unit/🦀️.rs`: one test replaced (`terminal_widget_scroll_search_and_passthrough` asserted the `/` search trap; now
`terminal_widget_passes_keys_through_and_opens_search_only_by_command`).

Language-neutral fixtures, features and oracle (all new):

- `UI/🧫️fixtures/⌨️tui-terminal-streams/🔣️.json` (30 byte streams -> grid + cursor), `…-keys/🔣️.json` (154 terminfo capabilities), `…-mouse/🔣️.json` (316 pointer reports)
- `UI/🧪️tests/⌨️tui-terminal-{streams,keys,mouse}/🥒️.feature`
- `T/r2-tc-oracle.py` (`generate` / `verify`; input script, ticket-local, not a permanent script)

## 2. Oracles (third-party, independent of the Rust code)

`vte`/`vt100` are not in `Cargo.lock` and adding them would change the shared lockfile, so the oracles are third-party tools driven by
`r2-tc-oracle.py`; the Rust adapters replay the committed fixtures.

| Feature | Oracle | What it independently produces | Result |
| --- | --- | --- | --- |
| streams | pyte 0.8.2 | rows (trailing blanks trimmed) and cursor for 30 byte streams | `verify`: pyte reproduces 30 cases; Rust screen equals the fixture |
| keys | ncurses terminfo `infocmp -x -1 xterm-256color` | bytes for kcuu1..kcub1, khome/kend, kich1/kdch1/kpp/knp/kbs/kcbt, kf1..kf63, kUP..kPRV and k{UP,DN,LFT,RIT,HOM,END,IC,DC,NXT,PRV}3..7, keypad k*, kxIN/kxOUT | 154 capabilities; Rust encoder equals terminfo exactly |
| mouse | prompt_toolkit decode tables `xterm_sgr_mouse_events`, `typical_mouse_events`, `urxvt_mouse_events` | each report decodes to the button, event type, modifiers and cell it was made from | 316 reports; Rust encoder equals the fixture bytes |

Oracle deviations found and kept out of the fixture (Rust unit tests pin the xterm behaviour instead):

- pyte treats NEL (`ESC E`) as a bare line feed; xterm does CR+LF.
- pyte lets a wide character straddle the right margin; xterm wraps it whole.
- pyte ignores CUP outside the region in origin mode; xterm clamps into the region.
- Streams ending in a pending wrap carry `"cursor": null` (pyte reports column == width, xterm the last column).

Key convention worth knowing: `Key::F(13..=63)` follow terminfo (kf13 = Shift+F1 = `CSI 1;2P`, kf25 = Ctrl+F1, ...), not VT220's `CSI 25~`.

## 3. Per-id status

| ID | Status | Evidence |
| --- | --- | --- |
| D03 (terminal cursor) | DONE in the widget | `WidgetState::cursor` -> `TerminalState::cursor(rect)`: position, `Block/Underline/Bar` from DECSCUSR, blink, hidden by `?25l`, hidden when the viewport is scrolled away, caret in the search row while searching. Tests `cursor_follows_the_child_and_hides_when_scrolled_away`, `cursor_style_and_visibility_are_reported`. Visible on screen needs T-A `present(patch, cursor)` and the engine's focus. |
| D04 (reserved keys, encoder) | DONE in the widget | Passthrough sends every key; encoder: arrows/Home/End with DECCKM, modified `CSI 1;m`, Insert/Delete/PgUp/PgDn, F1-F63, Alt = ESC prefix, Ctrl+Space = NUL and the full Ctrl map, BackTab `CSI Z`, Enter CR/CRLF (LNM), Backspace DEL/BS, keypad (`encode_keypad`). 154 terminfo cases. The dashboard still has to stop intercepting Ctrl+W, Tab, Esc (Request A-3). |
| D05 (search trap) | DONE | `/` is forwarded to the child. `begin_search()` is the only entry; Esc always closes it (`search_is_opened_only_by_command_and_escape_always_leaves`). |
| D13 (resize geometry) | PARTIAL | `TerminalState::fit(rect) -> bool`, `resize(size)`; reflow instead of the top-left copy. Calling `fit` after every layout pass and sending the PTY resize is A-3 (Request). |
| D14 (modes and replies) | DONE in the widget | DECSET 1,5,6,7,9,12,25,47,66,1000,1002,1003,1004,1005,1006,1007,1015,1047,1048,1049,2004,2026 tracked; DA1/DA2/DSR 5,6/DECXCPR/DECRQM/XTVERSION/`CSI 18t`/OSC 10,11 answered through `feed() -> Vec<WidgetSignal::TerminalInput>`; replay never answers (`feed_replay`). Paste/focus/mouse honoured by the widget; delivery of `Event::Paste`/`FocusGained`/mouse kinds depends on T-B (Request). |
| D15 (scrollback) | DONE in the widget | Absolute row ids, viewport anchored while output arrives, eviction clamps, wheel scrolls (3 lines/step), wheel on the alternate screen = arrows (mode 1007), wheel forwarded when the child tracks the pointer, scrollbar data and click/drag, browse-mode keys (PgUp/PgDn/Home/End/j/k/Ctrl+B/F/U/D/g/G). |
| D16 (selection/copy) | DONE in the widget | Drag, double click = word, triple click = line, Shift bypasses child mouse tracking, copy on release as `WidgetSignal::Copy`, Ctrl+click on a URL = `OpenUrl`, right click = `ContextMenu`, `copy_selection`, `selected_text`, `select_all`, highlight painted, selection anchored to absolute rows. The host must hand `Copy` to `TerminalBackend::copy` and implement paste (Request). |
| D35 (resize reflow) | DONE | History + screen re-wrap at the new width, wide characters kept whole, cursor/screen top/viewport anchor mapped, history grows/shrinks, alternate screen cropped. Release timing: 10k rows x 200 columns -> 120 columns takes 56 ms; hosts should coalesce resize bursts. |
| R04 (rect-derived size) | PARTIAL | `fit(rect)` is the primitive; the `-4` guess and `sync_terminal_sizes` live in the dashboard (A-3). |
| R09 (theme, cursor, indicator) | DONE | Default fg/bg and the 16 colours resolved through `Palette::from_theme` at paint time (default colours are flagged, not baked in), reverse video (`?5`) swaps them, cursor, selection and search highlights, `↓N` scrolled-back indicator, scroll bar overlay while scrolled up. |
| R15 (VT gaps) | DONE | CHA, VPA, CNL, CPL, CHT, CBT, HPA/HPR/VPR, REP, SU/SD, ICH/DCH/ECH/IL/DL with correct margins, IND/NEL/RI/HTS/TBC, charset designators and DEC line drawing (SO/SI), DECSC/DECRC + CSI s/u, DECALN, DECSTR, RIS (keeps history), IRM, LNM, colon SGR (`38:2::r:g:b`, `38:5:n`, `4:3`), BCE, ED 3, BEL counter, CAN/SUB/ESC abort, unknown CSI/DCS/APC swallowed cleanly. |

## 4. Public API for consumers

`TerminalState` (via `widget::TerminalState`, unchanged constructor `new(size, scrollback_cap)`):

- `feed(&[u8]) -> Vec<WidgetSignal>`: `TerminalInput(replies)` and `Copy(text)` (OSC 52). The host forwards them. `feed_replay(&[u8])` for daemon replay (answers nothing).
- `resize(Size)`, `fit(Rect) -> bool`, `apply_theme(&Theme)` (colour query answers), `title()`.
- `pub passthrough: bool`, `set_passthrough`, `wants_escape()`.
- `begin_search()`, `end_search()`, `step_search(older)`, `search() -> Option<&TerminalSearch>` (Enter/Up/Ctrl+P older, Shift+Enter/Down/Ctrl+N newer; signals `ValueChanged(query)`, `SelectionChanged(index)`).
- `copy_selection()`, `selected_text()`, `clear_selection()`, `select_all()`, `scrollbar(track) -> Option<Scrollbar>`.
- `on_key`, `on_mouse(rect, ev)`, `on_paste(text)`, `on_focus(bool)`, `cursor(rect)`, `paint(theme, rect, buf)`.
- `screen: VtScreen` with `view_offset()`, `scroll_view(i64)`, `scroll_view_to_bottom/top`, `modes` (`synchronized`, cursor shape...), `input_modes()`, `cwd`, `bells`.

Removed (greenfield, no shims): `scrollback_offset`, `follow`, `pinned`, `search`, `search_active` fields; `TerminalSelection{start:Pos,end:Pos}` (now `CellPoint` absolute); `VtScreen::blit_to(.., offset)` (now `blit(dest, rect, &Palette)`); the flat mode fields on `VtScreen` (now `modes` and `input`).

Signals: `Toggled(true)` when browse mode hands back to passthrough (Esc / `i`).

## 5. Commands and results

All run from `/c/git/semio` with `CARGO_TARGET_DIR=/c/git/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-tc` unless noted.

| Command | Result |
| --- | --- |
| `cargo check -p semio-framework-ui --features tui-terminal --message-format=short` | exit 0, no warning in `📟️vt` |
| `cargo check -p semio-framework-ui --features tui` and `… --features tui --target wasm32-unknown-unknown` | exit 0 / exit 0 |
| `cargo test -p semio-framework-ui --features tui-terminal --lib -- component::vt component::tests::terminal component::tests::mount_window component::tests::vt_` | 87 passed, 0 failed (79 new vt tests, 8 existing terminal/VT tests of the shared suite) |
| `cargo test -p semio-framework-ui --features tui-terminal --lib -- tui::` (whole TUI suite, mid-run snapshot) | 257 passed, 5 failed: `render_equivalence::mutating_a_node_after_the_first_frame_yields_a_non_empty_patch`, `cell_buffer_put_pairs_and_orphans_wide_char_continuations`, `diff_full_redraw_when_sizes_differ`, `shell_window_wizard_body_paints_options_after_remount`, `text_cell_width_unicode_goldens`; none touches the terminal, they sit in T-B/T-D regions that were being edited |
| `python -I -X utf8 T/r2-tc-oracle.py verify --libs <pyte dir> --libs <prompt_toolkit dir>` | streams: pyte reproduces 30 cases; keys: terminfo reproduces 154; mouse: prompt_toolkit decodes 316 |
| robustness fuzz, 120 000 rounds (temporary bump of the committed 6 000), debug build with overflow checks | no panic, cursor and viewport invariants hold |
| `cargo check -p semio-framework-repo-dashboard` | exit 101 on errors outside my API (A-1 `🧭️cli` `Option<ErrorCode>` Display, earlier A-3/T-B signature drift); no error names a `TerminalState`/`VtScreen` member, and `selected_text` (used by A-3's `⌨️controls`) was kept for them |

While T-B/T-D files were mid-edit I iterated in an isolated scratch crate that includes the real `vt`, `geometry`, `cell`, `text`, `event` sources
with a stub `widget` (outside the repo); the final results above are from the real crate.

Not run: any PTY/ConPTY session, the dashboard binary, a real browser/xterm.js host. Runtime behaviour in a live shell is UNVERIFIED.

## 6. Requests

Engine and windows (T-B):

1. `Event::Paste(text)` -> focused widget `on_paste(text)`; for a terminal pass the raw text (the generic newline flattening in `WidgetState::on_paste` is bypassed for `Terminal` before it runs).
2. `Event::FocusGained/FocusLost` -> focused widget `on_focus`; on a focus change call `on_focus(false)` on the old and `on_focus(true)` on the new widget (`WidgetState::on_focus` is new).
3. Mouse pipeline: deliver `Down/Up/Drag/Move/Scroll` (all buttons, `mods`, `clicks` 1/2/3) to the hit node's `on_mouse(rect, event)`; capture the node on `Down` so `Drag/Up` still arrive outside its rect (the pane clamps and auto-scrolls); do not steal focus on `Scroll/Move`.
4. Honour `WidgetState::cursor` for the focused terminal every frame (hardware cursor with shape and blink).

Terminal I/O (T-A):

5. Enable `?1004h` (focus), `?1003h`/`?1002h` as needed, keep SGR `?1006h`; fill `MouseEvent.clicks` (double/triple click by position and time) and `mods`.
6. A `Key` variant for numeric keypad keys: the encoder side exists (`encode::KeypadKey`, `encode_keypad`, tested against terminfo) but the event type cannot express it.
7. Paste decoding must stay raw UTF-8; the pane strips C0/ESC itself and brackets exactly once.

Dashboard view (A-3):

8. Forward the result of `terminal.feed(&data)` (`Vec<WidgetSignal>`): `TerminalInput` to the PTY, `Copy` to `TerminalBackend::copy`. Use `feed_replay` for the daemon's replay burst and never rebuild the state with `TerminalState::new` on `ReplayComplete` (it loses `passthrough`, search, selection); `sessions` currently does that at `📡️sessions/🦀️.rs:227`.
9. Stop encoding bytes or wrapping paste in the view; reserve only the prefix key; use `set_passthrough(false)` for browse mode (scroll/copy/search keys are handled by the pane) and `begin_search()` for the search command; Esc: when `wants_escape()` is true forward it to the widget, otherwise the prefix decides.
10. After every layout pass call `terminal.fit(rect)` and send the PTY resize when it returns true (removes the `-4` guess, R04, D13); coalesce resize bursts (a 10k-row reflow is ~56 ms in release).
11. Call `apply_theme(&theme)` at creation and on appearance changes; optionally skip repainting a pane while `screen.modes.synchronized` is true (mode 2026, tracked not enforced).
12. Paste: read the clipboard on the paste command and call `on_paste`; middle click is intentionally not handled in the pane.

Daemon (A-2):

13. The VT derives every mode from the byte stream, so the replay on attach must contain the current DECSET state (`?1000`, `?1006`, `?2004`, `?1004`, `?1049`, `?25`, DECSCUSR); the `📼️replay` tracker is the place.

Text and cells (T-D):

14. The VT stores bytes 0x40 and 0x80 of `Cell::attrs` as private "default colour" flags and strips them in `blit`; please do not allocate `attr` bits 0x40/0x80. A STRIKE/BLINK/CONCEAL bit (0x20 is free) would let the VT keep SGR 5/8/9 (currently swallowed).
15. The VT keeps one scalar per cell; combining marks and ZWJ clusters from a child are dropped. If `CellBuffer` tails become usable from outside, the VT can attach them.

## 7. Known limits

- Search matches within one row only (not across a soft wrap); case-insensitive only; capped at 5 000 hits.
- DECRQSS, XTGETTCAP, kitty keyboard protocol (answers "unsupported"), sixel, OSC 8 hyperlinks, mouse pixel mode 1016 are not implemented.
- `?2026` synchronized output is tracked but not enforced by the pane.
- Selection and search hits are dropped on resize (they refer to the old layout).
