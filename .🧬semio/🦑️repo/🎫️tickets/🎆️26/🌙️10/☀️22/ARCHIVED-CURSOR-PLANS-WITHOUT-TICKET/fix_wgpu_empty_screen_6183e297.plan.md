---
name: Fix Wgpu Empty Screen
overview: Fix three GPU-side bugs that make the wgpu shell render an empty screen (buffer aliasing across draw layers, wrong scissor Y-flip, glyph atlas never re-uploaded), plus a hover-order bug, then strengthen the E2E paint assertion so it can't pass on a blank canvas.
todos:
 - id: fix-buffer-aliasing
   content: Single concatenated buffer upload with per-layer draw ranges in draw.rs render + render_overlay
   status: completed
 - id: fix-scissor-flip
   content: Remove Y-flip in ScissorRect::from_rect, update unit tests
   status: completed
 - id: fix-glyph-upload
   content: FontAtlas dirty flag + per-frame re-upload of glyph atlas
   status: completed
 - id: fix-hover-order
   content: update_hover before clear_frame in AppRuntime::frame
   status: completed
 - id: strengthen-e2e
   content: Real pixel-based paint + chrome-structure assertions in E2E script
   status: completed
 - id: verify
   content: Rebuild wasm, cargo test, run 25-plugin suite, inspect screenshots for navbar/footer/panels/text
   status: completed
isProject: false
---

# Fix Wgpu Empty Screen

## Root causes found

Screenshots from the last E2E run ([.repo/🎫️/26/07/04/WGPU-PLAYGROUND-E2E/screenshot-s.png](.repo/🎫️/26/07/04/WGPU-PLAYGROUND-E2E/screenshot-s.png)) show a flat dark canvas — the suite "passed" because the paint check only inspects compressed PNG byte variance.

### Bug 1 — All draw layers alias one GPU buffer (the main cause)

In `UiPipelines::render` ([ui/wgpu/rs/draw.rs](ui/wgpu/rs/draw.rs) ~line 1329), each layer calls `frame_buffers.ui_instances.upload(...)` inside the render-pass loop. `GrowBuffer::upload` does `queue.write_buffer` into the **same buffer** every time. All `write_buffer` calls execute at submit time _before_ any recorded draw call, so every layer's data is overwritten by the last layer's — every draw call renders the final layer's (often tiny) contents. Same for `vector_vertices`. Since the scissor work split the frame into many layers, the whole UI collapses.

**Fix**: concatenate all layers into a single upload per frame and record ranges:

- Pre-pass: build `Vec<(scissor, instance_range)>` while appending all layers' `ui_instances` into one `Vec`, same for vector vertices.
- One `upload` call per buffer, then in the pass loop `pass.draw(0..6, range.start..range.end)` (instanced) and `pass.draw(vrange, 0..1)` per layer with its scissor.
- Apply the same fix inside `render_overlay`.

### Bug 2 — Scissor rect Y is flipped

`ScissorRect::from_rect` ([ui/wgpu/rs/draw.rs](ui/wgpu/rs/draw.rs) line 84) computes `y = screen_h - rect.y - rect.h` (OpenGL convention). WebGPU `set_scissor_rect` uses top-left framebuffer origin, which matches UI coordinates directly. Result: panel/window content is clipped to the wrong region.

**Fix**: `y = rect.y` (no flip); drop the now-unneeded `screen_h` plumbing or keep for clamping only. Update the `draw.rs` unit tests.

### Bug 3 — Glyph atlas uploaded before any glyph exists

`gpu.upload_font_atlas(&atlas)` runs once at boot ([framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs) line 248), but glyphs are rasterized lazily into `atlas.pixels` during `draw_text`. The GPU glyph texture stays all-zero, so **no text ever renders**.

**Fix**: add a `dirty: bool` to `FontAtlas` ([ui/wgpu/rs/text.rs](ui/wgpu/rs/text.rs)), set in `rasterize_glyph`; in `AppRuntime::frame` after `render_chrome`, if dirty, call `gpu.upload_font_atlas` and clear the flag.

### Bug 4 — Hover computed after hit targets are cleared

`frame()` calls `input.clear_frame()` (clears `hit_targets`) _before_ `input.update_hover(...)`, so `hovered_id` is always `None`.

**Fix**: call `update_hover` first (against last frame's hit targets), then `clear_frame`.

## Verification (make it impossible to pass blank)

- Strengthen the paint check in [.repo/🎫️/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts](.repo/🎫️/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts): in-page `page.evaluate` draws the WebGPU canvas into a 2D canvas and reads real pixels via `getImageData`; assert (a) ≥1% of pixels differ from the background color and (b) a navbar-height strip at top and footer strip at bottom differ from the window body (chrome structure present).
- Rebuild wasm (`bun ./framework/renderer/wgpu/script.ts wasm`), run `cargo test -p ui_wgpu`, run the 25-plugin suite, and visually inspect regenerated `screenshot-s.png` / `screenshot-flow.png` for navbar, footer, floating panels, and text.
