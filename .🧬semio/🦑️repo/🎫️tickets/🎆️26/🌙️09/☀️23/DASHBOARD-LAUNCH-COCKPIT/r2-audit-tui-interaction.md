# R2 Audit: TUI Interaction Defects D01–D37 and Fleet-Plan §4.1 Contract

Read-only audit, 2026-10-08. Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT`. Brief: `r2-fleet-brief.md`. Inputs read: `fleet-plan.md` §4, `tui-interaction-audit.md` (D01–D37), `w0-tui-module-split.md`.

Evidence method: static reading of the current source plus `rg` through Bash. Nothing was run except one `cargo check` (section 4). No runtime, PTY or dashboard test was executed, so every status below is a static verdict. No source file, git state or AGENTS.md was touched.

## 0. Legend and aliases

Status: **FIXED** = the defect's described symptom is gone in code and nothing contradicts it. **PARTIAL** = a named part of the fix has landed, but the user-visible symptom or the wiring is still present. **OPEN** = unchanged, or the fix exists only as an unused API.

Path aliases (all relative to repo root). The `⌨️tui/` module directory is `TUI`.

| Alias | File |
| --- | --- |
| ENG | `TUI/⚙️engine/🦀️.rs` |
| WID | `TUI/🪀️widget/🦀️.rs` |
| ANSI | `TUI/🔡️ansi/🦀️.rs` |
| BE | `TUI/🔌️backend/🦀️.rs` |
| VT | `TUI/📟️vt/🦀️.rs` |
| SCN | `TUI/🎬️scene/🦀️.rs` |
| CHR | `TUI/🖥️chrome/🦀️.rs` |
| LAY | `TUI/📏️layout/🦀️.rs` |
| EVT | `TUI/📡️event/🦀️.rs` |
| PTY | `TUI/🚇️pty/🦀️.rs` |
| WIZ / LST / INP / LOG / TBL | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/{🧙️Wizard,📃️List,✏️Input,🪵️Log,📊️Table}/🎯️targets/⌨️tui/🦀️.rs` |
| DASH | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs` (919 lines) |
| CONN | `…/🎛️dashboard/📎️connection/🦀️.rs` |
| DMN | `…/🎛️dashboard/🌀️daemon/🦀️.rs` |

Line numbers in this report are current line numbers in the files above. They are not the old monolith numbers used by `tui-interaction-audit.md`. Use the w0 mapping (`w0-tui-module-split.md`, "Translating Old Line Citations") if needed, but note that module files have grown since w0 (section 3).

## 1. Summary

| Status | Count | IDs |
| --- | ---: | --- |
| FIXED | 1 | D23 |
| PARTIAL | 3 | D02, D06, D07 |
| OPEN | 33 | D01, D03, D04, D05, D08–D22 (except D02, D06, D07, D23), D24–D37 |

Blocking finding: **`semio-framework-ui --features tui-terminal` does not compile on this Windows host** (`error[E0277]` at `PTY:728:107`, exit 101; section 4). The w0 slice reported exit 0 on macOS, and its Windows halves were recorded as "WRITTEN BUT UNVERIFIED". Nothing downstream of `semio-framework-ui` (including the dashboard crate) can build on Windows until this is fixed.

The §4.1 contract is in place with exact signatures (section 2). Most of the new surface is declared but not produced or consumed, so the defects it was meant to fix are still visible to the user.

## 2. Fleet-plan §4.1 shared types: presence and wiring

Presence was checked by `rg` and by reading each declaration. Every item in §4.1 is declared with the signature §4.1 gives. No deviations in signatures were found.

| §4.1 item | Declared at | Wired / used | Verdict |
| --- | --- | --- | --- |
| `MouseButton { Left, Middle, Right }` | EVT:40-44 | produced by ANSI:340-345 | present, used |
| `MouseKind { Down, Up, Drag, Move, Scroll{dx,dy} }` | EVT:48-54 | Down, Up, Drag, Scroll produced (ANSI:346-358). **Move never produced** (ANSI:352-353 maps no-button motion to `Drag`). Engine consumes only `Down` (ENG:142) | present, partly unwired |
| `MouseEvent { kind, pos, mods, clicks }` | EVT:58-63 | `clicks` is always `1` (ANSI:359) | present, no click counting |
| `Event { Key, Mouse, Paste, Resize, FocusGained, FocusLost, Wake }` | EVT:67-75 | Paste produced ANSI:367. Resize produced only by WasmHost (`🏃️host/🦀️.rs:30`), never by the native backend. FocusGained/Lost produced ANSI:287-288 but never enabled (no `?1004h`). Wake produced BE:426, BE:657. Paste, Focus and Wake fall into engine `_ => {}` (ENG:179) and the dashboard catch-all (DASH:887-890) | present, mostly unwired |
| `CursorShape { Block, Underline, Bar }`, `CursorSpec { pos, shape, blink }` | WID:44-56 | No producer: `WidgetState::cursor` returns `None` for every variant (WID:487-490) | present, unused |
| `WidgetState::on_key` | WID:449 | Live | used |
| `WidgetState::on_mouse(rect, &MouseEvent)` | WID:467-472 | Only Wizard, only `Down(Left)` | stub for all other widgets |
| `WidgetState::on_paste` | WID:475-478 | `None` for all | stub, never called |
| `WidgetState::set_hover` | WID:481-484 | returns `false`, never called | stub |
| `WidgetState::cursor` | WID:487-490 | returns `None`; `Tui::cursor` (ENG:70-76) delegates, never called | stub |
| `WidgetState::interactive` | WID:493-495 | always `true` (drives focus ring, D25) | stub |
| `WidgetState::tick` | WID:498-501 | returns `false`, never called | stub |
| `WidgetSignal` (18 variants, §4.1 order) | WID:20-40 | Produced: Activated, SelectionChanged, ValueChanged, Toggled, TabChanged (Tabs element only), NavigateBack, TerminalInput, WindowClose, WindowMaximize, WindowNewTab, WindowTabActivated. **Never produced:** Hovered, ContextMenu, Copy, OpenUrl, WindowFocus, TabMoved, SplitterDragged | present, 7 of 18 never produced |
| `Tui::dispatch(&Event) -> Vec<(NodeId, WidgetSignal)>` | ENG:126-182 | Live, Down-only for mouse | present, partial |
| `Tui::layout` / `render` | ENG:95-97 / ENG:208-233 | `render()` used by nobody (DASH uses `render_full` at DASH:214, 320, 623, 907) | present, `render` broken (D08) |
| `Tui::cursor` / `hovered` / `capture` / `tick` | ENG:70 / 79 / 84 / 89 | `cursor` unused; `hovered` never set (ENG:31, 40); `capture` only stores, `#[allow(dead_code)]` (ENG:32-33); `tick` returns `false` (ENG:89-92) | present, stubs |
| `Capabilities { color, synchronized_output, unicode }` | BE:243-247 | constants (BE:287-289), no detection, no consumer | present, stub |
| `ColorDepth`, `UnicodeLevel` (both `Ord`) | BE:227, 236 | used only by `Capabilities` | present, provisional (w0 says so) |
| `TerminalBackend` trait (8 methods) | BE:264-273 | `present` ignores cursor (BE:417, windows BE:649 `let _ = cursor`) | present, cursor unused |
| `TerminalBackend::wait` | BE:422-450 (unix), BE:653-677 (windows) | 80 ms slices (`INTERIM_WAKE_SLICE`, BE:276) | present, interim |
| `TerminalBackend::waker` / `Waker` | BE:250-261, 452-455, 679-682 | Atomic flag checked per slice; **dashboard never calls `waker()`** | present, not wired |
| `TerminalBackend::copy` | BE:457-460, 684-686 | OSC 52 write; **no caller** | present, unused |

