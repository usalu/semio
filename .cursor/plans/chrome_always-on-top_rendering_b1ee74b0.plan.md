---
name: Chrome always-on-top rendering
overview: Fix procedural3d's missing navbar/footer/window-chrome by routing all "must always be on top" chrome through the same guaranteed-last compositing path already used successfully by the floating side panels, instead of the fragile plain backdrop layer that any window's raster content can silently paint over.
todos:
 - id: repro
   content: Boot dev host, screenshot procedural3d and lowpoly before the fix to confirm exact missing chrome elements
   status: completed
 - id: chrome-sink
   content: Add chrome_sink helper in framework/renderer/wgpu/rs/lib.rs and route render_navbar/render_footer through it
   status: completed
 - id: dock-chrome
   content: Route DockState::paint_chrome border+cap call (render_main_window) through chrome_sink
   status: completed
 - id: rails
   content: Route render_window_measures_rail and render_window_engagement_rail glass through chrome_sink
   status: completed
 - id: verify
   content: Re-screenshot procedural3d/lowpoly/other component kinds, run cargo test for ui/wgpu and framework/renderer/wgpu
   status: completed
 - id: ticket
   content: Update verify-log.md and close/reopen the PLAYGROUND-CHROME-RELIABILITY ticket via repo MCP with full file summary
   status: completed
isProject: false
---

# Chrome Always-On-Top Rendering

## Root cause (confirmed via code trace)

The wgpu compositor has two structurally different destinations for content, and the current code mixes them inconsistently:

- **Backdrop layer** (`draw: &mut DrawList`): rendered into an offscreen "scene" texture in `render_scene_content` (`ui/wgpu/rs/lib.rs:3000-3199`). Inside that phase, the **interleaved UI/vector/world pass runs first**, then a **separate raster pass** (`draw_raster_layers`, `ui/wgpu/rs/lib.rs:3084-3136`) always runs **after** it with `LoadOp::Load`, regardless of CPU push order. Any full-pane raster quad (e.g. `procedural3d`'s node-graph, painted via `vello` and uploaded with `push_raster_quad([inner.x, inner.y, inner.w, inner.h], ...)` at `framework/renderer/wgpu/rs/lib.rs:2360-2364`) can visually cover anything else drawn earlier on the same backdrop layer/scissor region.
- **Overlay `DrawList`**: composited strictly last, in `render_overlay` (`ui/wgpu/rs/lib.rs:3765-3827`), after the scene blit, after `draw`'s own glass regions/foreground, and after the overlay's own glass regions/foreground. This is the one truly-guaranteed-last destination in the whole pipeline.

Today:

- `render_navbar` (`framework/renderer/wgpu/rs/lib.rs:10293`) and `render_footer` (`:10492`) call `draw.push_solid(...)` / `chrome_text(...)` directly on the **backdrop** layer — no glass, no overlay. They are drawn from `render_chrome` at `:10105-10106`, using `draw`.
- Floating left/right panels already correctly prefer the overlay list when available (`render_chrome`, `:10091-10104`): `if let Some(panel_draw) = overlay_slot.as_deref_mut() { render_left_panel(panel_draw, ...) } else { render_left_panel(draw, ...) }`. This is why the Document/Catalogue and Inspection panels render cleanly in the screenshot even though the main window's node-graph doesn't.
- The dock window cap/border chrome (`DockState::paint_chrome` → `render_stack`, `framework/renderer/wgpu/rs/lib.rs:1226-1420`) already uses `ctx.draw.push_glass(..., GlassTier::Toolbar, ...)` + `begin_glass_content` for the tab/Focus/Close bar, so it composites via `draw`'s own glass-foreground phase (after `draw`'s own raster pass finishes) — safer than navbar/footer, but still tied to `draw` rather than the outer overlay.

