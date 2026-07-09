# Playground Chrome Reliability — verify log

## Root cause (2026-07-08)

The wgpu compositor renders backdrop content into an offscreen scene texture in two passes: interleaved UI/vector/world first, then a **separate raster pass** that always composites on top (`ui/wgpu/rs/lib.rs`). Navbar, footer, and dock window chrome were drawn on the backdrop `draw` list, while procedural3d's Flow node-graph uploads a full-pane `push_raster_quad` vello raster on the same backdrop tier. CPU push order did not matter — the raster pass ran after navbar/footer and painted over them. lowpoly (world-3d via `push_scene_pass`, no backdrop raster) was unaffected.

Dock tab caps partially survived because they used `push_glass` + `begin_glass_content` (glass-foreground phase on swapchain), but navbar/footer used plain `push_solid` on backdrop.

## Fix (2026-07-08) — Chrome Always-On-Top Rendering

Added `with_chrome_sink()` in `framework/renderer/wgpu/rs/lib.rs`: a single helper that routes chrome draws to the overlay `DrawList` (composited strictly last via `render_overlay`) whenever one is available, mirroring the existing left/right floating panel pattern.

Routed through `with_chrome_sink`:

- `render_navbar` / `render_footer` in `render_chrome`
- `DockState::paint_chrome` in `render_main_window` (`body_fill: false` — caps/borders only, no opaque body fill)
- `render_window_measures_rail` (folded chips + glass Window Options rail)
- `render_window_engagement_rail` (folded chips + glass Command rail)

When chrome is already the overlay list, nested widget overlays (select menus inside measure/engagement rails) use a local empty overlay slot to avoid double-borrow.

## Prior work (still in place)

1. **Registration contract** (`framework/plugin/rs/lib.rs`, `framework/core/rs/lib.rs`)
   - `AppBuilder::build_definition()` asserts non-empty unique window kinds, non-empty body keys, and layout `window_kind_id` cross-references.
   - `PanelGroup` enum (`Workbench`, `Details`, `Display`, `Settings`) replaces free-form panel group strings.

2. **All 24 plugins** use `PanelGroup::Workbench` / `PanelGroup::Details` in `panel_tab()` registrations.

## Regression history

**2026-07-07 evening:** Moving `paint_chrome` to overlay with `body_fill: true` covered all window content. Fixed by `body_fill: false` on main draw list. This follow-up keeps `body_fill: false` but moves chrome to overlay so raster cannot cover it.

## Verification

- `cargo build -p semio-framework-renderer-wgpu` — pass
- `cargo test -p ui_wgpu --lib` — 31 passed
- `cargo test -p semio-framework-renderer-wgpu --lib` — pre-existing test-module compile errors (unrelated `PanelGroup`/`ShellState` test scope issues)
- `bun ./framework/renderer/wgpu/script.ts wasm` — fails in this environment (wasm32 clang target unavailable); not caused by this change
- Before screenshot: user-provided procedural3d broken chrome (missing navbar/footer, tab caps floating)
- After: requires live dev-host visual check in IDE launch config
