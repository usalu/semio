# Wave 1b — PTY Spawn Region

## Scope
Handcrafted TUI `pub mod pty` inserted between Backend and WasmHost in `⌨️tui/🦀️component.rs`, gated on `feature = "tui-terminal"`.

## Cargo.toml
Under `[target.'cfg(windows)'.dependencies] windows-sys` features, added:
- `Win32_System_Pipes`
- `Win32_Security`

`tui-terminal` still pulls `dep:libc` / `dep:windows-sys`.

## API
- `PtySize { cols, rows }`
- `PtyError { message }`
- `Pty::spawn / resize / writer / try_read / write_all / try_wait / kill`
- Unix: `openpty` + `Command::pre_exec` (`setsid`, `TIOCSCTTY`, dup2), nonblocking master, `TIOCSWINSZ`, Drop kills child
- Windows: ConPTY via `CreatePseudoConsole` + `CreateProcessW` (`PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE`), pipes, `ResizePseudoConsole` / `ClosePseudoConsole`

Callers: `use crate::tui::pty::Pty` (via `pub use component::*` in tui glue) when `tui-terminal` is enabled.

## Tests
- `pty_spawn_echo_hello` (unix + tui-terminal)
- `pty_resize_ok` (unix + tui-terminal)

## Verification command
```
CARGO_TARGET_DIR=<ticket>/target-w1b cargo test -p semio-framework-ui --features tui-terminal pty_ -- --nocapture
```
(Used ticket-local target dir due to shared `target/` cargo lock contention from parallel agents.)

## Result
**PASS** — `pty_spawn_echo_hello` ok, `pty_resize_ok` ok.

```
   Compiling semio-framework-ui v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust)
warning: `semio-framework-ui` (lib test) generated 3 warnings (run `cargo fix --lib -p semio-framework-ui --tests` to apply 3 suggestions)
    Finished `test` profile [unoptimized] target(s) in 2.79s
running 4 tests
test tui::component::tests::select_on_key_wraps_and_ignores_empty_options ... ok
test tui::component::tests::tabs_on_key_wraps_and_ignores_empty_tabs ... ok
test tui::component::tests::pty_resize_ok ... ok
test tui::component::tests::pty_spawn_echo_hello ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 81 filtered out; finished in 0.03s
```

Full log: `scratch-w1b-cargo-test.txt`

## Notes
- macOS `openpty` requires `*mut winsize` (`&mut ws`).
- Pre-existing unused-qualification warnings in Tests (Vt/Event area) are from parallel work; not introduced by Pty.