Net effect: navbar and footer have no structural protection at all, so any plugin whose main window content includes a full-pane raster/vello quad (procedural3d's Flow node-graph) can blank out the navbar/footer pixels above/below it, while plugins using only `push_scene_pass` (world-3d, e.g. lowpoly) never hit the vulnerable raster pass and look fine.

## Fix strategy — one enforced "chrome sink", reusing the pattern that already works

Introduce a single helper that all "must render after every window body" chrome goes through, mirroring the existing (working) left/right panel pattern, instead of leaving each chrome function to decide for itself which `DrawList` to use:

```rust
/// 🛡 Chrome content must always win over window bodies; route it to the
/// overlay compositing phase (guaranteed last) whenever one is available.
fn chrome_sink<'a>(draw: &'a mut DrawList, overlay: &'a mut Option<&'a mut DrawList>) -> &'a mut DrawList {
    overlay.as_deref_mut().unwrap_or(draw)
}
```

Route through it:

1. `render_navbar` / `render_footer` calls in `render_chrome` (`:10105-10106`) — pass the overlay-backed sink instead of `draw`.
2. The `DockState::paint_chrome(&mut dock_ctx, canvas, false)` call in `render_main_window` (`:10902-10912`) — build `dock_ctx.draw` from the sink so window border + cap/Focus/Close chrome also gets the guaranteed-last placement (currently only "safer", not fully guaranteed, since it still depends on `draw`'s own internal raster-vs-glass ordering).
3. `render_window_measures_rail` / `render_window_engagement_rail` (`:11400s`/`:11700s`), which already receive both `draw` and `overlay` params but currently push their `GlassTier::WindowOptions` rail onto `draw` unconditionally (`:11471-11479`) — switch them to the sink too for consistency.

This does not change navbar/footer's visual style (they stay plain opaque solids, no blur/tint); it only changes which `DrawList` receives their draw calls. Left/right panels are untouched (already correct).

## Verification plan (execution phase, not part of this plan-mode research)

1. Reproduce live: boot the dev host, open `procedural3d`, screenshot before the fix; open `lowpoly` and one other node-graph-based plugin as a baseline/control.
2. Apply the `chrome_sink` refactor.
3. Re-screenshot `procedural3d`: confirm navbar (logo/title/examples/fullscreen), footer, and the Flow window's border/cap all render correctly over the node-graph raster content.
4. Spot check a handful of other plugins with different component kinds (world-3d, canvas-2d, text-editor, gis-map — all of which also call `push_raster_quad`, see `framework/renderer/wgpu/rs/lib.rs:3310-3314, 3570-3574, 5307-5308, 5610-5611`) to make sure nothing regresses.
5. `cargo test` for `ui/wgpu` and `framework/renderer/wgpu` crates (DrawList/glass region unit tests already exist there).
6. If the Flow window's cap/border is still visually different from Preview's after the chrome_sink fix (i.e. a second, independent bug), drill into `render_stack`'s cap-rendering for that specific stack/path — but do not pre-emptively rewrite dock layout logic without confirming this with a live screenshot first, since `paint_chrome` renders every stack in `self.dock.root` uniformly today with no code path that special-cases single-tab stacks for border/cap suppression.
7. Update `.repo/🎫/26/07/07/PLAYGROUND-CHROME-RELIABILITY/verify-log.md` with root cause, fix, and before/after screenshots; reopen/close the ticket per repo MCP workflow with the full list of touched files.

## Files to change

- [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs): add `chrome_sink` helper; update `render_chrome` (navbar/footer calls), `render_main_window` (`paint_chrome` call), `render_window_measures_rail`, `render_window_engagement_rail`.
- `.repo/🎫/26/07/07/PLAYGROUND-CHROME-RELIABILITY/verify-log.md`: document this follow-up regression + fix.

No changes are needed to `ui/wgpu/rs/lib.rs` (the compositing primitives — `DrawList`, `push_glass`, `render_overlay`, `draw_raster_layers` — are already correct; the bug is purely in which list `framework/renderer/wgpu` chooses to draw chrome onto), and no `AppDefinition`/`AppBuilder`/`PanelGroup` changes are needed this time (that hardening from the earlier fix already stands on its own).
