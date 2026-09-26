# Table Stepper Keyboard and Accessibility

## Contract

The shared `table-stepper-keyboard` fixture and schema define ArrowUp/ArrowRight, ArrowDown/ArrowLeft, PageUp/PageDown, Home, and End as clamped deltas over the current cell value, min, max, and step. A bound produces no action. The fixture retains pre-existing action arguments and requires the computed `delta` to be merged into them.

## Input and focus lifecycle

`KeyAction` and the native/browser converters now carry Home, End, PageUp, and PageDown rather than dropping them. The WGPU table stepper center establishes a renderer-local focus address only after the accepted scene verifies a live stepper cell. That address retains the accepted window and component generation, scene node and host, row id, and column id.

Every key resolves the row, column, current value, bounds, step, and action again from the current accepted scene. A replaced generation, retired host, missing row, changed column, or non-stepper cell clears focus and emits no action. Generic shell keys remain available when no live table stepper owns the key.

## Accessibility

Accepted Table scenes publish one virtual `spinbutton` node for each visible stepper cell. Its id is derived from host, row, and column; its label comes from the column label; its value min/max/now/text and center-third rect come from the same accepted scene used by hit testing. The focused virtual node tracks the exact accepted stepper address. Scene replacement or retirement removes both the node and its action authority.

## Oracles

The production React Table test consumes the shared fixture through the actual readonly spinbutton. It verifies keyboard deltas, clipping, bounds, original argument retention, and role/value semantics.

The native laws are:

- `table_stepper_keyboard_matches_the_shared_react_oracle_and_revalidates_the_live_cell`
- `accepted_table_stepper_centre_owns_navigation_keys_and_a_missing_row_retires_focus`
- `presented_table_stepper_focus_rebases_only_after_same_host_pixels_are_accepted`

## Validation

The focused production React Table oracle passed. The three native laws were queued in the root-owned cargo run because the shared workspace uses one coordinated native build session.
