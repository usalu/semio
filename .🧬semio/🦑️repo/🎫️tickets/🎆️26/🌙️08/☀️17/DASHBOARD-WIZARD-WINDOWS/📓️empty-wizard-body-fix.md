# Empty Wizard Body Fix

## Symptom
Dashboard chrome rendered (navbar, `w1` window frame, footer) but the window body was blank.

## Cause
`shell()` created window nodes with the default layout constraint (`Direction::Row`, `Dimension::Auto`).
The wizard child only set `height: Weight(1)` and `WidgetState::Wizard` preferred size was `0×0`, so row layout allocated **width 0**. Chrome still painted the full window rect; the wizard had nothing to paint into.

## Fix
1. `shell()` and `mount_window_layout` now set each window to `Direction::Column` with padding `[2,1,1,1]` and weight fill (same as `add_wizard_window`).
2. Wizard/Terminal children set both `width` and `height` to `Weight(1)`.
3. `Wizard` preferred size is at least `1×1`.
4. Regression: `shell_window_wizard_body_paints_options_after_remount`.

## Verify
- `cargo test -p semio-framework-ui --features tui-terminal shell_window_wizard_body_paints`
- `cargo build -p semio-framework-repo-cli`
- Relaunch `semio` (no args): first wizard step should list verbs (`dev`, `build`, …).
