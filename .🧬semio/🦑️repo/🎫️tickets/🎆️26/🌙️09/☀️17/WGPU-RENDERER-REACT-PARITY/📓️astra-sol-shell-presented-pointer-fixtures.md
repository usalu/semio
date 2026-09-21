# Shell Presented Pointer Fixtures

## Native 134 evidence

Seven Shell laws mutated candidate state or geometry and then called functions that intentionally read `presented_input_geometry`. Native 134 therefore observed empty window/drop plans, no modal owner, and no panel rectangles:

- `window_action_context_fallback_uses_the_clicked_window_kind`
- `a_press_in_a_docked_pane_arms_the_activation_the_undo_chord_replays`
- `open_shell_panels_never_activate_the_window_underneath`
- `an_open_overlay_owns_every_pointer_until_it_closes`
- `an_open_panels_whole_box_owns_the_pointer_over_the_pane_it_floats_on`
- `a_press_inside_a_window_body_activates_that_window_and_notes_it`
- `a_modal_layer_or_an_unnamed_dock_row_activates_no_window`

Receipt: `🗑️generated/astra-runtime/renderer-native134-full/failures.json`.

## Fixture repair

The Shell test helper `publish_retained_input_for_test` seals the current hit and geometry candidate with the test's exact theme and acknowledges it through the normal Shell and Interpreter promotion path. The pre-existing default-theme helper delegates to it.

Each law now establishes candidate state first, then promotes it before asking a presented-input question. The command menu publishes its clicked dock-drop body. Dock activation publishes the planned intact and empty layouts. Panel reservation publishes both the dock plan and the open-anchor box. Overlay tests publish every open and closed modal transition. Wheel/escape activation publishes both the named and unnamed window plans. No production pointer predicate, modal rule, panel reserve, or activation expectation changed.

Changed files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (test-only helper)
- `…/Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs`
- `…/Shell/🧪️tests/🛰️wgpu-dock-close-reopen-journals/🦀️.rs`
- `…/Shell/🧪️tests/🛟️panel-window-reservation/🦀️.rs`
- `…/Shell/🧪️tests/🎯️wgpu-pointer-hit-ownership/🦀️.rs`
- `…/Shell/🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs`

Every changed test file passes `rustfmt --edition 2021 --check`. The shared Shell root has an independent pre-existing rustfmt proposal around `progress_presented_input_candidate`; this packet did not rewrite it. Native execution is root-owned.