§4.1 deviation to report to the coordinator (contract gap, not a signature mismatch): `WindowClose` carries an index, but `WindowMaximize` and `WindowNewTab` carry none (WID:33-35). D09's fix direction asks for indices on all three chrome controls, so the contract itself does not deliver D09 for maximize and new-tab.

Interim bodies that §4.1 bodies must replace (as w0 listed): `wait` 80 ms slices, `waker` flag, `capabilities` constants, `present` cursor drop, `copy` raw OSC 52, `tick`, `hovered`, `capture`, `cursor`, `set_hover`, `on_paste`, `on_mouse` outside Wizard. All of these are still in place.

## 3. Module split (w0) status

**Landed.** `TUI/🦀️.rs` is a 56-line manifest with 15 `#[path]` module declarations (ANSI, BE, CHR, ENG, EVT, LAY, PTY, SCN, TEXT, VT, WID, geometry, theme, cell, host). The folders exist with the names in w0's table, plus `🪟️windows` and `🧪️tests`. The one monolith file is gone. `tests` and `windows_abi` resolve.

Caveat: module files have grown since the w0 slice (w0 recorded line counts; current counts are in parentheses): BE 734 (621), PTY 930 (525), WID 519 (437), ENG 240 (207), EVT 75 (64), ANSI 372 (367). Peer edits account for this; the post-split changes were not audited for whether they were verified. The w0 byte-identical proof no longer applies.

