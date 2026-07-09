---
name: Wgpu Glass Panel Crispness Fix
overview: "Fix two glass-blur bugs: (1) the blurred backdrop shows a shrunk/repeating artifact because of a mis-sized scratch texture in the downsample chain, and (2) side-panel and window-options-rail content is fully erased by the glass composite because it draws its own content before the glass background is composited, unlike context menus which already do this correctly."
todos:
 - id: reopen-ticket
   content: Reopen ticket 26/07/06/WGPU-PANEL-GLASS-BLUR via MCP ticket_reopen
   status: completed
 - id: fix-blur-scratch
   content: Give blur_scratch a full mip chain in SceneColorTarget; copy per-level; bind per-level view in run_blur_chain (ui/wgpu/rs/lib.rs)
   status: completed
 - id: drawlist-foreground
   content: Add begin_glass_content/end_glass_content + foreground_of tagging to DrawList and DrawLayer (ui/wgpu/rs/lib.rs)
   status: completed
 - id: skip-foreground-backdrop
   content: Skip foreground-tagged layers when building the pre-blur scene_color backdrop in render_scene_content
   status: completed
 - id: foreground-pass
   content: Add post-composite foreground render pass in composite_to_swapchain reusing render_interleaved_layers/upload_world_passes, targeting swapchain with LoadOp::Load
   status: completed
 - id: migrate-panel
   content: Wrap render_floating_panel content (borders, tabs, widget body) with glass-content markers (framework/renderer/wgpu/rs/lib.rs)
   status: completed
 - id: migrate-rails
   content: Wrap render_window_measures_rail/render_window_measure and render_window_engagement_rail (+input/control helpers) content with glass-content markers
   status: completed
 - id: tests
   content: Extend existing DrawList/UiPipelines tests for foreground layer tagging and skip-from-backdrop behavior
   status: completed
 - id: verify
   content: Rebuild wasm, rerun headless Playwright screenshot checks, visually verify panel/rail crispness and correct non-repeating blur in light/dark themes
   status: completed
 - id: close-ticket
   content: Close ticket with summary and full list of touched files
   status: completed
isProject: false
---

# Wgpu Glass Panel Crispness Fix

## Root cause 1 -- backdrop "repeats and is smaller" (especially near the top)

`SceneColorTarget` in [ui/wgpu/rs/lib.rs](ui/wgpu/rs/lib.rs) (`draw` module, struct at line 416, `ensure` at 427) creates `blur_scratch` as a **single-mip texture sized to the full base resolution** (lines 458-471). Each blur iteration in `run_blur_chain` (line 2820) calls `copy_mip_to_blur_scratch(encoder, src_mip)` (line 540), which copies the source mip (whose extent shrinks by half each level) into the top-left corner of that fixed-size scratch texture, then the downsample fragment shader samples the **entire** `blur_scratch_view()` (UV 0..1 spans the full base resolution) via the bind group at line 2846.

Only mip 1's copy (`src_mip = 0`, full-size) fills the whole scratch texture correctly. From mip 2 onward, the copy only fills a shrinking top-left fraction of the scratch texture; the remaining area still holds **stale data from the previous iteration's larger copy**. Sampling UV 0..1 over that mismatched texture produces exactly the "smaller and repeating" artifact -- worse at higher mip levels (used for the stronger blur that panels/menus with larger `blur_px` request via `Theme::glass_mip_level`).

### Fix

Give `blur_scratch` its own full mip chain that matches `texture`'s dimensions at every level, so each iteration's copy always exactly fills the view being sampled:

- `blur_scratch` texture descriptor (line ~458): change `mip_level_count: 1` to `mip_level_count: SCENE_MIP_LEVELS`.
- Add `blur_scratch_mip_views: Vec<wgpu::TextureView>` (mirrors `mip_views`, one per level) built the same way as lines 485-496.
- Replace `blur_scratch_view()` (line 528) with `blur_scratch_mip_view(&self, level: u32) -> &wgpu::TextureView`.
- `copy_mip_to_blur_scratch` (line 540): copy into `mip_level: src_mip` on the destination (currently hardcoded to `0`), so source and destination extents always match exactly.
- `run_blur_chain` (line 2820): bind `scene.blur_scratch_mip_view(src_mip)` instead of `scene.blur_scratch_view()` at line 2846.

This is self-contained to the `draw` module in `ui/wgpu/rs/lib.rs`; no call-site changes elsewhere.

## Root cause 2 -- panel/rail content is fully erased by the glass tint

The glass fragment shader (`GLASS_SHADER`, line ~4512) returns `fill_alpha` close to `1.0` everywhere inside the rounded rect (only the AA edge fades out), and the pipeline blends with `wgpu::BlendState::ALPHA_BLENDING`. That means compositing a glass region is effectively an **opaque replace** of whatever was already drawn at that pixel -- by design, since it's meant to draw a frosted background _before_ any foreground content.

