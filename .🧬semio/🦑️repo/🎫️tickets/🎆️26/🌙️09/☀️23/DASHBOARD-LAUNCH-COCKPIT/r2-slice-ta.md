# R2 Slice T-A2: Terminal I/O (finish of T-A)

TUI = `🧰️framework/🔨️modules/🖱️ui/⌨️tui`. Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT`, 2026-10-08. T-A was stopped mid-work; its tree was
already far ahead of the audit (the audit texts D01..D36 describe the pre-T-A code). T-A2 verified every item against the code, compiled it,
fixed what was open and added the missing pieces. No dangling `include!`/`#[path]` was found: all test files exist and are included
(`🔡️ansi` -> `ansi-unit`, `input-decoding`; `🔌️backend` -> `backend-capabilities`, `backend-clipboard-mailbox`, `backend-native-common`,
`backend-native-unix-unit`, `backend-native-windows-unit`).

## 1. Per-id status (evidence = file in TUI, symbol)

| ID | Status | Evidence |
| --- | --- | --- |
| D01 / R01 resize | DONE (code), runtime on a live console UNVERIFIED | Unix: `🔌️backend` `native_unix::on_resize` (SIGWINCH sets `PENDING_RESIZE`, writes the self-pipe), `drain_signals` reads `TIOCGWINSZ` through `ResizeTracker::observe` (no event for 0 sizes or repeats). Windows: `native_windows::records_to_bytes` flags `WINDOW_BUFFER_SIZE_EVENT` (`ENABLE_WINDOW_INPUT`), `poll_size` + a 250 ms size poll as fallback. Tests `resize_tracker_reports_only_real_changes`, `a_resize_signal_sets_the_flag_and_wakes_the_pipe_then_handlers_are_put_back` (Unix, compiled only), `console_records_become_the_vt_byte_stream_and_a_resize_flag`. |
| D02 waker | DONE | `TerminalBackend::waker() -> Waker`. Unix: self-pipe in the `poll` set (`select` on macOS, FD_SETSIZE guarded). Windows: auto-reset event waited together with the console input handle by `WaitForMultipleObjects` (`wait_signalled`). Tests `a_set_wake_event_ends_a_pending_wait_immediately` (woken after ~40 ms of a 10 s wait), `a_written_wake_pipe_makes_the_wait_return_at_once_and_drains_clean`. A-2 `Connection::set_notifier` + A-3 wiring consume it. |
| D06 paste | DONE | `AnsiParser`: bracketed paste collected as bytes, decoded once as UTF-8 (`finish_paste`), `PASTE_LIMIT` 1 MiB cut on a character boundary, `PASTE_TIMEOUT` 2 s delivers a paste that never ends. Fixture rows and tests `paste_*`. `?2004h` in the mode table. |
| D20 panic/signal restore | DONE (code) | `native_common::panic_restore` (hook fires only on the thread that entered the terminal); Unix `restore_now` async-signal-safe (write + tcsetattr from a pre-built teardown) on SIGINT/TERM/HUP/QUIT, `SignalGuard` puts the old dispositions back; Windows `SetConsoleCtrlHandler` (`on_console_control`) + `restore_for_panic`. Test `panic_restore_fires_only_for_the_thread_that_entered_the_terminal` (no `[DEBUG]` probes any more, real assertions on the unwind). Actual SIGTERM/CTRL_CLOSE delivery UNVERIFIED. |
| D22 parser | DONE | `🔡️ansi` `AnsiParser`: SS3 incl. keypad, `?`/`>`/`:` private/sub-parameters, CSI u incl. shifted/release, modifyOtherKeys, F1-F20+, Linux console `CSI [ A`, OSC with BEL/ST/CAN/SUB, 0x1c-0x1f as Ctrl+punctuation, 0x0a = Ctrl+J, UTF-8 recovery, wheel left/right as `dx`, `ESC` + UTF-8 = Alt+char. 55+ shared vectors, each fed whole and byte by byte. |
| D26 Windows ABI | DONE | `🪟️windows`: console flags (VT input/output, window input, extended flags, quick-edit off, newline auto-return off), `INPUT_RECORD` union, `ReadConsoleInputW`, `GetNumberOfConsoleInputEvents`, `CreateEventW`, `WaitForMultipleObjects`, ctrl handler. No blocking `ReadFile` in `wait` any more. Unused `WAIT_FAILED`/`PeekNamedPipe` deleted. |
| D27 select/dead tty | DONE | `poll` instead of `select` (select only on macOS), EOF on the tty returns `UnexpectedEof` instead of spinning. |
| D36 X10/buttons | DONE | `finish_csi` `M` with no parameters switches to the 3-byte X10 reader, urxvt `M` with 3 parameters, SGR; buttons 8-11 (`code & 0x80`) ignored; low bits 3 + motion = `MouseKind::Move`, held button drag keeps its button. Fixture group "pointer" (also in `🖱️pointer-routing`). |
| R12 capabilities | DONE | `detect_capabilities(env, windows) -> Capabilities` (`NO_COLOR`, `TERM`, `COLORTERM`, `TERM_PROGRAM`, `WT_SESSION`, `KITTY_WINDOW_ID`, `VTE_VERSION`, locale); `FramePresenter::compose` quantises with `quantize_patch` (256 / 16 / mono); `TerminalBackend::capabilities()` returns the detected value. Fixture table of 29 environments. |

