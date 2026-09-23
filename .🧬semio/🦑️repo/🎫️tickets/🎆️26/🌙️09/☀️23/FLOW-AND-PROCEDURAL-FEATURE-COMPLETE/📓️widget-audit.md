# Two-Sided Widget Audit

Source: widget audit on 2026-09-23.

## Root cause

`computation_column_divider_x` in `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` (~605) returns `None` when either port list is empty. `computation_channel_row_divider_x_span` then paints the remaining side across the full node width.

`uses_computation_layout` (~500) is only `Computation` and `Cluster`. Slider, note, image, preview, action, and export never get the center divider. Their tracks and content fill the whole `DAG_COMPONENT_WIDTH` box.

`widget_io_ports` in `🌊️flow/🧬️schema/📸️snapshot/🦀️.rs` (~629) returns no ports for preview, action, and export, while `widget_to_dag_node` stores one input on those kinds. Paint uses the node kind, so the rectangle can still have a handle, but host helpers disagree.

`rectangle_handle_angle_toward` ignoring width and height is not the same-side bug. Insets do not collapse two populated columns.

## Fix

1. If exactly one of inputs or outputs is empty, `computation_column_divider_x` returns `Some(node.x)`. `None` only when both are empty.
2. Include Slider, Note, Image, Preview, Action, and Export in the two-column paint path, and clamp tracks and preview content to the occupied half.
3. `widget_io_ports` for sinks returns the same single input `widget_to_dag_node` stores.

## Test

In the DAG board unit tests, an output-only slider must report a divider at `node.x` and an output hit row that starts at that divider and ends at the right edge. A companion snapshot test: `widget_io_ports` on a preview returns one input.
