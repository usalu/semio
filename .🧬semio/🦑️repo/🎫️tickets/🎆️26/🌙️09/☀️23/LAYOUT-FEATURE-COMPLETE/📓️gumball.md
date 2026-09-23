# Blueprint gumball

[Wire layout gumball commands](e431a3fa-e556-475f-9976-31003e8b6beb) stopped before editing. The commands are in the layout plugin now.

Selecting a frame on the blueprint emits `meta:utility` = `transform` and `meta:gumball` with `space: "world"`. An empty selection stays on `select` and omits the gumball. The overlay's world space is page coordinates.

- `translateSelection` → `move-frame`
- `scaleSelection` → `resize-frame` (size at least 1)
- `rotateSelection` → new `rotate-frame` (`bounds.rotation` plus the drag angle)

`cargo test -p semio-s-artifact-layout-layout --lib -- gumball translate_selection rotate_selection scale_selection kinds_match command_from_action canvas_layers_arms` — 7 passed.

## Utility arms the gumball

The earlier note tied `transform` to a non-empty selection. That is no longer the case. `LayoutWindowConfig.active_utility` defaults to `select`. A live host utility (`ViewModel.active_utility_id`) overrides it for the blueprint render. `canvas_layers` emits `meta:utility` on the blueprint, and `meta:gumball` with `space: "world"` only when that utility is `transform` and at least one frame is selected. Select keeps the pointer and omits the gumball.

Blueprint utilities are `select` and `transform`. Dragging dispatches `translateSelection`, `rotateSelection`, and `scaleSelection`, coalesced as `gumball-translate`, `gumball-rotate`, and `gumball-scale`. Translate maps to `move-frame`, scale to `resize-frame` (minimum 1), rotate to `rotate-frame` (incremental radians on `bounds.rotation`). `patchFrame` accepts `rotation` as an absolute value.

`cargo test -p semio-s-artifact-layout-layout --lib -- gumball translate_selection rotate_selection scale_selection canvas_layers` — 9 passed, 0 failed, 376 filtered out.

The blueprint paints `bounds.rotation` as a rotated quad, and hover hit-testing uses that same rotation. `cargo test -p semio-s-artifact-layout-layout --lib -- hit_test_uses_frame_rotation canvas_layers_draw_a_rotated_frame` — 2 passed.

Scale keeps the frame center: the gumball emits a move to the new origin and a resize. `scale_selection_resizes_and_keeps_a_minimum` passed.
