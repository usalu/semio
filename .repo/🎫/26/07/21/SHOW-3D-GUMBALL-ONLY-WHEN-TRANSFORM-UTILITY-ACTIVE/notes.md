# Notes

## Root cause

Puzzle 3D treated an unset host `active_utility_id` as `move` (`PUZZLE3D_DEFAULT_UTILITY`). The shell starts with no utility pressed and Escape clears the active utility to null, but the plugin fell back to `move`, so `gumballActive` stayed true whenever objects were selected.

## Fix

Set `PUZZLE3D_DEFAULT_UTILITY` to `""`. Existing `puzzle3d_gumball_active` already requires `move|rotate|scale`, so selection alone no longer shows the gumball.

## Verification

- `cargo test -p puzzle-plugin --lib gumball_active_only_for_transform` — pass
- `cargo test -p puzzle-plugin --lib main_window_utilities_lead_with_move` — pass
