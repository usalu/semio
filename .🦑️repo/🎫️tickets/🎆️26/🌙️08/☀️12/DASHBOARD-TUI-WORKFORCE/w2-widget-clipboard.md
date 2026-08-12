# W2 — Terminal Widget, Layout Mutations, Clipboard

## W2a Widget / Layout / Chrome
- `WidgetState::Terminal(TerminalState)` wrapping `vt::VtScreen` with follow/pin, scrollback offset, search mode (`/`), selection extract, and `WidgetSignal::TerminalPassthrough`.
- `WindowLayout.zoomed` + mutations: `zoom_window`, `split_window`, `resize_window`, `move_window_to_stack`, `activate_stack_tab`, `cycle_stack_tab`.
- `WindowState.stack_tabs` / `active_stack_tab` + strip painted on the window body top hairline (`Window` element).

## W2b Clipboard
- `backend::Clipboard` trait.
- `osc52_copy_sequence` (hand-rolled base64) for SSH/devcontainer-safe copy.
- `HostClipboard<F>` (OSC writer + native fallback: `pbcopy` / `wl-copy`/`xclip`/`xsel` / `clip.exe`).
- `MemoryClipboard` for tests/headless.

## Verification
```
CARGO_TARGET_DIR=<ticket>/🎯️target-w2 cargo test --features tui --lib -- window_layout_split terminal_widget osc52_copy window_stack_tabs paint_log_shows
```
Result: **5 passed; 0 failed**.
