# TUI interaction audit (input, mouse, focus, cursor, selection, embedded terminal, event loop)

Read-only source audit. No build, test or `nx`/`cargo` command was run. Every claim is tied to `file:line`.

Evidence tags: **[S]** proven by reading the source; **[T]** an existing unit test asserts or enshrines the behaviour; **[E]** estimate or inference that was not measured or run (called out explicitly, confirm before acting on the number).

## Path aliases

| Alias | Path (relative to repo root) |
| --- | --- |
| `TUI` | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🦀️.rs` (5451 lines, one file) |
| `TUIT` | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🔬️unit/🦀️.rs` |
| `WIN` | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🪟️windows/🦀️.rs` |
| `EL/<X>` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/<X>/🎯️targets/⌨️tui/🦀️.rs` |
| `DASH` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs` (941 lines) |
| `CONN` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📎️connection/🦀️.rs` |
| `DMN` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🦀️.rs` |

TUI module map (line ranges in `TUI`): geometry 8, theme 87, text 205, cell 669, ansi 839 (emit 843, parse 911), vt 1211, event 2109, scene 2178, layout 2428, widget 2968, chrome 3411, engine 3986, backend 4198 (unix 4439, windows 4606), pty 4826, host 5356. `NativeTerminal` has exactly one consumer in the whole framework: `DASH:8,629`. There is no signal handling anywhere in the framework (grep for `sigaction|libc::signal|SIGWINCH|SIGTERM|SIGHUP|signal_hook` over `🧰️framework` finds nothing).

---

## 0. Executive summary

1. The engine only understands **left/right/middle button-down** and **keys**. `Tui::dispatch` drops mouse up, drag, move, wheel, paste, focus and (natively) resize events (`TUI:4094-4132`). Hover, mouse capture, double-click, wheel scrolling, drag-resize, tab drag, text selection and copy do not exist at all.
2. The dashboard does **not** use the engine's focus model for keyboard routing. It keeps its own `dash.focused` (window id) and re-points `tui.set_focus` on every key (`DASH:885`, `897`). Mouse clicks change only `tui.focus` (`TUI:4105-4107`), so clicking a pane never moves keyboard focus.
3. **Terminal resize is never detected.** `Event::Resize` is constructed in exactly one place, `WasmHost::resize` (`TUI:5386`); the unix/windows backends never emit it; the dashboard calls `term.size()` once at startup (`DASH:631`). The resize handler at `DASH:657-661` is dead code in a native terminal.
4. **The embedded terminal pane is not usable as a terminal**: no visible cursor, Tab/Esc/Ctrl+B/Ctrl+W/PageUp/PageDown/End/Ctrl+P and `/` never reach the child, Delete/Insert/F-keys/Alt+key are dropped, paste is discarded, mouse and wheel are not forwarded, application-cursor mode is ignored, and typing `/` enters a search mode that cannot be exited.
5. **Echo latency is bounded below by the 80 ms poll**: the UI thread sleeps in `select()` on the TTY only (`TUI:4551-4556`, `DASH:651`); PTY output arrives on another thread through a channel and is not seen until the select times out. The connection thread adds two more 10 ms sleeps (`CONN:76`).
6. `Tui::render()` (the diffing path) is **unusable**: dirty flags are cleared only on the root while `mark_dirty` short-circuits on already-dirty nodes (`TUI:2328-2347`, `4162`). Nothing below the root can re-open the gate after the first frame. The dashboard hides this by calling `render_full()` for every paint (`DASH:929`), which re-emits every cell of the screen.
7. `WizardState` is O(N) per keypress and per frame (`TUI:3110-3114`, `EL/Wizard:31,75`): it lowercases all ~44k option strings at least twice per keypress. The viewport has no persisted offset, so the highlighted row is always pinned to the bottom edge once the selection passes the first page.
8. Selection, clipboard, OSC 52 and mouse passthrough exist only as dead or half-built pieces: `TerminalState.selection` is never written, `selected_text()` is never called, `HostClipboard`/`osc52_copy_sequence` have zero consumers outside tests, and the VT mouse/bracketed-paste/cursor flags are tracked but never read (`TUI:1552-1556`).
9. Enabling mouse reporting (`?1002h`, `TUI:902`) disables the host terminal's native text selection, and nothing replaces it. Text cannot currently be copied out of a dashboard pane without knowing the terminal-specific modifier bypass.
10. Tab-chip controls act on the wrong window: the close/maximize/new glyphs are drawn on every tab chip, but `window_hit` returns a bare `WindowClose` that the dashboard applies to the **active** window (`TUI:3538-3546`, `4114-4122`, `DASH:666-671`).

---

## 1. Capability matrix

| # | Capability | State | Anchor |
| --- | --- | --- | --- |
| 1a | Alt screen (`?1049`) enter/leave | implemented | `TUI:902,907` |
| 1b | Cursor hide on enter / show on leave | partial (hidden forever while running) | `TUI:902,907` |
| 1c | Mouse press/release/drag (`?1002`) + SGR (`?1006`) | implemented | `TUI:902` |
| 1d | Mouse any-motion (`?1003`) for hover | missing | no `1003` string in `TUI` |
| 1e | Pixel mouse (`?1016`) | missing | no `1016` |
| 1f | Bracketed paste (`?2004`) | broken (enabled, decoded, never consumed, mojibake) | `TUI:902,1196-1204,4132`, `DASH:909-912` |
| 1g | Focus events (`?1004`) | missing (parser decodes, never enabled, no consumer) | `TUI:1126-1127` |
| 1h | Synchronized output (`?2026`) | missing | no `2026` |
| 1i | Kitty keyboard / modifyOtherKeys | missing | no `u`/`>4;2m` handling |
| 1j | Autowrap off, DECCKM reset, DECSCUSR reset on enter/leave | missing | `TUI:902,907` |
| 1k | Restore on panic (unwind) | partial (Drop restores; message is lost with the alt screen) | `TUI:4596-4600` |
| 1l | Restore on SIGINT/SIGTERM/SIGHUP/abort/SIGKILL | missing | no handlers; `ISIG` cleared `TUI:4510` |
| 1m | Windows console mode equivalents | partial | `TUI:4704-4740`, `WIN:12-17` |
| 2a | Key decoding: arrows/Home/End/PgUp/PgDn/Ins/Del/F1-F12 + CSI modifiers | implemented | `TUI:1113-1171` |
| 2b | Ctrl/Alt ASCII | implemented (ASCII only; legacy ambiguities) | `TUI:1031-1081` |
| 2c | Alt+non-ASCII, SS3 arrows/Home/End, modified F1-F4, F13+, `:`/`?`/`>` CSI | missing/broken | `TUI:1065-1111` |
| 2d | UTF-8 text input / IME commit | partial (scalar values only, no graphemes) | `TUI:1002-1016` |
| 2e | Esc vs sequence disambiguation | partial (depends on caller poll timeout) | `TUI:993-1000,4565-4567` |
| 2f | Mouse decode (down/up/drag/wheel + modifiers) | implemented with defects | `TUI:1173-1194` |
| 2g | Resize event | missing natively | `TUI:5386` only |
| 3a | Hit test | partial (deepest visible, last child wins, no capture/overlay) | `TUI:2354-2368` |
| 3b | Click -> focus node | partial | `TUI:4096-4107` |
| 3c | Hover state | missing | `Tui` fields `TUI:4010-4018` |
| 3d | Mouse capture during drag | missing | `TUI:4095` |
| 3e | Double-click / click count | missing | `MouseEvent` has no time/count `TUI:2157-2162` |
| 3f | Wheel scrolling per widget | missing | `TUI:4095,4132` |
| 3g | Wizard row click | implemented (activates immediately) | `EL/Wizard:20-28` |
| 3h | Window chip click (tab/new/max/close) | broken for non-active tabs | `TUI:3523-3554` |
| 4a | Focus ring (Tab/Shift+Tab) | partial (includes hidden and non-interactive widgets) | `TUI:3996-4007,4052-4076` |
| 4b | Dashboard focus (`dash.focused`) vs engine focus | broken (two sources of truth) | `DASH:104-112,873-907` |
| 4c | Directional pane navigation | missing | not in `layout` or dashboard |
| 4d | Tab-stack navigation (next/prev tab) | missing (`cycle_stack_tab` exists, unwired) | `TUI:2813`, `DASH:13` |
| 5a | Hardware cursor placement/shape/visibility | missing | `TUI:902`, `Tui` has no cursor |
| 5b | Software cursor in `Input` | partial (block overwrites glyph) | `EL/Input:43-45` |
| 5c | Terminal pane cursor | missing | `TUI:3322-3337,1675-1706` |
| 5d | DECSCUSR / cursor visibility from child | missing / tracked-unused | `TUI:1552,2017` |
| 6a | Mouse text selection | missing | `TUI:3208` never written |
| 6b | Selection highlight | missing | `TUI:3322-3337` |
| 6c | Copy (OSC 52 / native) | dead code | `TUI:4257-4264,4343-4388` |
| 6d | Paste into PTY | missing | `TUI:4132`, `DASH:909-912` |
| 6e | Keyboard copy mode | missing | - |
| 7a | Scrollback keys | partial (steals PgUp/PgDn/End/Ctrl+Home) | `TUI:3286-3304` |
| 7b | Scrollback wheel/scrollbar | missing | - |
| 7c | Alt screen of child | implemented (`?1049` only) | `TUI:2021-2032` |
| 7d | Mouse reporting passthrough | missing (flags tracked, unused) | `TUI:1553-1555` |
| 7e | Key passthrough completeness | broken (see Section 7) | `DASH:44-62` |
| 7f | VT query responses (DSR/DA/OSC/DECRQM) | missing | `TUI:1447-1491` |
| 7g | Resize propagation | partial (only `ReplayComplete`, never on layout change) | `DASH:535,659` |
| 8a | Wizard filter | implemented, O(N) per key/frame | `TUI:3110-3114` |
| 8b | Wizard scrolling/Home/End/PgUp/PgDn/wheel/hover | missing | `EL/Wizard:14-18,30-71` |
| 9a | Split resize by drag | missing (`resize_window` unwired, no gutter) | `TUI:2872` |
| 9b | Tab drag between stacks | missing (`move_window_to_stack` unwired) | `TUI:2918` |
| 9c | Close/maximize/new-tab buttons | implemented with wrong-target bug | `TUI:3523-3554` |
| 9d | Zoom | implemented (keyboard + button), no PTY resize | `TUI:3961-3968`, `DASH:672-680,762-771` |
| 10a | Event-driven wake | missing (polling) | `DASH:651` |
| 10b | Incremental render | broken (dirty bug) | `TUI:2328-2347,4162` |
| 10c | Input/output fairness and budgets | missing | `DASH:506`, `CONN:74` |

---

## 2. Section 1 - Terminal setup/teardown

**Sequences** (`TUI:901-908`):

```
setup:    ESC[?1049h ESC[?25l ESC[?1002h ESC[?1006h ESC[?2004h ESC[2J
teardown: ESC[?2004l ESC[?1006l ESC[?1002l ESC[?25h ESC[?1049l ESC[0m
```

Backend enter/leave: unix `TUI:4505-4543`, windows `TUI:4704-4750`. Both are idempotent and have rollback paths; ownership flags (`entered`, `ansi_setup_owned`, `raw_mode_entered`) are carefully modelled and unit-tested in the `🔬️backend-native-*-unit` includes (`TUI:4593-4594`, `4797-4798`). That part is solid. The sequences themselves are minimal:

| Mode | Set? | Consequence |
| --- | --- | --- |
| `?1049` alt screen | yes | ok |
| `?25l` | yes, never re-shown while running | no hardware cursor (Section 5) |
| `?1000` | no (1002 is a superset on all modern terminals) | ok |
| `?1002` button-event | yes | drags are reported; **host-terminal native selection is disabled** |
| `?1003` any-event | **no** | **no hover is possible**; `MouseKind::Move` (`TUI:2152`) has no producer (`finish_sgr_mouse` maps motion to `Drag(3)`, `TUI:1186-1187`) |
| `?1006` SGR | yes | ok; legacy X10 fallback (`ESC[M`) is not decoded and would leak garbage keys |
| `?1016` pixels | no | not needed yet |
| `?2004` bracketed paste | yes | **paste is then discarded** (Section 6) |
| `?1004` focus | no | `FocusGained/FocusLost` are decoded (`TUI:1126-1127`) but never enabled or consumed (`DASH` has no reference) |
| `?2026` synchronized output | no | each full-frame write can tear; relevant because every paint is a full frame |
| kitty `CSI > 1 u` / `CSI > 4;2 m` | no | Esc vs Ctrl+[, Tab vs Ctrl+I, Enter vs Ctrl+M/J, Shift/Ctrl+Enter, Ctrl+Backspace indistinguishable |
| `?7l` autowrap off | no | benign today (`emit_runs` always repositions), needed once a cursor row is written |
| `?1l` + `ESC >` (DECCKM/DECKPAM reset) | no | **if the outer terminal was left in application-cursor mode, arrows arrive as `ESC O A` and are silently dropped** (`feed_ss3` handles only `P/Q/R/S`, `TUI:1083-1095`) [S] |
| `ESC[0 q` (DECSCUSR reset on leave) | no | cursor style leaks if a child changed it |

Teardown omits `?1000l ?1003l ?1004l ?2026l` (harmless today because they are never set; must be added together with the setters).

**Raw mode (unix, `TUI:4509-4521`)**: clears `ECHO|ICANON|ISIG|IEXTEN`, `IXON|ICRNL|BRKINT|INPCK|ISTRIP`, `OPOST`; `VMIN=0, VTIME=0`. `ISIG` off means Ctrl+C/Ctrl+Z arrive as bytes (0x03/0x1a) and the dashboard never treats Ctrl+C as quit; `CS8` is not forced.

**Panic / abort / signal safety** [S]:
- `impl Drop for NativeTerminal` calls `leave()` (`TUI:4596-4600`, windows `4812-4816`). `Cargo.toml` leaves `panic` at the native default (unwind; the only `panic` mentions are comments about wasm, `Cargo.toml:506-509`), so a panic **does** restore modes on unwind. There is no panic hook, so the default hook prints the message while still on the alternate screen; leaving `?1049` then discards it. A crash is silent and unreadable.
- `abort`, `SIGKILL`, and every signal other than the tty-generated ones leave the terminal in raw mode on the alternate screen with mouse reporting on. `SIGINT` from `kill -INT`, `SIGTERM`, `SIGHUP`, and `SIGQUIT` have no handler (framework-wide grep is empty). The user must run `reset`.
- `eprintln!` inside the raw alternate screen (for example `DASH:646`) scrambles the display.

**Windows** (`TUI:4704-4740`): output `ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN`; input `(orig | ENABLE_VIRTUAL_TERMINAL_INPUT) & !(LINE|ECHO|PROCESSED)`; code page 65001 saved/restored. The ABI file defines no `ENABLE_MOUSE_INPUT`, `ENABLE_QUICK_EDIT_MODE`, `ENABLE_EXTENDED_FLAGS` or `ENABLE_WINDOW_INPUT` (`WIN:12-17`). [E] In Windows Terminal/ConPTY the VT mouse sequences flow once the app sets `?1002/?1006`; in legacy conhost quick-edit stays on (it is part of `original_in`) and swallows mouse clicks. `poll` waits on the console handle and then calls a blocking `ReadFile` (`TUI:4756-4766`); a signalled handle with no byte-producing event (focus/buffer-size event) can block the UI thread. Resize needs polling `GetConsoleScreenBufferInfo` (no VT resize report exists).

**Required**: one shared "terminal mode set" struct (alt screen, mouse level, focus, paste, sync, keyboard protocol, cursor style) generated from a single table so setup, teardown and the parser stay in lock-step; a panic hook that leaves the terminal before printing; async-signal-safe handlers for TERM/HUP/INT/QUIT that set an atomic and wake the loop (restore from the main thread, never inside the handler); SIGWINCH via the same self-pipe; on Windows add `ENABLE_EXTENDED_FLAGS` (clear quick-edit), `ENABLE_MOUSE_INPUT`, `ENABLE_WINDOW_INPUT` and a `CTRL_CLOSE_EVENT` handler.

---

## 3. Section 2 - Input decoding

`Event` (`TUI:2166-2173`): `Key(KeyEvent{key, mods})`, `Mouse(MouseEvent{kind,pos,mods})`, `Paste(String)`, `Resize(Size)`, `FocusGained`, `FocusLost`. `Key` (`TUI:2114-2132`): `Char(char)`, Enter, Esc, Tab, BackTab, Backspace, Delete, Insert, arrows, Home, End, PageUp, PageDown, `F(u8)`. `mods` is a `u8` of SHIFT=1, ALT=2, CTRL=4 only (`TUI:2135-2139`).

**What is dropped**: Meta/Super/Hyper, Caps/Num lock, key release/repeat, shifted-key vs base-key, text of the key (so `Ctrl+Shift+Letter` is lost), time stamp and click count on mouse events, button state on motion, horizontal wheel, and any pixel coordinates.

Decoder behaviour (`AnsiParser`, `TUI:926-1205`; the same parser object is used by the unix backend `TUI:4466,4563`, the windows backend `TUI:4631,4764` and `WasmHost` `TUI:5367`), with proven defects:

| Input bytes | Result | Defect |
| --- | --- | --- |
| `0x0a`, `0x0d` | `Enter` | Ctrl+J == Enter == Ctrl+M (`TUI:1037`) |
| `0x08`, `0x7f` | `Backspace` | Ctrl+H / Ctrl+Backspace indistinguishable (`TUI:1039`) |
| `0x09` | `Tab` | Ctrl+I indistinguishable (`TUI:1038`) |
| `0x00` | `Char(' ')+CTRL` | ok, but the dashboard drops it (Section 7) |
| `0x01..0x1a` | `Char(lowercase)+CTRL` | ok |
| `0x1c..0x1f` | `Char(ctrl-char)`, mods 0 | wrong shape: Ctrl+\\ Ctrl+] Ctrl+^ Ctrl+_ look like plain characters (`TUI:1045`) |
| `ESC x` (ASCII) | `Char(x)+ALT` | `ESC ESC` becomes `Char('\x1b')+ALT`, not two Esc; `ESC \r` becomes `Char('\r')+ALT` (not `Enter+ALT`); `ESC 0x7f` likewise (`TUI:1074-1078`) |
| `ESC` + UTF-8 lead byte | **entire key dropped** | `feed_escape` consumes the lead byte (`_ => Ground`, `TUI:1079`); the continuation bytes then fall to `_ => {}` in `feed_ground` (`TUI:1061`). Alt+ä, Alt+€ etc. vanish [S] |
| `ESC O A/B/C/D/H/F` (SS3) | **dropped** | only `P Q R S` handled (`TUI:1085-1091`); breaks in application-cursor mode |
| `ESC [ 1;5 A` etc. | arrow + mods | ok (`TUI:1133-1136`, `[T]` `TUIT:375`) |
| `ESC [ 1;2 P..S`, F13+ (`25~ 26~ 28~ 29~ 31~..34~`), `CSI u` | dropped | `TUI:1138-1171`, no `u` arm |
| `ESC [ Z` | `BackTab` | modifiers ignored (`TUI:1125`) |
| `ESC [ ? ... c`, `ESC [ > ... `, `:` sub-params | misparsed | `feed_csi` treats `?`, `>`, `=`, `:` and `0x20..0x2f` as the **final byte** (`TUI:1105-1109`), so the tail (`1;2c`) is re-fed as typed characters; any terminal reply or kitty-style sequence injects phantom keystrokes |
| `ESC ] ... ESC \` (OSC with ST) | the `\` leaks as `Char('\\')` | `TUI:1022-1026` ends OSC on the ESC byte and returns to Ground |
| SGR mouse wheel | `b & 0x40`: `btn==0` -> up, **else down** | wheel-left (66) and wheel-right (67) become `ScrollDown` (`TUI:1180-1185`) |
| SGR mouse motion | `b & 0x20` -> `Drag(btn)`; no-button motion is `Drag(3)` | `MouseKind::Move` never produced (`TUI:1186-1187`); buttons 8-11 alias to 0-3 |
| bracketed paste body | each **byte** pushed as `b as char` (`TUI:1197`) | UTF-8 text is double-encoded: pasted `ä` (C3 A4) becomes `Ã¤` [S]; no size cap and no timeout, so a lost `ESC[201~` keeps the parser in `Paste` forever and the keyboard is dead |
| UTF-8 | scalar values as separate `Char` events | no grapheme/ZWJ/flag clustering; invalid continuation bytes silently swallow the next byte (`TUI:1003-1016`) |

**Escape vs sequence timeout**: the parser stores a pending ESC and relies on the caller invoking `flush_escape` when a poll returns nothing (`TUI:993-1000`, unix `4565-4567`, windows `4767-4769`). With the dashboard's 80 ms poll (`DASH:651`) a lone Esc is delivered after up to 80 ms of silence; a sequence split by more than 80 ms (slow SSH) becomes `Esc` plus text. If `select` is interrupted (`ready < 0`) the same `else` branch also flushes a half-received sequence. [S]

**Unix poll quirks** (`TUI:4550-4569`): `select()` with `FD_SET` has undefined behaviour for fd >= `FD_SETSIZE` (1024); `read` returning `0` or `-1` (hang-up, `EIO`) is ignored, so a dead tty returns an empty `Vec` forever; only 4096 bytes are read per call (a 100 KB un-bracketed paste needs 25 loop iterations, each with a full repaint).

**Required**: byte-oriented paste buffer decoded once at the end (`String::from_utf8_lossy`), capped and timed out; a real CSI state machine (private marker, parameters with `:` sub-parameters, intermediates, final byte `0x40..0x7e`); SS3 for all keys; Alt-prefix for any UTF-8 scalar and for Enter/Tab/Backspace/Esc; `MouseKind::Move` and horizontal wheel; resize as an event source; optional kitty keyboard level 1 or modifyOtherKeys 2 so Ctrl/Shift combos and Esc are unambiguous; `flush_escape` driven by a monotonic deadline rather than by "poll returned empty".

---

## 4. Section 3 - Mouse routing in the engine

`Tui::dispatch` (`TUI:4079-4135`) is the whole routing layer:

```
Resize          -> Tui::resize
Key Tab/BackTab -> focus_next / focus_prev            (always, TUI:4083-4084)
Key other       -> focused widget .on_key             (TUI:4085-4093)
Mouse           -> only if kind == Down(_)            (TUI:4095)
                     hit = scene.hit(pos)
                     climb to nearest Widget ancestor -> tui.focus   (TUI:4096-4107)
                     if Down(0) and hit node is a Wizard -> wizard_hit (TUI:4108-4113)
                     climb to nearest Chrome ancestor -> window_hit   (TUI:4114-4128)  [any button]
everything else (Up, Drag, Move, ScrollUp, ScrollDown, Paste, FocusGained/Lost) -> ignored (TUI:4132)
```

**Hit testing** (`TUI:2354-2368`): recursive; a node must be `visible` and its rect must contain the point before its children are considered; children are tried last-to-first, so z-order is child order. There is no `pointer_events`/hit-test opt-out, no overlay root, no modal capture; `Dialog/Popover/ContextMenu` have no TUI target. Rects are those of the last `render()`/`render_full()` (`TUI:4166`), so a second mouse event in the same batch after a `remount` hits stale geometry.

**Hover**: none. `Tui` fields are `scene, theme, size, front, back, focus, full_redraw` (`TUI:4010-4018`); `Node` has no hover flag (`TUI:2212-2221`); `WidgetState::paint` receives only `focused: bool` (`TUI:3390`). [S]

**Capture / drag**: none. `Down` is stateless; `Drag/Up` never reach any code.

**Double-click**: `MouseEvent` has neither time nor click count (`TUI:2157-2162`) and the engine keeps no last-click state.

**Wheel**: dropped for every widget (`TUI:4095`). Terminal scrollback, wizard/list/table/log viewports and the scene have no wheel path.

**Click -> focus**: any widget ancestor becomes `tui.focus` (`TUI:4098-4107`). Clicking window frame, chip row or blank area focuses nothing. `Label/Divider/Chip` are `Widget` nodes and therefore focusable (`TUI:3996-3998`).

**Complete `WidgetSignal` inventory and producers** (`TUI:2988-3001`):

| Signal | Keyboard producer | Mouse producer | Dashboard handling |
| --- | --- | --- | --- |
| `Activated(usize)` | list Enter (`EL/List:29`), table leaf Enter (`EL/Table:53`), wizard Enter (`EL/Wizard:49`) | **wizard row Down(0)** (`EL/Wizard:20-28`, via `TUI:4108-4113`) | handled in both paths (`DASH:705-709`, `901-903`) |
| `SelectionChanged(usize)` | list/table/wizard Up/Down, select Left/Right/Enter, table expand/collapse (`EL/Table:29-55`) | none | **dropped** (`handle_view_signal` `_ => {}`, `DASH:467,470`) |
| `ValueChanged(String)` | input char/backspace (`EL/Input:14-24`), terminal search (`TUI:3266-3283`) | none | dropped |
| `Toggled(bool)` | list Space (`EL/List:22`), terminal Ctrl+P pin (`TUI:3310-3316`) | none | dropped |
| `TabChanged(usize)` | tabs Left/Right (`EL/Tabs:16-25`) | none | not used by the dashboard |
| `WindowClose` / `WindowMaximize` / `WindowNewTab` / `WindowTabActivated(i)` | none | `ChromeState::window_hit` for **any** mouse button (`TUI:4114-4122`, `3523-3554`) | handled (`DASH:666-704`) |
| `NavigateBack` | wizard Backspace on empty filter (`EL/Wizard:55-63`) | none | handled |
| `TerminalPassthrough` | terminal widget fall-through (`TUI:3317`) | none | handled only in terminal-input mode (`DASH:886-891`) |

**Proven defects** (each with a minimal reproduction):

1. *Wrong-target tab controls* [S]. Open two stacked tabs (default layout is tabs). Click the `x` glyph on the **inactive** chip. `build_corner_tab_interior` renders max/new/close for every tab (`TUI:3624-3674`) and `window_hit` returns a bare `WindowClose` for any tab's `close_x` (`TUI:3538-3546`); the signal is tagged with the probed chrome node, which is the visible (active) window (`TUI:4114-4122`); `DASH:666-671` closes that window. The tests only cover single-tab close/max and tab activation (`TUIT:148-177`, `241-255`). Same for maximize and new-tab.
2. *Right and middle clicks trigger chrome controls*: the chrome probe is outside the `Down(0)` guard (`TUI:4108` vs `4114`). Right-click on `x` closes the window.
3. *Clicking a pane does not move keyboard focus*: engine focus changes (`TUI:4105-4107`); `dash.focused` does not. Only `WindowTabActivated`, `WindowNewTab`, row `Activated`/`NavigateBack` assign `dash.focused` (`DASH:687,696,706`). Reproduction: split with Ctrl+B `|`, click inside the left pane's terminal, type: keystrokes go to the pane that `dash.focused` names.
4. *Single click activates*: `wizard_hit` returns `Activated` on the first Down(0) (`EL/Wizard:27`). In the launcher a click on any row spawns a process (`DASH:447-452`). There is no select-then-confirm or double-click.
5. *Wheel in output and lists is silently dropped*; *dragging* inside a pane produces full repaints for nothing: every non-empty event batch sets `need_paint` (`DASH:653`), so each reported drag event costs a full-frame emit.
6. *Stale-geometry hit in a batch*: after `dash.remount` in one event, the next mouse event in the same read uses old rects.

**Required** (engine): per-frame hit map or the existing tree walk with `hit_testable`; `MouseState { hover: Option<NodeId>, capture: Option<NodeId>, press: Option<(NodeId, Pos, Instant)>, last_click: (NodeId, Instant, u8) }`; `?1003h` and coalescing of motion events to one per frame; widget-local mouse API `on_mouse(local, ev) -> Option<WidgetSignal>`; new signals (`Scrolled`, `Hovered`, `Pressed`, `Released`, `DoubleActivated`, drag begin/move/end); chrome signals carrying the tab index (`WindowClose(idx)`, `WindowMaximize(idx)`); left-button-only chrome activation; modifier-aware bypass.

---

## 5. Section 4 - Focus model

Engine focus is one `Option<NodeId>` (`TUI:4016`). `focus_next/prev` cycle `dfs_focusables`, which returns **every** `Widget` node in tree order, visible or not (`TUI:3996-4007`): hidden stack tabs (`visible=false`, `TUI:3929`), `Label`, `Divider`, `Chip`. There is no focus scope per window, no `tabindex`, no roving focus, no `FocusChanged` notification, and `set_focus` accepts any id without validation. After `scene.remove`, `focus` can dangle; `node_raw_mut` indexes the slot and `expect`s (`TUI:2316-2322`) without comparing generation, so a stale id panics or, if the slot was reused, silently addresses another node. [S]

The engine claims Tab/Shift+Tab unconditionally before the focused widget sees them (`TUI:4083-4084`); a terminal widget can therefore never receive Tab through `dispatch`.

**Dashboard model**: `dash.focused: String` is the source of truth for keys (`DASH:873-907`); `sync_chrome_focus` mirrors it into `WindowState.focused` for the border colour (`DASH:104-112`); the engine's `tui.focus` is re-assigned from it before each key (`DASH:885`, `897`) and by Tab handling (`DASH:867`). Desync cases (all [S]):

| Trigger | `dash.focused` | `tui.focus` | Effect |
| --- | --- | --- | --- |
| click in another pane's terminal or wizard blank area | unchanged | clicked widget | keys go to old pane; the clicked wizard shows a selected-row highlight until the next key |
| click on a row in another pane | set (`DASH:706`) | set | ok |
| `close_window` | `order[0]` (`DASH:419-420`) | first window's widget (`DASH:422`) | focus jumps to the first window, not the neighbour; `activate_stack_tab` is not called, so the focused window can be a hidden tab (keys go to an invisible widget); `terminal_input` is flipped on if window 0 is an output (`DASH:421`) |
| zoom (`Ctrl+B z`, maximize button) | unchanged | unchanged | non-zoomed windows stay `visible=true` beneath the zoomed one (`TUI:3961-3968` returns before touching them) and remain in the focus ring and hit tree |
| `WindowNewTab` | new id | new widget | ok, but `terminal_input` forced false |

**Tab order**: the dashboard's Tab cycles `window_order()` = creation order of `self.windows`, not spatial order (`DASH:858-871`, `100-102`).

**Directional navigation between split panes**: missing everywhere. No `layout::neighbor(rect, dir)`, no leader binding (the Armed table at `DASH:722-838` has no arrows).

**Within tab stacks**: `layout::cycle_stack_tab` exists (`TUI:2813-2824`) and is unit-tested (`TUIT:888-912`) but is not imported (`DASH:13`); the only ways to change tab are clicking a chip or Tab-cycling every window.

**Required**: a focus tree (window -> widget) owned by the engine with the dashboard reading it; `focus_dir(Left|Right|Up|Down)` over the solved window rects; visible-only, interactive-only ring; focus events; click-to-focus of the window (not just the widget) including terminal panes; closing selects the previously focused window or the spatial neighbour and activates its stack tab.

---

## 6. Section 5 - Cursor

- Hardware cursor: **hidden on entry (`TUI:902`) and never shown again.** `Tui` has no cursor field (`TUI:4010-4018`), `AnsiPatch` is a plain string (`TUI:846`), `emit_runs` emits no cursor move or `?25h` after the runs (`TUI:882-898`), and `Tui::render` returns the patch unchanged (`TUI:4185`). No code path emits `?25h` except teardown.
- `Input` widget: a software block `U+2588` in the accent colour is `put` over the cell at the cursor (`EL/Input:43-45`), **replacing the glyph** under it (a mid-string cursor erases a character visually); the offset is computed from byte index `i.value[..i.cursor]` and `display_width`, so wide characters and grapheme clusters are mispositioned.
- `Wizard` filter row (`EL/Wizard:85-91`): plain text `"/ {filter}"`, **no caret at all** and no left/right/Home/End editing; the filter only ever appends or pops the last char.
- Terminal pane: `paint_terminal` (`TUI:3322-3337`) calls `screen.blit_to` (`TUI:1675-1706`), which copies cells only. `VtScreen.cursor` (`TUI:1543`) and `cursor_visible` (`TUI:1552`, set at `TUI:2017`) are never read outside `vt`. The PTY app's cursor is invisible, so a shell prompt or an editor gives no feedback where typing lands. `DECSCUSR` (`CSI Ps SP q`) arrives as intermediate `0x20` + final `q`, falls through `finish_csi` `_ => {}` (`TUI:1489`) and is discarded.
- Focus transitions: nothing hides or shows a cursor when focus moves.

**Required**: `Tui::cursor: Option<CursorState{pos, shape, blink, visible}>` computed in paint from the focused widget (`WidgetState::cursor(rect, focused) -> Option<CursorSpec>`; terminal returns `screen.cursor` offset by the pane rect, scrollback offset and `cursor_visible`, shape from DECSCUSR state); `AnsiPatch` epilogue `ESC[y;xH` + `ESC[n q` + `?25h`/`?25l`; hide at frame start and show at the end inside a `?2026` bracket; `Input` should use the hardware cursor and char-boundary-aware movement.

---

## 7. Section 6 - Selection and clipboard

| Piece | Status | Anchor |
| --- | --- | --- |
| `TerminalSelection{start,end}` (viewport coordinates, inclusive) | type exists | `TUI:3195-3198` |
| `TerminalState.selection` | field is written only as `None` in `new` | `TUI:3208,3214` |
| `TerminalState::selected_text` | row-major slice of the **active buffer only**, ignores `scrollback_offset`, no wrap joining, no trailing-space trim, no word/line modes | `TUI:3242-3261`; callers: none |
| Selection highlight | not painted | `TUI:3322-3337` |
| Mouse drag selection / word / line / block | missing | engine drops Drag/Up (`TUI:4095`) |
| Selection in lists/wizard | missing | - |
| OSC 52 copy | `osc52_copy_sequence` builds `ESC]52;c;<b64>BEL` (`TUI:4257-4264`); `[T]` `TUIT:1483-1500`; **no consumer** | - |
| Native clipboard | `Clipboard` trait + `HostClipboard` (pbcopy/wl-copy/xclip/xsel/`clip`, pbpaste/Get-Clipboard) on the worker-pool I/O lane, `MemoryClipboard` (`TUI:4225-4410`); **no consumer in `Tui`, `DASH`, or any other crate** (grep) | - |
| Bracketed paste -> PTY | missing (Section 3, `Event::Paste` has one producer `TUI:1201`, zero consumers) | `TUI:4132`, `DASH:909-912` |
| Keyboard copy mode | missing | - |

Consequence: with `?1002h` on, the host terminal's own click-drag selection is disabled (users must hold a terminal-specific bypass modifier), and the TUI has nothing in its place, so **text in any pane cannot be selected or copied** unless the user knows the bypass. Over SSH `pbcopy`/`xclip` would target the remote machine; OSC 52 is the correct remote path, native tools the correct local path, and the host should choose.

**Required**: `SelectionModel{ anchor, extent, mode: Cell|Word|Line|Block, space: AbsoluteRow }` in absolute scrollback coordinates (survives scrolling and resize); drag auto-scroll; `Tui::selection_text(node)` joining wrapped rows (needs a per-row `wrapped` flag in `VtScreen`), trimming trailing blanks, skipping `\0` continuation cells; highlight via `attr::REVERSE`/theme role in paint; copy command -> `Clipboard` + OSC 52 epilogue; paste command -> `on_paste`; modifier-aware passthrough (Shift+drag always selects locally even when the child has mouse reporting, matching other emulators).

---

## 8. Section 7 - Embedded terminal widget

`TerminalState` (`TUI:3201-3262`): `screen: VtScreen`, `scrollback_offset` (rows above the live bottom), `follow`, `pinned`, `search`, `search_active`, `selection`. Input goes `DASH:886` -> `Tui::dispatch` -> `terminal_on_key` (`TUI:3264-3320`) -> `TerminalPassthrough` -> `key_to_pty_bytes` (`DASH:44-62`) -> `ClientMsg::Input` (`DASH:888-889`, `try_send` into a 64-slot channel, `CONN:86-89`) -> daemon `Pty::write_all` on a **non-blocking** master (`TUI:4893`, `5042-5049`).

### 7.1 Key reachability (complete table, terminal-input mode)

Intercepted **before** the PTY (never forwarded):

| Key | Interceptor | Anchor |
| --- | --- | --- |
| Ctrl+B | leader arm; pressing it again re-arms, so a literal Ctrl+B cannot be sent | `DASH:716-720` (precedes the armed check at `721`) |
| Ctrl+W | close window | `DASH:849-856` |
| Tab, Shift+Tab | window cycling, before the terminal-input branch | `DASH:858-871` (`key_to_pty_bytes` has a Tab arm at `DASH:49` that is therefore unreachable) |
| Esc | leaves terminal-input mode | `DASH:876-880` (arm at `DASH:51` unreachable) |
| PageUp, PageDown (any mods) | scrollback | `TUI:3286-3294` |
| End (any mods) | jump to live bottom | `TUI:3300-3304` |
| Ctrl+Home | oldest scrollback | `TUI:3295-3299` |
| `/` (no mods) | enters search mode | `TUI:3305-3309` |
| Ctrl+P | toggles `pinned` (a no-op, see 7.4) | `TUI:3310-3316` |
| every printable key after `/` | consumed by the search buffer | `TUI:3266-3283` |

Reach `key_to_pty_bytes` but are **dropped** (returns `None`, `DASH:60`): Delete, Insert, F1..F12+, Shift+Tab (also intercepted), **all Alt+key** (should be `ESC` + key), Ctrl+Space (NUL; the parser yields `Char(' ')+CTRL`, `TUI:1040`, which fails `c.is_ascii_lowercase()`, `DASH:47`).

Forwarded with **lost information**: Up/Down/Left/Right/Home/End/PgUp/PgDn always as unmodified CSI (`DASH:52-59`), discarding Ctrl/Alt/Shift (word-jump Ctrl+Left, Shift selection in editors); no DECCKM awareness (`decset(1, _)` is `=> {}`, `TUI:2011`) so apps that enabled application cursor keys receive the wrong form; no DECKPAM; Enter always `\r`, so Ctrl+J (parsed as Enter) arrives as `\r` not `\n`.

Forwarded correctly: printable and UTF-8 text (`DASH:46`), Ctrl+letter (`DASH:47`), Enter, Backspace(0x7f), Ctrl+C/D/Z etc., Ctrl+\\ and friends by accident (`Char(0x1c)` with mods 0 passes the first arm).

Non-terminal-input mode: keys for output windows are ignored entirely except q (detach), Ctrl+B, Ctrl+W, Tab (`DASH:845-871`, no branch for output windows after `895`).

### 7.2 Proven trap: search mode

Type `/` while in terminal-input mode (any shell command with a path). The widget sets `search_active` and swallows the `/` and every subsequent printable key, Backspace and Enter (`TUI:3266-3283`, `3305-3309`). The only exit is `Esc` inside the widget (`TUI:3268-3272`), but the dashboard intercepts Esc first and flips `terminal_input` off instead (`DASH:876-880`), so the widget never receives it; re-entering input mode (`Ctrl+B t`) returns to the same swallowing state. The matcher does not exist (`search` is only displayed, `TUI:3329-3335`); `Enter` merely re-emits `ValueChanged`. The search row also steals the last pane row (`TUI:3329`). [S] The only reset is a fresh `TerminalState::new` on reconnect (`DASH:518`) or closing the window. The unit test asserts the widget-level Esc exit (`TUIT:1129-1146`) and misses the dashboard interaction.

### 7.3 Mouse, paste, focus, queries

- Mouse passthrough: `mouse_tracking/mouse_button_event/mouse_sgr` are set by `decset` (`TUI:2018-2020`) and **never read** (grep over `TUI` and `DASH`). Click, drag and wheel inside a pane are neither forwarded nor used locally; `less`, `htop`, `vim` mouse modes do not work. Needed: encode SGR (`ESC[<b;x;yM/m`) or legacy X10 when 1006 is off, translate to pane-local 1-based cells, honour 1000/1002/1003, forward focus (`?1004`) and wheel; if the child is on the alternate screen with mouse off, translate wheel to arrow keys (alternate-scroll `?1007`).
- Bracketed paste: `bracketed_paste` is tracked (`TUI:2033`) but unused; neither the outer `Event::Paste` nor the child's mode is honoured.
- Child queries: `VtScreen` has no response channel. `finish_csi` ignores DSR (`CSI 5n`/`6n`), DA1/DA2 (`CSI c`, `CSI > c`), DECRQM, window ops, OSC 10/11 colour queries, XTVERSION, kitty keyboard queries (`TUI:1447-1491`, `feed_osc` stores only title `TUI:1512-1521`). Children that query (cursor position, colours, device attributes; typical of TUI frameworks, `fzf --height`, `vim`) wait for a timeout or misdetect capabilities [E: app behaviour not run here]. Responses must be routed back through `ClientMsg::Input`, and exactly one attached dashboard may answer (several dashboards can attach to one daemon, `DASH:381`).
- Title: OSC 0/2 is stored (`TUI:1517-1519`) and never used; the dashboard sets titles itself (`DASH:357`).
- Environment: `Pty::spawn` passes no `TERM` (`TUI:4901-4906`, `DMN:692`); the child inherits the daemon's `TERM` (whatever terminal started the daemon), advertising capabilities the VT does not implement.

### 7.4 Scrollback and alternate screen

- Keys only (PgUp/PgDn page = pane height, Ctrl+Home, End). No wheel, no scroll bar, no half-page, no line step.
- Offset is measured from the live bottom (`TUI:1681-1686`), and nothing compensates when rows are pushed (`TUI:3218-3223`): while reading history, each new output line shifts the view toward newer content. `pinned` therefore cannot pin anything (an offset from the bottom is already independent of `pinned`) and its toggle (Ctrl+P) is invisible. At the 8000-line cap `pop_front` shifts all indices again.
- `max_offset` is the primary scrollback length even while the child is on the alternate screen (`TUI:3230-3232`, `1681`): PageUp in `vim`/`less` scrolls the UI into the shell's old history instead of reaching the app.
- Alternate screen: handled for `?1049` only (`TUI:2021-2032`); `?47/?1047` are ignored; scrollback is not recorded while alt (`TUI:1709`) which is correct.
- Resize: `VtScreen::resize` copies the top-left rectangle of both buffers and clamps the cursor (`TUI:1621-1650`): no reflow, shrinking height cuts the **bottom** rows (where the prompt lives) instead of pushing them to scrollback; scrollback rows keep their original width.

### 7.5 Resize propagation

`resize_terminals` (`DASH:550-566`) is called from exactly two places: `Event::Resize` (never generated, `DASH:659`) and `ReplayComplete` (`DASH:535`). It is **not** called after split (`DASH:772-797`), zoom (`DASH:672-680`, `762-771`), close (`DASH:402-425`), new window (`DASH:817-829`, `681-692`), layout-preference placement (`DASH:146-151`) or `show_home`. After any of these the widget's rect changes but `screen.size` and the PTY winsize do not; the child keeps painting for the old geometry and `blit_to` draws only the intersection (`TUI:1675-1706`). Initial sizing is a guess: `open_output` uses window rect `width-4 x height-4` (`DASH:237-238`) while the window content padding is `[top 3, 1, bottom 1, 1]` (`TUI:3841-3856`), i.e. the real widget is `width-2 x height-4`: 2 columns narrower than available, persistently.

---

## 9. Section 8 - Wizard and list interaction (44k options)

Behaviour of `WizardState` (`TUI:3096-3115`, `EL/Wizard`):

| Aspect | Finding |
| --- | --- |
| Filter | all-words, case-insensitive substring over `to_lowercase()` of every option: `visible_indices()` (`TUI:3110-3114`). The lowercase `String` is allocated for every option **before** the (possibly empty) token test. No cache, no incremental narrowing, no match highlighting, no ranking. |
| Calls per keypress | `wizard_on_key` computes `visible_indices` first for **every** key, including characters whose result is unused (`EL/Wizard:31`); then `render_full` -> `paint_wizard` computes it again (`EL/Wizard:75`). That is 2 full passes (~44k `String` allocations each) per keypress, and 1 per frame. `wizard_hit` adds one more per click (`EL/Wizard:25`). |
| `refresh_view_for` (`DASH:118-137`) | for every window: clones all 44k labels (`DASH:120`), then 2 more passes (`DASH:132,134`), then `offset = 0` (`DASH:135`). Triggered by every preference save (`174`), every inventory `Ready` (`186`), **every session status change** (`update_session` -> `361`) and every `Sessions` message (`523`). N launcher windows multiply it. |
| Cost estimate [E] | O(N·L) allocation per pass; unmeasured. The workspace `dev` profile has no `opt-level` override (`Cargo.toml:342-344`), so an unoptimised dashboard is plausible and each pass is a multiple of the release figure. Measure before tuning; the structure alone justifies a pre-lowercased cache. |
| Selection identity | `SelectionChanged(selected)` is a **visible position**, `Activated(i)` is an **option index** (`EL/Wizard:36,44,49,27`). |
| `refresh_view_for` selection restore | by label equality (`DASH:132-134`): duplicate labels jump to the first duplicate; a session whose label changes (status text is in the label, `DASH:123`) resets the selection to row 0. |
| Viewport | `viewport()` is a pure function with `w.offset` never written (`EL/Wizard:14-18`): `offset = min(max(w.offset, selected-(h-1)), selected)` collapses to `selected-(h-1)`. Once `selected >= h-1` the highlighted row is always the **last** visible row; Up scrolls the content under a fixed highlight and the rows below the selection can never be seen. Pointer vectors in `🧫️fixtures/🔎️launcher/🔣️.json` encode this behaviour and are shared with other renderers, so changing it changes the fixture and parity. |
| Keys | Up, Down, Enter, Esc (clear filter), Backspace (pop or `NavigateBack`), plain chars. **Missing**: PgUp/PgDn/Home/End, Ctrl+U/W, Left/Right inside the filter, wrap-around, Alt/Ctrl combos (`Char` only when `mods == 0`, `EL/Wizard:64`). Backspace key-repeat clears the filter then immediately leaves the launcher (`EL/Wizard:55-63`). |
| Mouse | Down(0) on a row activates at once (`EL/Wizard:20-28`); no hover, no wheel, no scroll bar, no double-click, no right-click. |
| Cursor | none (Section 5). |
| Paste into filter | dropped (`Event::Paste` unconsumed). |
| Dashboard hijack | `q` quits when the focused window is Overview or Settings, whose wizard accepts typed filters (`DASH:845`). |

`List` (`EL/List:12-32`): Up/Down/Space/Enter only; **`offset` is never updated**, so selection can leave the visible rows (`paint_list` uses `l.offset`, `EL/List:34-49`). `Table` (`EL/Table:14-58`): keyboard only; `rows[t.selected]` is indexed unchecked in Right/Left/Enter (`EL/Table:30,39,48`) and panics if `rows` shrank below `selected`. `Log` (`EL/Log:14-34`): first PageUp from Follow produces the same view (`At(len-1)` shows the tail), Home shows one line (`At(0)`), no wheel.

**Required**: `ListModel` abstraction (`len`, `key(i)`, `label(i)`) shared by List/Wizard/Table; cached lowercase index built once per options update; incremental filtering that narrows the previous result when the query extends; a persisted `ViewportState{offset, selected}` with `scroll_to(selected)` minimal-scroll semantics, wheel, Home/End/PgUp/PgDn; selection by stable key; click = select, double-click or Enter = activate; hover row; scroll bar; match highlighting; `WizardState::set_options` that preserves selection by key.

---

## 10. Section 9 - Window and pane interaction

| Operation | Layout API | TUI hit/gesture | Dashboard wiring |
| --- | --- | --- | --- |
| Split | `split_window` `TUI:2827` | none | `Ctrl+B -` / `\|` (`DASH:772-797`); not discoverable (footer list `DASH:570` omits `z - \| x t`) |
| Resize split | `resize_window(layout,id,delta)` `TUI:2872-2915` (adjusts weights of the nearest direct sibling) | none; windows have no gutter between them (`solve_axis` tiles edge to edge, `TUI:2652-2667`), so there is no splitter hit target | **not imported** (`DASH:13`); no key, no drag |
| Move window to another stack | `move_window_to_stack` `TUI:2918-2944` | no tab drag/drop | not imported |
| Activate tab | `activate_stack_tab` `TUI:2801` | chip click -> `WindowTabActivated(i)` | wired (`DASH:693-704`, `stack_tab_window_id` `588-614`) |
| Cycle tabs | `cycle_stack_tab` `TUI:2813` | none | not imported |
| New tab | `push_window_to_stack` `TUI:2952` | `new_x` glyph -> `WindowNewTab` | wired (`DASH:681-692`) |
| Close | `remove_window` `TUI:2947` | `close_x` glyph | wired (`DASH:666-671`), wrong target on inactive chips (Section 4) |
| Maximize | `zoom_window` `TUI:2796` | `maximize_x` glyph | wired (`DASH:672-680`); no PTY resize; other windows stay visible under the zoomed one (`TUI:3961-3968`) |

Hit areas (`TUI:3523-3554`): the controls are single cells at `close_x/maximize_x/new_x`; they are tested only on the chip text row (`rect.y+1` top, `rect.y+height-2` bottom); the tab label region is `tab.x < pos.x < tab.x+interior_width+1`. There is no hover feedback, no tooltip, no minimum hit slop, no double-click on the chip row to maximize, no middle-click close.

`mount_stack` labels chips with `window_kind_id` (`TUI:3923`); the dashboard rewrites them from window titles after every `remount` (`DASH:364-377`) as a workaround. Titles are the long `[status detail] cmd args` string (`DASH:357`), not a task name, and OSC titles are unused. (Rendering agent's domain; listed because "tabs named after tasks" depends on it.)

**Required**: gutter or border hit zones with a `SplitterDragged{path, delta_cells}` signal and `resize_window` driven by cell deltas; chip drag with drop targets (`TabDropped{window, target_stack, index}` / `SplitDropped{edge}`) feeding `move_window_to_stack`/`split_window`; keyboard equivalents (leader + arrows to focus, leader + Shift+arrows to resize, leader + `[`/`]` or Alt+n to cycle tabs); chip signals carrying the tab index; mandatory `resize_terminals` after every layout mutation.

---

## 11. Section 10 - Event loop quality (`DASH:649-931`)

Per iteration: (1) `poll_sessions | poll_inventory` (non-blocking drain, `DASH:650`), (2) `term.poll(80 ms)` (`DASH:651`), (3) handle events, (4) sync chrome + rebuild footer text/hints every iteration (`DASH:919-927`), (5) if `need_paint`, `term.present(&tui.render_full())` (`DASH:929`).

- **Latency**: the only wake source is the TTY (`select` on one fd, `TUI:4551-4556`). The daemon connection thread sleeps 10 ms per cycle and reads at most 16 KiB per cycle (`CONN:42-77`); output is delivered through an `mpsc` channel that cannot wake the UI. Keystroke echo therefore waits for the next poll timeout: up to 80 ms (mean ~40 ms) + up to 10 ms on each connection hop. Typing is visibly batched. Idle cost is a 100 Hz connection-thread wake-up plus a 12.5 Hz UI loop.
- **Throughput cap** [E from `CONN:42-77`]: one 16 KiB read + one 10 ms sleep per cycle bounds the dashboard connection near 1.6 MB/s of output.
- **Backpressure = disconnect**: `messages.try_send` into a 256-slot channel; overflow ends the connection with "dashboard view output backlog exceeded" (`CONN:74-79`). The UI drains at most 256 messages (1 + `take(255)`, `CONN:93-98`) per iteration of >= 80 ms when idle of input, i.e. ~3200 messages/s; the connection thread can enqueue 64 frames per 10 ms. A burst of small frames can therefore trip the limit [E]; the failure mode (drop the link, reconnect, replay) is user-visible.
- **Input starvation under output**: drain size is unbounded in time. Up to 256 frames of up to 16 KiB are parsed byte by byte into the VT (`DASH:526-534`, `TUI:1323-1327`) before any input is read, then a full frame is emitted. No time budget, no "input first" rule, no frame pacing.
- **`render_full` on every paint** (`TUI:4189-4192`): resets `full_redraw`, re-solves the whole layout (`TUI:4166`), repaints every node, emits **every row** via `emit_runs` (`TUI:4170-4178`) with a full `38;2;r;g;b;48;2;r;g;b` SGR per colour change, clones the entire front buffer (`TUI:4184`), and writes it with one blocking `write_all` to `/dev/tty` (`TUI:4546`). There is no `?2026` bracket and no frame skipping when the terminal is slow. The diffing path `render()` (`TUI:4161-4186`) is dead in the dashboard and **cannot be used until the dirty bug is fixed**:
  - `Node::new` starts with `dirty = LAYOUT|PAINT` (`TUI:2225`).
  - `take_dirty` clears only the **root** (the single call is `TUI:4162`).
  - `mark_dirty` stops at the first node whose bits are already set (`TUI:2335-2337`).
  - Therefore after the first frame every non-root node stays "dirty" forever, so any later mutation (`set_text`, `widget()`, `chrome()`, `add`, `remove`, `reparent` below the root) stops at that node and never reaches the root; `render()` returns an empty patch. The only test, `tui_render_skips_repaint_when_nothing_dirty` (`TUIT:1441-1447`), checks the no-mutation case; `wasm_host_feed_and_render_smoke` (`TUIT:455-460`) renders only the first frame. `WasmHost::render` (`TUI:5389`) and any future hover/cursor-blink repaint share the bug. [S]
- **Blocking calls on the UI thread**: `shutdown_daemon` waits up to 5 s in 100 ms slices (`DASH:313-325`, called from `DASH:757-759`) with no repaint and no cancel; `open_output` (`DASH:236`) and `update_session` (`DASH:342`) each run a full `render_full()` (layout + paint + diff + clone + string build, result discarded) just to learn a rect, and `update_session` is invoked per session on restore; `graph_is_current` does a `metadata` call per manifest plus a JSON read on the click path (`DASH:449`, `📚️inventory/🦀️.rs:48-54`) [E on count]; `term.present` blocks when the terminal back-pressures.
- **Resize**: not handled natively (D01); `Event::Resize` path exists but is never fed.
- **Errors swallowed**: `term.poll(...).unwrap_or_default()` (`DASH:651`) and `term.present(...).ok()` (`DASH:645`, `929`) hide terminal failure; a lost tty yields a hot loop or a stuck UI without exit.
- **Wasted work every iteration**: footer `format!` and `hints()` allocations, `sync_chrome_focus` (each `chrome()` call marks paint-dirty), `need_paint` set for every event including drags and focus events (`DASH:653`).

**Required**: a single-reader loop with a wake handle (self-pipe/eventfd or `CFRunLoop`-free `poll(2)` over tty + wake fd; `WaitForMultipleObjects` on Windows) so connection, inventory and preference threads wake the UI immediately; fixed frame budget (render at most every ~8-16 ms, coalesce); drain with a time/byte budget and "input first" ordering; explicit flow control (pause reading a session or ask the daemon to throttle) instead of disconnecting; fix dirty tracking so `render()` is the default and `render_full` is the exception; wrap every frame in `?2026`; move shutdown-wait and rect computation off the hot path (layout query API `Tui::layout()` that does not paint).

---

## 12. Prioritized defect list

P0 = unusable or blocks the "full-blown TUI" goal. Each row: id, evidence, minimal reproduction, fix direction.

### P0

| Id | Defect | Evidence | Minimal reproduction | Fix direction |
| --- | --- | --- | --- | --- |
| D01 | Terminal resize never detected | `DASH:631` (only `size()` call), `DASH:657-661` (dead handler), `TUI:5386` (only `Event::Resize` constructor), `TUI:4550-4569`, `4756-4771` | start dashboard, resize the terminal window | poll `size()` every loop + SIGWINCH self-pipe (unix) / console-info poll (windows) -> `Event::Resize`; then `resize_terminals` |
| D02 | Echo/output latency up to 80 ms; wake-less loop | `DASH:651`, `TUI:4551-4556`, `CONN:42-77` | type in a shell pane; echoes arrive in ~80 ms steps | unified wake handle, frame pacing |
| D03 | No visible cursor anywhere (terminal pane, wizard filter); hardware cursor hidden permanently | `TUI:902`, `3322-3337`, `1675-1706`, `4010-4018`, `EL/Wizard:85-91` | open any task, type | `Tui` cursor API + patch epilogue + DECSCUSR |
| D04 | Key forwarding hijacked and incomplete | `DASH:716-720,849-880`, `TUI:3286-3317`, `DASH:44-62` | in a shell pane press Tab (window switches), Esc (leaves mode), Alt+b, Delete, F1, Ctrl+Left, End, PageUp | move encoding into `vt::encode_key(KeyEvent, ModeState)`; reserve only a prefix key (Ctrl+B, doubled to send literally); give scrollback its own mode |
| D05 | `/` search trap swallows all terminal input; unreachable exit | `TUI:3305-3309,3266-3283`, `DASH:876-880` | `cd /usr`: after `/` nothing reaches the shell, Esc cannot exit | remove the stub or implement real search; never intercept printable keys in passthrough mode |
| D06 | Paste discarded; mojibake if consumed; no cap/timeout | `TUI:902,1196-1204`, `4132`, `DASH:909-912` | paste any text (terminal sends `ESC[200~..201~`) | decode bytes once, route `Event::Paste` to focused widget, wrap with `200~/201~` iff child enabled `?2004`, chunked write with backpressure |
| D07 | Mouse is half-wired: only Down; no wheel/drag/hover/selection; click does not focus pane; `?1002` removes native selection | `TUI:4094-4132`, `DASH:662-714`, `TUI:902` | wheel over output; click into another pane and type; try to select text | engine mouse state machine, widget `on_mouse`, selection model, `?1003`, bypass modifier |
| D08 | Dirty propagation bug makes `Tui::render()` unusable | `TUI:2225,2328-2347,4162` | mutate a node after first frame, call `render()` -> empty patch | clear dirty during the layout/paint walk or use a frame epoch; add a mutate-then-render test; then drive hover/cursor with diffs |

### P1

| Id | Defect | Evidence | Fix direction |
| --- | --- | --- | --- |
| D09 | Chip close/max/new act on the active window even when an inactive chip is clicked; right/middle clicks also fire | `TUI:3538-3546,4108-4122`, `DASH:666-692` | signals carry tab index; gate on left button |
| D10 | `dash.focused` vs `tui.focus` split brain; click never focuses a pane; `close_window` focuses window 0 and may pick a hidden tab | `DASH:104-112,419-422,873-907`, `TUI:4105-4107` | engine-owned focus tree; dashboard reads it; activate stack tab on focus change |
| D11 | Wizard O(N) per key and frame (>= 2 passes of 44k allocations per keypress); `refresh_views` clones 44k labels per window on every session change | `TUI:3110-3114`, `EL/Wizard:31,75`, `DASH:118-137,174,186,361,523` | cached lowercase index, incremental filter, key-stable selection, refresh only on options change |
| D12 | Wizard/List viewport pinned to bottom; offset never persisted; no Home/End/PgUp/PgDn/wheel/hover; single click launches a command; list `offset` never updated | `EL/Wizard:14-28`, `EL/List:12-49`, `DASH:447-452` | persisted viewport, select-then-activate, scroll API |
| D13 | PTY/VT size not updated after split/zoom/close/new; initial size guess is 2 columns short; no reflow | `DASH:535,659,237-238`, `TUI:1621-1650,3841-3856` | `resize_terminals` after every remount; derive size from the widget rect; reflow wrapped rows |
| D14 | No VT query responses; DECCKM/DECKPAM/1003/1004/1007 ignored; child mouse/paste/focus modes tracked but unused | `TUI:1447-1491,2009-2036,1552-1556` | response queue drained by the host, mode flags, encoder (Section 7.3) |
| D15 | Scrollback: wheel absent, view drifts on new output, `pinned` no-op, alt screen scrolls primary history | `TUI:3230-3239,3218-3223,1681-1686` | anchor to absolute row; per-buffer scrollback; wheel |
| D16 | Selection, copy and paste unimplemented; clipboard/OSC 52 dead code | `TUI:3195-3261,4225-4410,4257-4264` | Section 6 design |
| D17 | Backpressure disconnects; throughput capped; unbounded drain starves input | `CONN:42-79,86-100`, `DASH:506,526-534` | flow control, budgets (Section 10) |
| D18 | Blocking UI thread: `shutdown_daemon` 5 s, `render_full` in spawn/update, FS stat storm on click, blocking tty write | `DASH:313-325,236,342,449`, `TUI:4546` | async shutdown with progress/cancel, layout query without paint, pre-compute, bounded writes |
| D19 | Full-frame emit per paint, no `?2026`, no pacing | `TUI:4170-4178,4189-4192`, `DASH:928-930` | fixed `render()`, sync bracket, frame budget |
| D20 | No panic hook; no signal restore | `TUI:4596-4600`, framework-wide grep | hook + signal self-pipe |
| D21 | `Input` widget cursor steps by byte, so Left then typing on non-ASCII text panics at `String::insert`; `Char` ignores modifiers (Ctrl+A inserts "a") | `EL/Input:14-33`; `[T]` `TUIT:956-991` asserts the byte step | char-boundary movement, modifier filter, grapheme-aware cursor |
| D22 | Parser gaps listed in Section 2 (SS3 arrows, Alt+non-ASCII, `:`/`?`/`>` CSI, wheel-left/right, F13+, OSC ST leak, Esc-timeout dependence) | `TUI:1065-1111,1180-1185,1022-1026` | Section 2 required list |
| D23 | PTY master is non-blocking and `write_all` does not retry `EAGAIN`: large inputs are truncated | `TUI:4893`, `5042-5049`, `DMN:709-710` | loop on `WouldBlock` with `poll(POLLOUT)` or queue per session |
| D24 | `q` quits while typing in Overview/Settings filters; Ctrl+B conflicts with tmux default prefix | `DASH:845`, `716` | context-sensitive command keys; configurable prefix |
| D25 | Engine focus ring includes hidden and non-interactive widgets and can dangle after `remove` | `TUI:3996-4007,2316-2322,4048` | visible/interactive filter; validate generation; clear focus on remove |

### P2

| Id | Defect | Evidence |
| --- | --- | --- |
| D26 | Windows: no quick-edit/mouse-input/window-input flags; `ReadFile` after wait may block; no resize | `WIN:12-17`, `TUI:4713-4714,4756-4766` |
| D27 | unix `select` FD_SETSIZE; read error/EOF ignored | `TUI:4551-4564` |
| D28 | `Table` panics on stale selection | `EL/Table:30,39,48` |
| D29 | `Log` first PageUp no-op, Home shows a single line | `EL/Log:17-31` |
| D30 | Armed-leader hints omit `z - \| x t` | `DASH:570` |
| D31 | `TERM`/`COLORTERM` not set for children | `TUI:4901-4906`, `DMN:692` |
| D32 | Zoom leaves other windows visible under the zoomed window and in the ring | `TUI:3961-3968` |
| D33 | Wizard Backspace repeat leaves the launcher | `EL/Wizard:55-63` |
| D34 | `SelectionChanged` position vs `Activated` option index inconsistency | `EL/Wizard:36,49` |
| D35 | Scrollback rows are not reflowed on width change | `TUI:1621-1650` |
| D36 | Mouse X10 legacy fallback not decoded; high buttons alias | `TUI:1173-1194` |
| D37 | Hit-test uses stale rects within one event batch | `TUI:4166`, `DASH:655` |

---

## 13. Framework API additions the dashboard needs

1. **Events**: `MouseEvent{ kind, pos, mods, buttons: u8, at: Instant, clicks: u8 }`; `MouseKind::{Down, Up, Drag, Move, Scroll{dx,dy}}`; `Event::Resize`, `Focus` driven by real sources; `Event::Paste(Vec<u8>/String)` with size cap; keyboard enhancement (`KeyEvent.kind`, base key, text).
2. **Terminal modes** (single table): `TerminalModes{ alt_screen, mouse: Off|Press|Drag|Any, sgr, focus, bracketed_paste, sync_output, keyboard: Legacy|Kitty(level)|ModifyOther }` used by `setup`, `teardown`, `WasmHost` and tests.
3. **Mouse pipeline in `Tui`**: `hover`, `capture`, `press`, click counting, motion coalescing, `Tui::capture(node)`/`release()`, widget-local `on_mouse(local: Pos, ev) -> Option<WidgetSignal>`, hit opt-out, modal/overlay root.
4. **Signals**: `Scrolled{dy}`, `Hovered(Option<usize>)`, `RowClicked(usize)`, `DoubleActivated`, `DragStart/Move/End{origin, pos}`, `SplitterDragged{path, delta}`, `TabDropped{...}`, `WindowClose(tab)`/`WindowMaximize(tab)`/`WindowNewTab(tab)`, `FocusChanged`, `Copy`, `Paste`.
5. **Cursor**: `WidgetState::cursor(rect, focused) -> Option<CursorSpec{pos, shape, blink}>`; `Tui::cursor()`; patch epilogue; VT `cursor_style` (DECSCUSR), honouring `cursor_visible`.
6. **Selection model**: `TextSelection{ anchor, extent, mode }` in absolute coordinates; `Tui::selection_text`; highlight role; copy/paste commands; `Clipboard` and OSC 52 owned by the host (`Tui::set_clipboard(Box<dyn Clipboard>)`) with auto choice (local tool vs OSC 52).
7. **Terminal widget**: `vt::encode_key/encode_mouse/encode_paste/encode_focus` driven by `ModeState`; `VtScreen::take_responses()` for DSR/DA/DECRQM/OSC; DECCKM, DECKPAM, 1003/1004/1007/1016, 2026, kitty flags; absolute-row scrollback with view anchoring; per-row `wrapped` flag; reflow on resize; `TerminalState::set_scroll_mode(bool)` so scrollback keys exist only in an explicit mode.
8. **Keymap layer**: `Keymap{ prefix, bindings: Mode -> Key -> Command }` evaluated before widgets with explicit "passthrough" mode and `prefix prefix` to send the literal key; commands (`FocusDir`, `ResizePane`, `NextTab`, `Copy`, `Paste`, `ScrollMode`, ...) so mouse and keyboard share one command set.
9. **Focus**: engine-owned focus tree with scopes (window -> widget), `focus_dir`, visible-only ring, validated ids.
10. **Layout/query**: `Tui::layout()` (solve without paint), `layout::neighbor(rect, dir)`, splitter geometry (`splitters() -> Vec<Splitter{path, rect, axis}>`).
11. **List model**: `ListModel`/`Viewport`/`FilterIndex` shared by `List/Wizard/Table` (cached lowercase, incremental narrowing, stable keys, wheel/PgUp/PgDn/Home/End, hover row).
12. **Run loop**: `Host::run(app)` with wake handle, frame scheduler, signal and resize sources, panic hook; `TerminalBackend::poll` replaced by a `wait(deadline, wake)`.

## 14. Suggested implementation slices (to avoid collisions)

`TUI` is a single 5451-line file, so slices should be by module region and element file:
- **A. Backend + parser + modes**: `ansi` (839-1207), `backend` unix/windows (4439-4819), mode table, resize/signal/wake, panic hook, paste decode.
- **B. Engine**: `scene` dirty fix (2328-2347), `engine` (3986-4194) mouse pipeline, hover/capture/focus/cursor, `chrome::window_hit` signals with tab index.
- **C. Terminal widget + VT**: `vt` (1211-2105) modes/responses/encoders/scrollback/selection, `widget` terminal part (3192-3338).
- **D. List/Wizard/Table/Log/Input elements**: `EL/*` files plus launcher fixtures (`🧫️fixtures/🔎️launcher/🔣️.json`, shared with other renderers - update together).
- **E. Dashboard wiring**: `DASH` keymap, focus, resize propagation, loop; depends on A-C APIs.
A should land first (wake + resize are prerequisites for everything), B second (fixes the dirty bug that gates hover and cursor), C/D in parallel, E last.

## 15. Verification notes and out-of-scope observations

- Nothing was executed. Items that need a live run before acting: (a) D11 per-keypress latency with 44k options in the dev profile (measure, do not trust the estimate), (b) the output burst size that trips `CONN:74`, (c) Windows legacy-conhost mouse behaviour (D26), (d) arrows after an outer terminal was left in application-cursor mode (Section 1), (e) native text selection bypass modifiers per terminal.
- Cross-agent observations (rendering/runtime scope, not audited here): `vt::feed_escape` has no charset designation arm, so `ESC ( B` (emitted by `tput sgr0`/`reset`) leaves a stray `B` on screen (`TUI:1391-1427`, `1425`, `1387`); `vt::feed_csi` aborts on `>`/`:` and prints the tail (`TUI:1429-1445`); SGR 5/6/8/9/21/53 and colon sub-parameters are not applied (`TUI:2038-2102`); chip labels come from `window_kind_id` (`TUI:3923`) and are patched in `DASH:364-377`; the connection thread's unconditional 10 ms sleep and 16 KiB read quantum (`CONN:42-77`) cap throughput; daemon PTY writes are synchronous on a non-blocking master (`DMN:706-717`).
- Existing coverage that guards parts of this area: control-byte vector for the leader (`DASH` tests `🧪️tests/🔬️unit/🦀️.rs:4-11`), launcher filter and pointer vectors (`:25-51`), parser/mouse/paste unit tests (`TUIT:375-433,684-715`), `window_hit` single-chip tests (`TUIT:148-177`). Nothing covers: paste routing, resize, wheel, hover, selection, key encoding, multi-chip controls, focus sync, mutate-then-`render()`.