`composite_to_swapchain` (line 2698) runs in this order: blit `draw`'s already-rendered scene to the swapchain, **then** composite `draw.glass_regions` on top, **then** (if present) composite `overlay.glass_regions` and finally render `overlay`'s own content via `render_overlay` (line 3013, `LoadOp::Load`) crisply on top of its glass.

- Context menus / dropdowns / palette / select-popup push both their glass region and their row content onto `overlay` (`framework/renderer/wgpu/rs/lib.rs` lines 9178, 9243, 9954, 9987, 10034) -- so they get the correct order: glass first, crisp content after. This is why menus already look right.
- The side panel (`render_floating_panel`, line 8531; glass pushed at line 8559) and the window-options rails (`render_window_measures_rail` line 9366 / glass at 9412, `render_window_engagement_rail` line 9649 / glass at 9706) push **both** their glass region and all of their own content (borders, tabs, header, chrome-group buttons, the entire scrollable widget body via `render_ui_node`) onto `draw`. Because `draw`'s content is rendered and blitted to the swapchain _before_ `draw.glass_regions` is composited, the opaque glass pass completely overwrites the panel's own crisp content -- this is why panels currently show only a blurred/tinted rect with no visible text or controls.

A naive fix (redirect panel/rail content to `overlay`) would break embedded 3D content: `render_ui_node` (line 2891) supports `UiNode::ComponentScene`, which calls `push_scene_pass` -- but `render_overlay` (line 3013) only renders flat `ui_instances`/`vector_vertices`, with no depth-buffer/3D-world support. A panel tab that ever shows a live 3D preview would silently stop rendering.

### Fix -- general "glass foreground" support in `DrawList`

Add a proper foreground tier to `DrawList` (`ui/wgpu/rs/lib.rs`) that supports the same content types as the main scene pass (UI, vector, and 3D world), so panel/rail content (including any future embedded 3D scene) renders crisply _after_ glass compositing instead of being folded into the pre-blur backdrop:

1. **`DrawList`**: track which layers belong to "foreground of glass region N", mirroring how `scene_passes` already record `layer_index`/`ui_watermark`/`vector_watermark` (line 750). Add:
   - `push_glass` (line 774) returns a region index/handle.
   - `begin_glass_content(handle)` / `end_glass_content()` push a new layer (like `push_scissor`/`pop_scissor` at lines 725-748) tagged with `foreground_of: Option<usize>`.
   - Store enough per-region bookkeeping (e.g. `pub glass_foreground_layers: Vec<usize>` or a `foreground_of` field directly on `DrawLayer`) so the renderer can filter layers by "is this pre-blur backdrop or post-composite foreground".

2. **`UiPipelines::render_scene_content`**: when calling `build_layer_batches(draw)` (line 1420) to populate the pre-blur `scene_color`, skip layers tagged `foreground_of: Some(_)` -- so panel/rail content never bleeds into what gets sampled as "behind" it, and the backdrop shown through the glass genuinely reflects what's visually behind the panel.

3. **`UiPipelines::composite_to_swapchain`**: immediately after compositing `draw.glass_regions` (line 2715), add a new step that reuses the existing interleaved renderer (`render_interleaved_layers`, `upload_world_passes`) but scoped to only the foreground-tagged layers, targeting the swapchain view with `LoadOp::Load` (same pattern as `render_overlay`, but with 3D-world support). This draws panel/rail content -- text, icons, buttons, and any embedded `ComponentScene` -- crisply on top of the already-composited glass background.

4. **`framework/renderer/wgpu/rs/lib.rs`**: wrap the content-emitting calls with the new begin/end markers, tied to each function's `push_glass` call:
   - `render_floating_panel` (8531): borders (8575-8578), tab bar (8580-8639), scissor + `render_ui_node` body (8652-8681).
   - `render_window_measures_rail` (9366) and `render_window_measure` (9502): header buttons (9414-9467), measure rows/widgets (9474-9497).
   - `render_window_engagement_rail` (9649) and its helpers `render_engagement_input`/`render_engagement_control`: header, options, input/control widgets, status rows, possible-engagement chips (9707-9809).

## Verification

- `cargo test -p ui_wgpu -p framework_renderer_wgpu` for the new `DrawList`/render-pipeline unit tests (extend existing tests near `draw_list_push_scissor_splits_layers` / `scene_pass_records_layer_watermarks`, per repo convention of extending existing test files rather than adding new ones).
- Rebuild wasm (`bun ./framework/renderer/wgpu/script.ts wasm`) and re-run the headless Playwright check used previously (screenshot + pixel sampling) to confirm:
  - The blurred backdrop behind a panel/menu shows a correctly-scaled, non-repeating image at every blur strength.
  - Side panel tabs, window-options rails, and their text/icons/buttons are pixel-crisp, with only the area behind them blurred+tinted.
- Manual visual check in light and dark themes: side panel, window measures rail, window engagement rail, plus a spot-check that context menus/dropdowns (already working) remain unaffected.

## Ticket

Reopen `26/07/06/WGPU-PANEL-GLASS-BLUR` for this follow-up fix and close it again with a summary once verified, per repo ticket workflow.