## 4. Cargo check (requested)

Command (as specified, with the output saved to the ticket):

```
cd /c/git/semio && CARGO_TARGET_DIR=/c/git/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-audit-a3 cargo check -p semio-framework-ui --features tui-terminal --message-format=short
```

Package `semio-framework-ui` and feature `tui-terminal` were verified in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` (name at line 3, `tui-terminal = ["tui", "dep:libc"]`).

Result: **exit 101, fails**. Full log: `🗑️generated/r2-audit-a3-cargo-check.log` (inside this ticket). Only error:

```
…\⌨️tui\🚇️pty\🦀️.rs:728:107: error[E0277]: `*mut c_void` cannot be sent between threads safely
error: could not compile `semio-framework-ui` (lib) due to 1 previous error
```

Source at PTY:728-732: `std::thread::Builder::new().name("ConPTY output")…spawn(move || { … ReadFile(output_read.0.as_raw(), …) })`. `Shared` (PTY:531-533) has `unsafe impl Send`, but the closure uses `output_read.0`. Under edition 2021 disjoint closure capture (`Cargo.toml` `edition = "2021"`), the closure captures the inner `OwnedHandle` field, not `Shared`, so the unsafe `Send` impl is bypassed. **Likely cause, UNVERIFIED**: no fix was attempted (read-only audit). A probable fix is to make the closure capture the whole wrapper, for example `let output_read = output_read;` before `spawn`, but that must be checked by a build.

Not run, UNVERIFIED: `cargo check -p semio-framework-repo-dashboard` (depends on the failing crate; expected to fail the same way), `cargo test`, and the wasm checks.

## 5. Per-defect verdicts

| ID | Pri | Status | Evidence (static, file:line) | Why |
| --- | --- | --- | --- | --- |
| D01 | P0 | OPEN | DASH:635-639 (handler exists); DASH:609 (`size()` once at startup); only producer of `Event::Resize` is `🏃️host/🦀️.rs:30` (WasmHost); BE:422-450 never emits Resize; no SIGWINCH handler anywhere (only child `killpg` at PTY:369) | Resize is never detected in a native terminal, so DASH:635 is dead code. |
| D02 | P0 | PARTIAL | Wake primitive landed: BE:425-428, 656-659, Waker BE:250-261. Input path: select returns when the tty is readable (BE:430-443), so keystrokes are not waiting. Output path: DASH:629 `wait(80 ms)`; `waker()` never called (grep: none in DASH, CONN); CONN:31, 76 sleep 10 ms per cycle | Keystroke input is immediate. PTY output still reaches the UI only at the next 80 ms slice plus up to 10 ms of connection sleep. User-visible echo latency is unchanged. |
| D03 | P0 | OPEN | Hardware cursor hidden at ANSI:63 (`?25l`), shown only at teardown (ANSI:68); BE:417 and BE:649 ignore cursor; DASH:623, 907 pass `None`; `Tui::cursor` exists (ENG:70) but every widget returns `None` (WID:487-490); VT `cursor_visible` set (VT:806) and never read; INP:43-46 paints a block over the glyph; WIZ:86 filter row has no caret | No visible cursor in the terminal pane, wizard filter or input. |
| D04 | P0 | OPEN | Intercepted before the child: Ctrl+B DASH:694; Ctrl+W DASH:827; Tab/BackTab DASH:836-849 (and ENG:130-131 before the widget); Esc DASH:854-857. Widget intercepts PgUp/PgDn/End/Ctrl+Home/`/`/Ctrl+P (WID:362-393). Encoder `key_to_pty_bytes` (WID:320-339) still has no Delete, Insert, F-keys, Alt+key, modified arrows; Ctrl+Space dropped | Encoding moved into the widget (WID:394, DASH:864-868: progress), but the reserved-key set is still wrong. |
| D05 | P0 | OPEN | `/` enters search WID:382-386; search swallows printable keys WID:343-359; dashboard Esc intercept DASH:854-857 means the widget's Esc arm (WID:345) never runs; search row steals the last pane row (WID:406) | After `/` in a shell pane, typed input is swallowed; Esc leaves terminal mode but search stays armed. |
| D06 | P0 | PARTIAL | Producer: bracketed paste start/end ANSI:300-306, `Event::Paste` emitted ANSI:362-370; `?2004h` in setup (ANSI:63). Still open: ANSI:363 `paste_buf.push(b as char)` (mojibake for UTF-8); no size cap or timeout; no consumer: DASH:887-890 routes to `tui.dispatch`, ENG:179 drops it; WID:475-478 `on_paste` is a stub; no `200~/201~` wrap in DASH:864-868 | Paste still does nothing visible; if consumed as it is today it would be double-encoded. |
| D07 | P0 | PARTIAL | Decoded now: Up, Drag, Scroll, Down (ANSI:334-360). Engine still routes only `Down` (ENG:141-142), only `Down(Left)` to a widget (ENG:155-160), and only Wizard implements `on_mouse` (WID:467-472). `Move` never produced (ANSI:352-353). No `?1003h` (ANSI:63), no `?1004h`. `capture` stores only (ENG:84-86). `hovered` unset (ENG:31, 79). Click focuses the widget, not the pane (ENG:152-154). `clicks` always 1 (ANSI:359). `?1002h` still disables host selection (ANSI:63). No selection model | Mouse decoding is in place; routing, hover, drag, wheel, selection and native-selection preservation are not. |
| D08 | P0 | OPEN | `mark_dirty` stops at the first node that already has the bits (SCN:157-158); only the root is cleared (ENG:209, `take_dirty(root)`; SCN:165-169); no layout clears non-root dirty bits (grep: none in LAY); DASH uses only `render_full` (DASH:214, 320, 623, 907) | Incremental `render()` returns empty patches after the first frame. The dashboard hides this by full repaints (DASH:907). |
| D09 | P1 | OPEN | CHR:127-129 `WindowClose(w.active_stack_tab)` for every tab's ✕; CHR:130-135 maximize and new-tab unindexed; ENG:161-175 chrome probe runs for any button (inside the `Down(_)` guard at ENG:142); DASH:644 ignores the index and closes the active window | Closing an inactive chip closes the active tab; right and middle clicks on ✕ still fire. |
| D10 | P1 | OPEN | `dash.focused` is the key-routing source (DASH:58, 851-885); engine focus changes only on click (ENG:152-154) and is then overwritten by `tui.set_focus` at DASH:863; `close_window` focuses `order[0]` (DASH:397-400) and does not activate its stack tab | Two sources of truth; clicking a pane does not move keyboard focus. |
| D11 | P1 | OPEN | `visible_indices` allocates a lowercase `String` per option per call (WID:165-169); called per key (WIZ:31), per paint (WIZ:75), per click (WIZ:25); `refresh_view_for` clones all labels and scans twice (DASH:98-112); no cache | O(N) work per keystroke and per frame over ~44k options. Not measured here. |
| D12 | P1 | OPEN | Viewport uses `w.offset`, which only DASH:113 writes (to 0) (WIZ:14-18); click activates on first press (WIZ:27) and DASH:683-687 spawns; LST:12-32 never updates `offset` (paint at LST:34-48 uses it); WIZ:30-71 has no PgUp, PgDn, Home, End | Highlight pinned to the bottom edge; single click launches a task. |
| D13 | P1 | OPEN | `resize_terminals` only at DASH:513 (ReplayComplete) and the dead DASH:637; not called after split (DASH:753, 766), zoom (DASH:651-656, 741-746), close (DASH:380-403), new tab (DASH:659-669, 798-804); initial size `width-4` at DASH:216; VT resize copies top-left only (VT:410-439) | PTY and VT geometry drift from the widget rect after any layout change. Daemon has a working resize path (DMN:801-809) that the view does not trigger. |
| D14 | P1 | OPEN | `decset` handles 6, 7, 25, 1000, 1002, 1006, 1049, 2004 only; `1 => {}` (DECCKM, VT:800); no 1003, 1004, 1007, 1016, 2026, 47/1047 (VT:798-825); no reply path (`finish_csi` VT:236-280 has no `n` or `c`); mouse and paste flags are set (VT:806-822) and read nowhere (grep) | Child mouse, paste and focus modes and queries are not honoured by the view. The daemon has a separate replay mode tracker (`📼️replay` DMN:27, 322, 329) that the view does not use. |
| D15 | P1 | OPEN | Scrollback keys only (WID:363-381); `pinned` toggle is a no-op for offset (WID:387-393); `max_offset` uses primary scrollback even on the alternate screen (WID:285-287, VT:454-456); no wheel (ENG drops Scroll, ENG:141-178) | Wheel absent; alternate-screen PgUp scrolls the shell's history. |
| D16 | P1 | OPEN | `selection` never set (WID:263, 269); `selected_text` no callers (WID:297); `copy()` implemented (BE:457-460, 684-686) with no caller; `HostClipboard` (BE:151-197) no consumer; no highlight painted (WID:399-414) | No selection, no copy, no paste. |
| D17 | P1 | OPEN | CONN:42 16 KiB read; CONN:76 10 ms sleep; CONN:39 256-slot channel; CONN:74 disconnect on overflow; CONN:98 drains at most 255 per call; DASH:484 `receive(ZERO)` per loop | Backpressure still disconnects; throughput still bounded by the read quantum and sleep. |
| D18 | P1 | OPEN | Synchronous shutdown wait DASH:291-303 (5 s), called on the UI thread at DASH:736; `render_full` on UI thread DASH:214, 320; `graph_is_current` on click DASH:427; blocking tty write BE:418. Already async: preference save DASH:135, daemon connect DASH:457-465, inventory `start` DASH:179 | Some work moved off-thread; shutdown and render-to-measure still block the UI. |
| D19 | P1 | OPEN | Every change does `render_full` (DASH:907); no `?2026` (ANSI:63); no frame budget (loop DASH:627-909) | Full-frame emit per paint, tearing risk. |
| D20 | P1 | OPEN | No `set_hook`/`take_hook` in TUI or dashboard (grep); signal counter `interrupts()` (PTY:71-85) is installed only inside itself and has **zero callers**; no SIGWINCH/TERM handler; Drop restores on unwind (BE:487-491) | Abort, kill and signals leave the terminal in raw/alt-screen mode; panic message lost on alt screen. |
| D21 | P1 | OPEN | INP:14-16 inserts any `Char` ignoring modifiers (Ctrl+A inserts `a`); INP:25-31 moves the cursor by one byte; INP:44 `value[..cursor]` panics off a char boundary. Backspace (INP:19-22) is char-aware | Left/Right on non-ASCII text panics. |
| D22 | P1 | OPEN | SS3 only P/Q/R/S (ANSI:244-256); ESC + non-ASCII dropped (ANSI:235-240, `_ => Ground`); `?`, `>`, `:` treated as CSI final byte (ANSI:258-272, only `<` special); wheel left/right → `dy:+1` (ANSI:346-351); F13+ and CSI u absent (ANSI:299-332); OSC ST leaks `\` (ANSI:183-187); 0x1c-0x1f as plain chars (ANSI:206); 0x0a → Enter (ANSI:198); paste mojibake (ANSI:363) | Parser gaps unchanged. |
| D23 | P1 | FIXED | PTY:290-307 `write_all` retries on `WouldBlock` with a poll(POLLOUT) readiness wait and fails after 10 s stall; daemon input calls it (DMN:788-793); Windows counterpart PTY:826-840 | Large input is no longer truncated. Caveat: a stalled child can hold the daemon loop for up to 10 s. |
| D24 | P1 | OPEN | `q` quits when focused window is not Launcher and not terminal input, so Overview and Settings quit on typed `q` (DASH:823); Ctrl+B leader (DASH:694) | Typing `q` in task or settings filter quits; no configurable prefix. |
| D25 | P1 | OPEN | `interactive()` always true (WID:493-495); `dfs_focusables` includes all widgets regardless of visibility (ENG:10-21); `set_focus` unvalidated (ENG:65-67); `node_raw_mut` `expect("stale NodeId")` (SCN:142-144) | Hidden stack tabs and labels take focus; dangling id panics. |
| D26 | P2 | OPEN | Windows ABI defines only ECHO, LINE, PROCESSED, VT-input, VT-output flags (`TUI/🪟️windows/🦀️.rs:13-17`); no mouse, quick-edit, window-input or extended flags; Windows wait does a blocking `ReadFile` after the wait (BE:662-668); no resize; plus the compile break in section 4 | Windows mouse and resize are not enabled; compile fails. |
| D27 | P2 | OPEN | `libc::select` with `FD_SET` (BE:430-436); `read` return `0` or `-1` ignored and an empty Vec returned forever (BE:439-443) | FD_SETSIZE risk; dead tty spins. |
| D28 | P2 | OPEN | TBL:30, 39, 48 index `t.rows[t.selected]` unchecked | Panic if selection is stale. |
| D29 | P2 | OPEN | LOG:17-22 first PageUp from Follow yields the same tail view; LOG:30 Home → `At(0)` shows one line | As described. |
| D30 | P2 | OPEN | Armed hint list DASH:548 omits `z`, `-`, `|`, `x`, `t` | Keys not discoverable. |
| D31 | P2 | OPEN | No `TERM` or `COLORTERM` set in PTY spawn (PTY:156-175) or in daemon session env (DMN:770-774); grep finds none | Children inherit whatever the daemon had. |
| D32 | P2 | OPEN | LAY:368-370 sets `zoomed` only; CHR:486-495 reparents all windows back to canvas every remount; CHR:550-557 reparents the zoomed window and returns, leaving others as visible canvas children; LAY:276-279 solve returns one measure only | Non-zoomed windows stay visible, hit-testable and in focus ring under the zoomed one. |
| D33 | P2 | OPEN | WIZ:55-63 Backspace on empty filter → `NavigateBack` → DASH:447 leaves launcher on key repeat | As described. |
| D34 | P2 | OPEN | WIZ:36, 44 emit visible position; WIZ:27, 49 emit option index | Inconsistent index semantics. |
| D35 | P2 | OPEN | VT:410-439 copies top-left rectangle; scrollback rows keep original width | No reflow. |
| D36 | P2 | OPEN | Legacy X10 `ESC [ M` has no decoder (ANSI:279-291 has no `M` arm); button code masked with `& 3` (ANSI:340), buttons 8-11 alias | As described. |
| D37 | P2 | OPEN | DASH:633-691 processes events in one batch using the rects from the last layout (layout re-solved only on `render`/`render_full`, ENG:213, DASH:214) | Stale hit test within one batch. |

## 6. New findings outside D01–D37

1. **Windows compile break** (section 4). P0 blocker for everything on Windows.
2. **Phantom right-button drag in the decoder (latent).** ANSI:352-353 maps motion with no button (SGR code 35, button bits = 3) to `Drag(Right)` because `btn == 3` falls into `_ => Right` (ANSI:341-345). The engine ignores drag today, but once D07 routes `Drag`, every hover would start a right-button drag. Fix together with D07.
3. **Dead signal counter.** `PTY:71-85` `interrupts()` installs SIGINT/SIGTERM/SIGHUP handlers, but nothing calls it, so no handler is installed. Reads as D20 coverage; it is not.
4. **Second mode tracker in the daemon** (`📼️replay`, DMN:27, 322-329). It tracks private modes and DECSCUSR for replay. The view has no equivalent, so D03/D14 would need the same table in two places unless the view reads the daemon's tracker.
5. **Focus-tree helpers unwired.** `resize_window` (LAY:444), `move_window_to_stack` (LAY:490) and `cycle_stack_tab` (LAY:385) exist; DASH:13 does not import them. Split resize, tab moves and tab cycling are missing (`tui-interaction-audit.md` §10 capability rows 4d, 9a, 9b).
6. **Stub signals defined but never produced:** Hovered, ContextMenu, Copy, OpenUrl, WindowFocus, TabMoved, SplitterDragged (WID:27-39).

## 7. Not verified / not run

- No runtime behaviour was observed. D02 latency, D11 cost, D32 visibility and D37 batching are static readings only.
- `cargo test`, `cargo check` of the dashboard crate, wasm checks and the Windows backend were not run (section 4).
- The compile-break cause (section 4) is a hypothesis, not checked by a build.
- The mapping of old monolith line numbers to the new files was not re-derived; every citation here is in the current files.
- Peer edits after w0 (module growth in section 3) were not reviewed beyond the defect areas above.
