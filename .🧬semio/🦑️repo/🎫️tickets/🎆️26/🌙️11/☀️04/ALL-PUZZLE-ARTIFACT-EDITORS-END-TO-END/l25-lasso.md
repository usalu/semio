# L25: closed lasso commit test

## Root cause
The test, not the engine. The L8 click fix in `pointer-planners` (all screen points within click distance of the start) is present and was compiled into the failing run, and the commit step is correct. The test never configured the selection method: `BoardFocusState` defaults to the rectangle method, so `selection_drag_shape` built a box from `start` to the last point. The closed loop ends at its start, so that box is degenerate and encloses nothing, and the commit event carried an empty selection (no `node-a`). Lasso polygon hit-testing (`polygon_contains_world_box`) only runs when `selection_options.method == "lasso"`. Geometry checked: the screen loop 380..420 x 280..320 on an 800x600 view at zoom 1 encloses node-a at screen (400,300) and excludes node-b at (480,300); the first horizontal step goes right, so the drag is enclosing.

## Fix
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host/🧪️tests/🔬️board-host/🪝️pointer-plans-and-commit-turns/🦀️.rs`: added `host.focus.set_selection_options("lasso", "replace", true, false, false);` before the pointer down in the test. No engine change.

## Verification
UNVERIFIED: the sequential runner L23 was still active, so no build was started. Source-verified only.
