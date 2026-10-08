# R2 audit: TUI chrome defects R01–R25, task naming, keymap, locale

Read-only audit of the current code against `tui-chrome-parity-audit.md` (R01–R25) and `fleet-plan.md` §2.3 (TaskLabel).
Method: static reading of the files below plus `rg` searches. Nothing was built, run or tested (no cargo, no nx, no daemon).
Every runtime-behaviour statement is therefore marked **UNVERIFIED** where it depends on execution. Line numbers are of the
files as they are now (the audit's `TUI:nnnn` line numbers refer to the old 5451-line monolith and are obsolete).

## 0. Aliases and where the code now lives

| Alias | Path (repo-relative) |
| --- | --- |
| `TUI` | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/` (crate-root glue, engine, scene, layout, backend, vt, text, cell, ansi, chrome, widget, pty, host) |
| `EL` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/<Element>/🎯️targets/⌨️tui/🦀️.rs` (painters: Window, Navbar, Footer, Tabs, List, Table, Wizard, Input, Select, Log, Label, Divider, Chip) |
| `THEME` | `TUI/🎨️theme/🦀️.rs` |
| `VIEW` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs` (919 lines) |
| `REG` | `…/🎛️dashboard/🎮️registry/🦀️.rs` |
| `CMD` | `…/🎛️dashboard/🌳️command-tree/🦀️.rs` |
| `IPC` | `…/🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs` |
| `DMN` | `…/🎛️dashboard/🌀️daemon/🦀️.rs` |
| `PREF` | `…/🎛️dashboard/⚙️preferences/🦀️.rs` |
| `LOC` | `🧰️framework/🔨️modules/🖱️ui/🌐️locale/` (+ `🎚️axes/🤖️generated/🦀️.rs`) |

Audit mapping for the old line references: `WIN`→`EL/🪟️Window`, `NAV`→`EL/🔝️Navbar`, `FOO`→`EL/🔚️Footer`, `LIST`→`EL/📃️List`,
`TBL`→`EL/📊️Table`, `WIZ`→`EL/🧙️Wizard`, `INP`→`EL/✏️Input`, `TUI:3732-3838` (window geometry)→`TUI/🖥️chrome` (`window_chip_layout`) +
`EL/🪟️Window`, `TUI:1211-2106`→`TUI/📟️vt`, `TUI:855-879`→`TUI/🔡️ansi` (`emit_runs`), `TUI:567-664`→`TUI/📝️text`,
`TUI:2652-2715`→`TUI/📏️layout`, `DASH:*`→`VIEW`.

## 1. Verdicts R01–R25

Summary: **0 FIXED, 1 PARTIAL (R03), 24 OPEN.** No defect in the list is fully fixed.

| ID | Verdict | Evidence (file:line) | Note |
| --- | --- | --- | --- |
| R01 | OPEN | `TUI/🔌️backend/🦀️.rs:422-447` (Unix `wait` = `select` on the tty fd only), `:653-` (Windows `WaitForSingleObject` on stdin only); no SIGWINCH / `WINDOW_BUFFER_SIZE` handling anywhere in `TUI`; `TUI/🏃️host/🦀️.rs:30` is the only producer of `Event::Resize` (WASM host); `VIEW:635-639` arm is unreachable natively; `VIEW:609` reads size once | UNVERIFIED at runtime |
| R02 | OPEN | `TUI/🖥️chrome/🦀️.rs:363` top-left budget = `rect.x + width/2`; `:274` and `:283` silent `break` in `layout_corner_tabs` (`:265-293`); `build_corner_tab_interior` (`:213-262`) has no 24-cell cap and no `…`; no overflow chips; no keep-active-visible logic | later tabs and an active tab beyond the half-width vanish |
| R03 | PARTIAL | Building blocks landed: `REG:21-30` (`TaskLabel`), `REG:268-294` (`tab_text`, `tab_texts`, `window_title`), `REG:253-263` (`elide_middle`, `elide_end`), `REG:218` (`TAB_CELLS=24`), `IPC:76-84` and `IPC:119` (`SessionCommand.label`), `DMN:79-88, 122`, `REG:545-563` (`LaunchProcess.label`, `Launch.label`). Not wired: `CMD:15-20` `CommandSpec` has only cmd/args/cwd/env and `CMD:62` drops `label`/`command_id`/`ready`; `VIEW:236` builds `SessionCommand` with `..Default::default()` (empty label); `VIEW:251` title `"{cmd} {args}"`; `VIEW:317,323` title `"task"`; `VIEW:331-335` title `[{status} {detail}] {cmd} {args}`; `VIEW:343-352` string-match relabel; `VIEW:662,754,767,799` `"Commands"`; `VIEW:614` layout title `"Tasks"` | no user-visible change; see §2 |
| R04 | OPEN | `VIEW:214-216`: `render_full()` used only to read a rect, then `inner - 4` in both axes; real content inset is `[top,1,bottom,1]` (`TUI/🖥️chrome/🦀️.rs:430-444`), i.e. width minus 2 and height minus 4 with a top chip, so the VT and PTY start 2 columns narrow (`VIEW:216`, PTY size from it); `VIEW:320` discarded `render_full`; `resize_terminals` only at `VIEW:513` (ReplayComplete) and `VIEW:637` (dead arm, R01); no `sync_terminal_sizes` after layout | size arithmetic checked by hand, not run |
| R05 | OPEN | `TUI/🔲️cell/🦀️.rs:16-22` `Cell { ch: char }` (one scalar per cell); `TUI/📝️text/🦀️.rs:212` `0xfe00..=0xfe0f` (VS16) is zero width; `text.rs:431-440` `char_cells`; `cell.rs:83-98` `put_str` skips zero-width scalars (`:88-89`); no cluster model, no presentation-aware table | |
| R06 | OPEN | `TUI/🖥️chrome/🦀️.rs:128` `WindowClose(w.active_stack_tab)` (signal carries the active tab, not the hit tab); `:148` `window_control_at` likewise; `:217` `show_max = w.maximizable` (never set to false by the dashboard, grep empty); `:219` `show_new_glyph = true` (`⧉` on every chip); `VIEW:644-646` ignores the index; `VIEW:659-670` `WindowNewTab` opens a window | |
| R07 | OPEN | `TUI/⚙️engine/🦀️.rs:208-233` `render()` uses `take_dirty(root)` only; `TUI/🎬️scene/🦀️.rs:150-169` `mark_dirty` stops at the first ancestor already flagged and `take_dirty` clears only the node; `VIEW:623` and `VIEW:907` call `render_full()` on every frame | dead incremental path confirmed by reading; test of R7 repro not run |
| R08 | OPEN | `EL/🪟️Window/…/🦀️.rs:113` focus = `ActiveBase` border colour only (no heavy line set); `:45` active tab = `Accent` text on window surface, no fill; `:44` inactive = `MutedForeground` | |
| R09 | OPEN | `TUI/📟️vt/🦀️.rs:7-8` `DEFAULT_FG=[192,192,192]`, `DEFAULT_BG=[0,0,0]` fixed (not theme roles); `TUI/🪀️widget/🦀️.rs:399-410` `paint_terminal` paints VT cells, search row only, no cursor, no scrolled-back indicator; `widget.rs:487-490` `cursor()` returns `None`; `TUI/⚙️engine/🦀️.rs:70-76` `cursor()` therefore `None`; `VIEW:623`, `VIEW:907` pass `None` cursor to `present` | "view drifts with new output" not verified (UNVERIFIED) |
| R10 | OPEN | `TUI/📝️text/🦀️.rs:447-459` `truncate_to` with no ellipsis; no `…` in `TUI` or any `EL/*/…/⌨️tui` file (grep); `EL/🔚️Footer:23-25` status unbounded; elide helpers exist only in `REG:252-263`, used only by `tab_text` | helper not shared with TUI |
| R11 | OPEN | `THEME:21-32` `Role` enum has no Success/Warning/Danger/Info; `VIEW:331` running state is bracket text | |
| R12 | OPEN | `TUI/🔌️backend/🦀️.rs:287-288` `assumed_capabilities()` (TrueColor, no sync, Unicode Full), returned by `:412-414` and `:644-646`; no non-test caller of `.capabilities()` in `TUI` or `DASH` (grep); no `COLORTERM`/`TERM`/`NO_COLOR` anywhere | |
| R13 | OPEN | `TUI/🖥️chrome/🦀️.rs:550-556` zoom reparents the zoomed window only; other windows keep `visible` and are painted underneath; `TUI/📏️layout/🦀️.rs:368-370` `zoom_window` only sets a field; no zoom indicator; `VIEW:651-657`, `VIEW:740-749` toggle zoom; maximize glyph shown for a single window (`:217`) | |
| R14 | OPEN | `TUI/⚙️engine/🦀️.rs:141-178` `dispatch` acts only on `MouseKind::Down`; `:31` `hovered` never written; `TUI/🪀️widget/🦀️.rs:481-484` `set_hover` is a stub returning `false`; `:467-470` `on_mouse` handles only Wizard; `TUI/🔡️ansi/🦀️.rs:63` mouse mode `?1002` (no `?1003`); no tooltip, gutter, scroll bar or wheel handling; `WidgetSignal::SplitterDragged` (`widget.rs:39`) constructed nowhere | |
| R15 | OPEN | `TUI/📟️vt/🦀️.rs:180-213` `feed_escape`: handles `[ ] P X ^ _ 7 8 c` only, `_ => Ground` (charset `ESC ( B` leaks `B`; no `D`, `E`, `M`); `:236-287` `finish_csi` lacks `G d E F s u b`; `:218-235` `feed_csi` does not accept `:` (0x3a) so colon SGR terminates the CSI | |
| R16 | OPEN | `TUI/🖥️chrome/🦀️.rs:334` `<4x4` → flat box, no title or controls; `:354-358` `min_h` 6/4/2 rows; no compact one-row variant | |
| R17 | OPEN | `EL/🪟️Window/…/🦀️.rs:137-142` side walls start at `rect.y+1` even when a top chip exists (should start at `top_body_y+1`); `:170-171` draws the corner `┐` at `top_body_y` when there is no top-right chip, so the wall at `y+1` sticks up above that corner | |
| R18 | OPEN | `TUI/🔲️cell/🦀️.rs:60-75` `put`: the orphan-continuation branch is empty (`// keep continuation paired`), no blanking of the other half; `cell.rs:140-142` size change returns one run of `w*h`; `TUI/🔡️ansi/🦀️.rs:50-53` zero-width cells skipped without advancing the terminal | |
| R19 | OPEN | `THEME:95-108` `HoverInteractive`, `BorderEmphasized`, `BorderElement` and `Surface::{Pane,Dialog,Menu}` are only mapped, no painter uses them (grep); `TUI/⚙️engine/🦀️.rs:32-33` `capture` `#[allow(dead_code)]`; `widget.rs:27,39` `Hovered`, `SplitterDragged` (and `TabMoved`, `WindowFocus`, `OpenUrl`, `ContextMenu`) are never constructed in TUI or element painters (`WindowClose` is, see R06); `TUI/🔌️backend/🦀️.rs:245-246` `synchronized_output`, `unicode` unused; two solvers: `TUI/📏️layout/🦀️.rs:48` `distribute` vs `:224` `solve_axis` and `:275` `solve_window_layout` | |
| R20 | OPEN | Painters: `EL/🧙️Wizard/…:111` `"no matches"`, `EL/📊️Table/…:110` `"(empty)"`. Dashboard literals not routed through `text()`: `VIEW:612` `semio`/`dashboard`, `:613` `"connecting · discovering commands"`, `:614` `"Tasks"`, `:617` `"connecting"`/`"discovering commands"`, `:476` `"connected"`, `:486` `"reconnecting"`, `:521` `"daemon stopped"`, `:287,292,302` error strings, `:604` whole help text, `:753,766` `"shell"`, `:902-903` footer `" · discovering {}s · C-B e cancels"` and `" · {} commands · "` (no German) | 17 `self.text(` lines show the intended pattern |
| R21 | OPEN | `EL/✏️Input/…:15-16` `cursor += c.len_utf8()`, `:25-26` Left does `cursor -= 1` (one byte, can land mid-character before the next insert), `:44-46` caret cell replaces the glyph, no horizontal scroll | runtime panic claim UNVERIFIED |
| R22 | OPEN | `EL/📃️List/…:12-32` `list_on_key` never updates `offset`; `:37` offset only read; `:40` selection painted only when focused. (Wizard does use a `viewport()`; Table's `table_on_key` was not checked) | |
| R23 | OPEN | `EL/🔝️Navbar/…:26-30` left/centre/right drawn with no collision handling (centre overwrites left); `EL/🔚️Footer/…:17-25` hints then status, no minimum gap, no ellipsis | |
| R24 | OPEN | `TUI/🖥️chrome/🦀️.rs:154-156` `⤢ U+2922`, `⧉ U+29C9`, `✕ U+2715` hard-coded; no ASCII fallback; `UnicodeLevel` (`backend.rs:236-239`) is defined but never consulted by a painter | font coverage UNVERIFIED |
| R25 | OPEN | `VIEW:331-335`: `format!("[{status} {detail}] {} {}", …)` with empty detail produces `[running ] …` | |

## 2. Task tab naming: is it computed from a TaskLabel?

**Short answer: the naming function exists and matches §2.3 / §4.3 in shape, but the running dashboard does not use it.**

What matches fleet-plan §2.3:
- `TaskLabel { verb, owner: Vec<String>, subject, qualifier, parameters: Vec<(String,String)>, members: u16 }` is the same shape in `REG:21-30`, `IPC:76-84`, `DMN:79-88`.
- It travels on `SessionCommand.label` (`IPC:119`, `DMN:122`) and on the registry's `LaunchProcess.label` (`REG:551`) and `Launch.label` (`REG:561`).
- `tab_text(label, locale, budget)` (`REG:268-283`): `verb subject [qualifier] [×members]`; the subject is middle-elided keeping its last `TAIL_CELLS = 6` cells (`REG:219, 253-257`); the verb is never elided (`REG:280`); `tab_texts` (`REG:285-291`) adds ` ·2`, ` ·3` for duplicates by creation order.
- `window_title` (`REG:294-302`) = verb · owner path · subject · qualifier · parameters.
- `TAB_CELLS = 24` exists (`REG:218`).

What does not match or is not connected:
1. **Not called.** `tab_text`, `tab_texts`, `window_title`, `TAB_CELLS` have no caller outside `REG` (grep, non-test). `VIEW` never builds a tab title from a label.
2. **Label dropped at the boundary.** `CMD:62` converts `LaunchProcess` to `CommandSpec`, which has only `cmd, args, cwd, env` (`CMD:15-20`). `VIEW:236` sets `SessionCommand` with a default label. The TUI launcher therefore never sends a `TaskLabel` to the daemon.
3. **Titles still raw.** `VIEW:251` and `VIEW:335` use `cmd args`; `VIEW:331-335` prepends `[status detail]`; `VIEW:343-352` string-matches window ids to rewrite tab labels; `VIEW:317,323,662,754,767,799` use English placeholder titles.
4. **Status glyph missing.** §2.3 and §4.3 rule 4 want status as a glyph and colour role in the tab; `tab_text` has no status input and `THEME` has no status roles (R11).
5. **No tests.** No test in the dashboard or TUI references `tab_text`, `tab_texts`, `window_title`, `elide_middle` or `TaskLabel` (grep over `🧪️tests` and `🔬️unit` paths). `REG` test file is one line (`REG` test path, `use super::*;`).
6. **Elision policy diverges from §4.3 examples (hand-traced, UNVERIFIED by running).** `REG:275` floors the subject budget at 8 cells (`TAIL_CELLS + 2`) before the qualifier is cut. Traced for verb `dev`, subject `puzzle3d·react`, qualifier `all examples`, budget 24: `dev p…·react all exampl…`. This follows the function's own doc comment (`REG:266-267`) but not the §4.3 example `◐ dev puzzle3d·react`. Coordinator decision needed.
7. Duplicates (`·N`) are implemented only inside the pure function, not per stack or per session list in the view.

## 3. Keymap: accessible and customizable?

**No.**
- Bindings are match arms in `VIEW` `run_with`: `Ctrl+B` leader (`VIEW:694`), leader keys (`VIEW:700-816`), `q` detach (`VIEW:823`), `Ctrl+W` close (`VIEW:827`), `Tab`/`BackTab` (`VIEW:836`), `Esc` (`VIEW:854`).
- Hints are duplicated by hand: `VIEW:546-560` (`footer_hints`), `VIEW:604` (help text), `README.md` table (about line 51+). They are not generated from one table.
- The armed-leader footer (`VIEW:548`) omits `z` zoom (`:740`), `x` close (`:776`), `t` toggle input (`:784`), `-` and `|` splits (`:750`, `:763`), so these bindings are undiscoverable in the UI and in the help line.
- `PREF:53-56` / preferences schema (`🧬️schema/⚙️preferences/🔣️.json`) have language, appearance, terminology, renderer, layout only: no keymap, no glyph mode, no colour mode, no reduce-motion.
- The only key test is a parsing vector: `VIEW` `…/🧪️tests/🔬️unit/🦀️.rs:5-11` checks that `Ctrl+B p` bytes parse to the expected events, against `🧫️fixtures/⌨️controls/🔣️.json`. The `🧪️tests/⌨️controls/🥒️.feature` file describes Windows and Node parity; no native conhost run was done (UNVERIFIED).
- No screen-reader or accessible-name layer exists in `TUI` (`TUI/🖥️chrome` has no labels; the audit's `ChromeControlHint` gap stands).

## 4. Locale and dashboard UI strings

- **Where strings live.** Dashboard UI strings are inline `self.text(en, de)` pairs in `VIEW` (`VIEW:74-76` helper; about 17 lines use it), plus the verb table `REG:222-228` (inline German map, unknown verbs fall back to English). The framework label catalogue `LOC` (`app_labels!`, `LabelText`, `AppLabels`, `🧩️labels/🦀️.rs`) is **not used by the dashboard** (grep: no `app_labels!`/`AppLabels` in `DASH`).
- **Two Locale types.** `VIEW:40` `enum Locale { English, German }` duplicates `ui_locale::Locale` (`🎚️axes/🤖️generated/🦀️.rs:10-14`, `#[default] En`, `De`), which `REG:15` and `CMD:55` use. The two cannot disagree silently today only because `VIEW:147` and `VIEW:616` map `preferences.language == "de"` by hand.
- **German coverage.** Every `self.text` call has both languages. Not covered (English only): the literals listed under R20 (`VIEW:476, 486, 521, 604, 612-617, 662, 753, 766, 799, 902-903`, error strings `287, 292, 302`), `EL/🧙️Wizard:111` `"no matches"`, `EL/📊️Table:110` `"(empty)"`, and `REG` error text.
- **Default language.** `PREF:22` `Preferences::default()` = `en`; `LOC` `Locale` `#[default] En`; `VIEW:616` treats any non-`de` value as English. Validation (`PREF:53-56`) limits values to `en`/`de`, so the fallback is not reachable with bad input. The README (`README.md:16`) says "opinionated defaults" without naming the language; whether English-as-default is "documented" is a coordinator decision (AGENTS.md says English first).
- **Mixed languages.** `VIEW:902-903` shows the footer status in English while the hints are German in `de` mode; `VIEW:613` shows English before preferences load.

## 5. Other findings (outside R01–R25)

1. `VIEW` launcher is built on the legacy `command_tree::CommandSpec` path (`CMD:62`), while the registry (`REG`) is the declared source of truth in fleet-plan §2; the registry's `command_id`, `label` and `ready` are discarded before the view.
2. `VIEW:236` sends `ready: None` and `command_id: ""` for every TUI spawn, so the daemon's ready detection and `--wait-ready` cannot work for TUI-launched tasks (by reading; UNVERIFIED at runtime).
3. `maximizable` and `closable` are never set by the dashboard (grep empty), so maximize and close show on every tab.
4. Dead or never-emitted signals: `SplitterDragged`, `TabMoved`, `WindowFocus`, `OpenUrl`, `Hovered`, `ContextMenu` (`TUI/🪀️widget/🦀️.rs:24-40`), and `Capabilities` fields.
5. `VIEW:604` help text omits the zoom, close, split and toggle leader bindings (see §3).
6. `VIEW:624` `[DEBUG]` trace line is correctly prefixed; no other temporary logs found.

## 6. Coordinator decisions needed

1. Wire the registry label through `CMD` → `VIEW` → `SessionCommand.label` (R03), or accept the legacy spawn path as the target.
2. Decide the tab elision policy (§2 item 6) and whether status glyphs belong in `tab_text` (§2 item 4).
3. Decide whether English is the documented default language (§4).
4. Decide whether the keymap should become schema-first data (one table driving bindings, hints and help), since none exists (§3).
5. Decide whether the dashboard should adopt `LOC` / `app_labels!` and drop its own `Locale` enum (§4).

## 7. Not run (UNVERIFIED)

No `cargo check`, `cargo test`, nx run, PTY session, terminal probe or screenshot. All verdicts are from source reading. R-level runtime claims (panics, drift, font coverage, terminal behaviour) are flagged inline.
