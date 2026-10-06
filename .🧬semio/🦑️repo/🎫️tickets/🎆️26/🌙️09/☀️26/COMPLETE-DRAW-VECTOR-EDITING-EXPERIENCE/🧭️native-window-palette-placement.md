# Native Window Palette Placement Follow-up

The current React fitting-pane change has a shared neutral safe-area kernel, but native window palettes still place their chips and bodies independently. Their current paint and hit registries use authored body anchors without shell-panel occupancy. This is an observed source gap, not a post-repair end-user runtime claim.

## Required Atomic Placement

`window_pane_chip_rect` derives only the label chip from the window body. `paint_window_pane_chips_step` uses that rectangle for both glass and deferred pane hits. Actions and Search derive separate 300px body rectangles under their chips. Measures derives its variable-width body under the top-right chip. Projection derives its 300px body above the bottom-right chip. Utilities lays out a horizontal rail directly to the right of its bottom-left chip. Moving only a chip would break the relationship to its actual open body.

The existing `open_floating_panel_rects` reads current anchor state and geometry before panel paint, so a complete footprint can consume current occupancy without previous-frame panel rectangles. A shared placement record must own the chip and open body/rail footprint, apply one valid displacement, and feed both paint and hits. Retained content measurement can change after layout; the record must cause a viewport restart consistently when the admitted body moves. The body painters must preserve document ownership when measurement or no-fit refuses placement.

The existing safe-area geometry contract admits fitting placements only, retaining the original box when neither authored axis fits. A reachable no-fit behavior still needs an explicit neutral contract rather than hiding a control or treating zero displacement as successful clearance. Expanded/mobile and intrinsic-body constraints remain separate open acceptance.

## Sources Read

- Shell native source: `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `window_pane_chip_rect`, `paint_window_pane_chips_step`, `paint_window_pane_body_step`, `window_measures_rect`, `window_projection_rect`, `open_floating_panel_rects` and the utility rail painter.
- Existing native runtime laws: `🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs`, including real font measurement, glass painting, deferred hit registration and fold accessibility.
- Existing common safe-area kernel and its 31-case neutral corpus were accepted in the preceding stage. No production palette placement source was changed during this audit.

Full renderer recheck 82337 is compiling on its original native build budget. It does not establish palette occupancy behavior unless new actual laws exercise that behavior.
