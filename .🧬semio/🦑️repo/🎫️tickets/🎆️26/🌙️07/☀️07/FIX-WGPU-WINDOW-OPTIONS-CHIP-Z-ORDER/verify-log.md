# Verify Log — Fix Wgpu Window Options Chip Z-Order

## Code change

- `render_window_measures_rail` folded "Window Options" chip: always `render_chrome_group(draw, …)` (was preferring `overlay`).
- `render_window_engagement_rail` folded "Command" chip: same fix.

## Build

```
cargo check -p semio-framework-renderer-wgpu
```

Failed due to **pre-existing** errors in `framework/plugin/rs/lib.rs` (`world3d_scene_extended` missing `lod_json` / `chunking_json` args). Unrelated to this change.

The edited lines are a straight draw-list target swap with no new symbols or signatures.

## Expected runtime behavior

1. Open wgpu app with a window that has measures/engagement.
2. Open left or right side panel so it overlaps the folded chip (top-right "Window Options" or top-left "Command").
3. **Before fix:** chip visible on top of panel glass (overlay backdrop composites after panel glass).
4. **After fix:** chip occluded by panel glass (chip on `draw`, panels on `overlay`).
5. Unfolded rails unchanged (`draw.push_glass` + `GlassTier::WindowOptions`).
6. Hit-test: side panel hits register after window chip hits in `render_chrome` — panel still wins pointer events in overlap.
