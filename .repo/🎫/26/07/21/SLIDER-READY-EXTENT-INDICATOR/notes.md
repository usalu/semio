# Slider Ready Extent Indicator

## Behavior
- Sliders keep a fixed `[min, max]` range.
- Optional `ready` is the absolute preloaded extent on that range.
- UI draws a highlighted segment from the knob to `ready` (right of the thumb).
- Interaction clamps to `ready` when set.

## Wiring
- `ui` Slider: `ready?: number` + `data-slot="slider-ready"`.
- `WindowMeasure::Slider.ready: Option<f64>` (serde default / skip none).
- React `WindowMeasureSlider` and engagement sliders pass `ready` through.
- wgpu `render_slider` draws the ready segment with `theme.border_emphasized`.
- Puzzle 3d fill-count measure: `max = PUZZLE3D_FILL_COUNT_MAX`, `ready = available_count`.

## Verification
- UI vitest: ready-extent + rest styles pass.
- `fill_count_measure_shows_planning_progress_while_precompute_incomplete` asserts fixed max + ready.
- `ui_wgpu` check and `puzzle-plugin` measure test compile/pass in ticket target dir.
