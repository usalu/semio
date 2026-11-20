# A6 TUI Shared Borders: Drag Preview and Docking Vectors

## Root causes

1. `semio-ui-tui-core` docking law: the neutral vectors in `⌨️tui/🧲️docking/🧪️tests/🔣️.json` were stale.
   They still used the old gap model, where the splitter cell belonged to neither side (for example left `[10,20,50,40]`, right `[61,20,49,40]`, with column 60 in neither).
   Commit `0b2d54d` (Nov 7) moved `DockPlacement::preview` to the shared-splitter model: both sides include the shared cell.
   The engine law `🧪️tests/🔬️engine` checks this model against Taffy and against the committed `solve_window_layout` rect, so the code is right.
   The independent SQLite oracle in the TS twin (`🖱️ui/🧪️tests/📦️crate-boundaries/🟦️.ts`) encoded the same stale gap formula.
2. `semio-ui-tui-engine` drag-preview laws (3 failures): the preview was drawn by `paint_window_tab_preview`, a standalone chip painted over the pre-drop frame.
   The committed frame joins the chip's border with its neighbours in two ways:
   - chip-to-chip shared columns inside `paint_window`;
   - shared seams with other outlined chrome in `compose_shared_borders`, such as the navbar hairline on canvas row 1 and adjacent windows.
   So the committed frame shows tees like `┬` where the preview showed `┌`.
3. `narrow_overflow_preserves_the_active_chip_and_its_controls`: the assertion `interior_width + 2 == 12` was written on Oct 19 for the old chip overhead of 2.
   Since `0b2d54d`, adjacent chips share a border (overhead 1).
   In 18 cells, `‹` + ` Source ⤢ ✕ ` + `›` now fits without elision: 1 + 2 + 13 + 2 = 18.
   The correct width is therefore the natural 14.

## Fix

- Vectors: corrected the 8 edge previews (100x40 and 5x3 areas) to the shared-splitter model, for example left `[10,20,51,40]` and right `[60,20,50,40]`. The SQLite oracle formula in the TS twin now uses the same model.
- Preview = the exact committed tab, by construction:
  - `chrome::project_window_layout` (new, pub) returns the visible windows that a mount of a layout would paint, each with the state its node would carry. It shares the extracted `StackSlot`, `stack_tab_states`, `apply_stack_slot` and `layout_peers` helpers with `mount_window_layout`, so the two cannot drift.
  - `propose_dock` stores the projected canvas windows (`DockPreview { canvas, windows }`).
  - `paint` runs a second `PaintPass` into a scratch buffer, painting the canvas with the projected windows in place of its children. It then runs the same `compose_shared_borders` and copies the proposed chip rect back with the new `CellBuffer::copy_rect`.
  - `paint_window_tab_preview` was removed.
- Updated the stale narrow-overflow constant from 12 to 14.

## Files

- `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧲️docking/🧪️tests/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️crate-boundaries/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🖥️chrome/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🔲️cell/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/⌨️tui/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/⌨️tui/⚙️engine/🖱️mouse-and-hover/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/⌨️tui/⚙️engine/🖌️paint-and-gutters/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/⌨️tui/⚙️engine/⏱️frame-layout-and-overlays/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🎯️targets/⌨️tui/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🪟️drag-preview/🦀️.rs`

## Results

- TS twin (`bun test crate-boundaries -t docking`): 2 pass, 0 fail.
- `semio-ui-tui-core` (log `🗑️generated/a6-core-1.log`): `test result: ok. 60 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out` (doctests: ok. 0 passed). This includes `docking::tests::shared_docking_contract_matches_neutral_vectors_without_frame_dependency`.
- `semio-ui-tui-engine` (log `🗑️generated/a6-engine-1.log`): `test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out` (all 8 drag-preview laws).
- The engine build recompiled core, including `CellBuffer::copy_rect`, with no warnings in the touched files.
- The core run started before `copy_rect` (an additive core API) was added. The engine run compiled and exercised it.
