# Puzzle 5d — 2D window left

## Problem

The default `edit` and `view` mode layouts used `create_default_layout` with `world3d` first and `board2d` second in a row, putting the 2D board on the right.

## Fix

Swap the window order to `[board2d, world3d]` with sizes `[40.0, 60.0]` so the board keeps two-fifths width on the left and the world keeps three-fifths on the right.

## Files

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/.../✏️editor/🎭️modes/✏️edit/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/.../👁️viewer/🎭️modes/👁️view/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/.../✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs`

## Verification

`cargo test -p semio-s-artifact-puzzle-5d default_layout_is_board_left_two_fifths_and_world_right_three_fifths` — passed.
