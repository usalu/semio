# wgpu-ui API for lane wgpu-os

Crate surface: `ctx.draw` is the wgpu `DrawList` (`🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🖊️draw-list-queries-and-push/🦀️.rs`). Everything below already exists except the one rename marked NEW.

## Removed (do not call)

- `DrawList::begin_silhouette_clip(&[Rect])`: deleted, no replacement name. Frames no longer clip to a union.
- `DrawList::end_silhouette_clip()`: renamed to `end_clip()`, see below.
- Scene builder (`🖌️render`) `begin_silhouette_clip` / `end_silhouette_clip`: deleted. A frame clips with `push_scissor` / `pop_scissor`.

## Frame rectangular clip

```rust
ctx.draw.push_scissor(frame_rect);   // Rect = the whole frame (WindowFrame::content_clip_rect())
// ...content: solids, text, render_body(...)
ctx.draw.pop_scissor();
```

The content fills the complete frame edge to edge, cap and footer rows included. Do not subtract chip rows.

## Scanline clip end (NEW name)

`begin_scanline_clip(&[[u32;4]]) -> bool` stays (canvas2d scenes). Its closing call `end_silhouette_clip()` is renamed `end_clip()` (same semantics: pops the innermost clip region). Only `Scenes/.../scene-nodes-and-image-upload` uses it.

## Chip glass (paint after the frame content, inside the frame)

```rust
let glass = ctx.draw.push_glass([chip.x, chip.y, chip.w, chip.h], 0.0, theme.glass(Level::Window));
ctx.draw.begin_glass_content(glass);
// chip label, icon, selected fill, ...
ctx.draw.end_glass_content();
```

- Call it after the frame content (and after `pop_scissor`, or inside the frame scissor; both stay within the frame). The glass samples everything painted before it, so the content shows through, blurred and tinted.
- Level: use one level above the frame content surface. Content surface is `theme.surface(Level::Base)` (canvas) or `Level::Pane`; chips use `theme.glass(Level::Window)` over a base canvas, otherwise the next level up (`Level::Pane` over `Window`, and so on). Same call the Dock already uses for tab groups.
- Chip rect comes from `WindowFrame::chip_regions()` (pure geometry, element `🔲️WindowFrame/🦀️.rs`). Chip glass rects always lie inside the frame rect.
- Reduced transparency / opaque fallback: draw `push_solid(chip_rect, theme.surface(level))` instead of `push_glass` (no other change).

## Frame border

```rust
ctx.draw.push_shared_border(frame_rect, stroke, color, SharedBorderState::Normal);
```

Frame outline: all four edges. Chip inner edges only (the edges not on the frame outline):

```rust
ctx.draw.push_shared_border_edges(chip_rect, stroke, color, state, [top, right, bottom, left]);
```

with `top`/`bottom` = false for the edge that lies on the frame outline, and `left`/`right` = false where the chip touches the frame's left/right outline. Coincident edges resolve to one divider through the shared border compositor; no extra call.