Other items of the brief: cursor emission in `present` (`CursorEmitter::frame`: hide while painting, DECSCUSR shape, `?25h/l`, position, only changed escapes), `?2026` bracket (`SYNC_BEGIN/END` only when something is written and the terminal supports it), `?1004h` focus and `?1003h` any-motion in the one `SESSION_MODES` table (setup forward, teardown reverse), click counting (`ClickCounter`, 400 ms, same cell/button, up to 3; applied in `InputPipeline::feed` from the wall clock of arrival), modifier bits on mouse events, OSC 52 copy (`copy`, 1 MiB cap, empty text is a no-op): all DONE.

## 2. Changed by T-A2

1. **Flaky test fixed**: `an_unsignalled_wait_times_out_and_a_signalled_input_wins` failed 10 of 60 runs because it asserted `elapsed >= 25 ms` for a 30 ms wait, but Windows timer granularity lets the wait return early. Now only the outcome and an upper bound are asserted: 0 of 50 failures. Removed the `println!("[DEBUG] ...")`.
2. **Escape timeout hole**: a lone ESC followed by a byte arriving after `ESCAPE_TIMEOUT` (when the loop was busy, so `wait` never ran `expire_due`) was merged into an Alt chord. `InputPipeline::feed` now expires a due partial input before it feeds new bytes. Test `a_byte_arriving_after_the_escape_timeout_is_not_an_alt_chord`.
3. **Keypad key**: `event::Key::Keypad(KeypadKey)`; `KeypadKey` moved from `📟️vt/🎹️encode` to `📡️event` (re-exported by `encode`); `KeypadKey::character()`, `Key::text_equivalent()`, `KeyEvent::text_equivalent()`. The parser decodes SS3 keypad codes (`ESC O M p..y j k l m n o X`) and kitty keypad code points (57399-57416) to it. `encode_key` encodes `Key::Keypad` through `encode_keypad` (application keypad mode of the child). `WidgetState::on_key` hands every non-terminal widget the `text_equivalent` (so Input/List/Select keep typing digits from the keypad) and the terminal widget the raw key. The dashboard `⌨️controls::spec_of` spells a keypad key as the main-block key it types (Enter -> `enter`, others -> the character), so keymap bindings need no keypad variant. The host stays in numeric keypad mode (`ESC >`), so keypad keys are produced only when the host terminal is in application keypad mode; switching the host to `ESC =` is a one-line mode-table decision for A-3 if the child should see distinct keypad codes.
4. **Third-party oracles**: `T/r2-ta-oracle.py verify` (run with the Python that has prompt_toolkit 3.0.52, `python3 -X utf8`): prompt_toolkit `Vt100Parser` decodes 17 tagged key rows of the fixture to the same events (arrows, SS3, home/end, tilde, F-keys 1-15+, ctrl letters, backtab, backspace, Esc); `infocmp -x -1` reproduces the colour depth of the 4 TERM rows it knows (xterm, xterm-256color, screen, xterm-direct). Node readline (17 rows), the xterm specification regexes (pointer), TextDecoder (paste) and `color-convert` (quantisation) were already tagged by T-A; `bun test .../⌨️input-decoding/🟦️.ts` = 5 pass.
5. **Fixtures**: `🧫️fixtures/⌨️input-decoding/🔣️.json` gained `capabilities` (29 environments), `keypad-application-mode`, `keypad-kitty-codepoints`; `🥒️.feature` gained the keypad and capability scenarios. The five hand-written capability tests were replaced by the one fixture adapter `every_capability_vector_detects_the_shared_depth_repertoire_and_sync_support`.
6. **Rules audit (T-A rows)**: restored `???`/`??` mojibake in region markers and docstrings (backend, host); unique emoji per docstring in `🔡️ansi`, `🔌️backend`, `🪟️windows`, `🏃️host`, `📡️event`; the in-body `//` comment in `🏃️host` became a docstring; `[DEBUG]` panic probes replaced; clippy clean for `🔡️ansi`, `🔌️backend`, `🪟️windows`, `🏃️host`, `📡️event` and their tests (`div_ceil`, `is_ok_and`, needless `return`, by-value `SavedConsole`, `Option` return of a test helper, field assignments after `Default`, `upper_case_acronyms` allowed with the other Win32 FFI naming allows); `osc52_payload_check` no longer warns in `--features tui`; mailbox test result no longer unused.
7. Not touched: `🚇️pty` (A-2 owns it; the `partition(is_script)` E0631 that blocked the first compile was fixed by A-2 in the meantime).

