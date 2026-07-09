# Wgpu Floating Window Rails — verify log

## Compile

- `cargo check -p semio-framework-renderer-wgpu` — **passed** (Finished dev profile, exit 0)
- Parallel `cargo test` / wasm trunk builds hit target-dir lock contention in this environment (unrelated to changes)

## Behavioral changes verified by code review vs React (`ui/js/react/index.tsx`)

- Window content renders at full body rect before measures/engagement overlays
- Measures/engagement rails use `GlassTier::WindowOptions`, positioned with `theme.gap_standard` inset
- Rails no longer mutate `content` rect (no squeeze)
- Hit targets registered after window content (rails win overlap)
- Widths from tokens: `window_measures_default_width` (224px), `window_engagement_max_width` (448px), resize clamp `panel_min_width`/`panel_max_width`
- Auto-height cards from `measure_window_measures_body_height` / `measure_engagement_body_height`
- Measures tree siblings stack via threaded `y` cursor

## Manual visual check

Run **Framework Renderer Wgpu Dev** from launch.json and confirm:

1. Command / Window Options panels float over window canvas with glass blur
2. Window content is not squeezed when panels are open
3. Side panels still render above window chrome overlays