## 3. API for consumers

- `backend::{TerminalBackend, Capabilities, ColorDepth, UnicodeLevel, Waker, FramePresenter, detect_capabilities, osc52_copy_sequence, OSC52_LIMIT, NativeTerminal}`; `Waker::new(fn)`, `Waker::wake()` (cheap, any thread). `present(&AnsiPatch, Option<CursorSpec>)` does colour degradation, cursor and `?2026`; call it once per frame with `tui.cursor()`.
- `ansi::{AnsiParser, ClickCounter, CursorEmitter, setup_sequence, teardown_sequence, quantize_patch, rgb_to_ansi256, rgb_to_ansi16, SYNC_BEGIN, SYNC_END, PASTE_LIMIT, ESCAPE_TIMEOUT, SEQUENCE_TIMEOUT, PASTE_TIMEOUT, CLICK_WINDOW_MS}`; `AnsiParser::{pending_timeout, expire}` for hosts that own the byte stream (WASM host: `WasmHost::feed_at(bytes, now_ms)`).
- `event::{Key::Keypad, KeypadKey, Key::text_equivalent, KeyEvent::text_equivalent}`.

## 4. Verification (all with `CARGO_TARGET_DIR=…/target-fleet-ta2` + private build dir)

| Command | Result |
| --- | --- |
| `cargo test -p semio-framework-ui --features tui-terminal --lib tui::` | 363 passed, 0 failed (1 filtered, not TUI), ~10 s |
| the 8 `native_windows` tests, 50 repeated runs | 0 failures |
| `cargo check -p semio-framework-ui --features tui` and `--target wasm32-unknown-unknown` | exit 0, no TUI warning |
| `cargo check -p semio-framework-repo-dashboard --tests` | exit 0 |
| `cargo check -Zbuild-std --target x86_64-unknown-linux-gnu -p semio-framework-ui --features tui-terminal --lib --tests` | exit 0 (Unix backend and its tests compile, no TUI warning) |
| `cargo +nightly-2026-07-07 clippy -p semio-framework-ui --features tui-terminal --all-targets` | 0 warnings in my modules |
| `bun test ./…/⌨️input-decoding/🟦️.ts` | 5 pass |
| `python3 -X utf8 T/r2-ta-oracle.py verify` | prompt_toolkit 17 rows agree, terminfo 4 rows agree |

UNVERIFIED: the Unix tests were only compiled (no Linux/macOS runtime here; WSL has no toolchain); SIGWINCH/SIGTERM delivery and the Windows
resize/ctrl-close behaviour on a live console (`r2-ta-probe` exists from T-A but was not re-run: this shell has no console); the `kitty`/`wezterm`
rows of the capability table encode known-terminal knowledge, not a measurement.

## 5. Requests

- A-3 / T-B: nothing blocking. Optional: decide whether the host should request application keypad mode (see 2.3).
- A-2 (`🚇️pty`): the rules audit lists pty rows (unique emoji, clippy `field assignment`, `Option`, by-value, `unwrap_or`) under T-A; T-A2 did not touch them.
