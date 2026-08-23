# P3l Glyph and Icon Atlas Prepared Upload Census — 2026-08-23

## Verdict

The glyph/icon atlas route remains RED and is the next coherent prepared-resource packet after the
World reference-raster handoff. It is distinct from text shaping/glyph generation and from native
GPU device/surface retirement.

## Live Defects

`PreparedRenderUpload::{GlyphAtlas, IconAtlas}` still owns one contiguous `Vec<u8>` and reports
logical length rather than an exact fixed-page authority. The prepared-input retirement path pops
individual bytes from that vector, while `Gpu::apply_prepared_upload_step` sends the entire slice to
the pipeline upload method in one opportunity.

The mounted renderer creates a complete duplicate backing whenever the icon atlas is replaced and
whenever the font atlas becomes dirty:

- `self.icons.pixels.clone()` constructs `PreparedRenderUpload::IconAtlas`;
- `self.atlas.pixels.clone()` constructs `PreparedRenderUpload::GlyphAtlas`.

Bootstrap directly calls `upload_font_atlas` and `upload_icon_atlas` with the complete atlas. The
browser boot job splits phases but each upload phase still submits the whole pixel owner. The
Wasm `uploadIconAtlas` bridge also performs `pixels.to_vec()` before it publishes the runtime atlas.

Thus neither “one prepared upload per step” nor a separate boot phase is a bounded byte/page
operation. A large atlas duplicates and submits its complete allocation before the scheduler can
yield.

## Required Packet

1. Replace the two contiguous prepared variants with a shared owned fixed-page atlas authority,
   parameterized by atlas kind, exact dimensions, row pitch, page count, logical bytes, allocation
   bytes, revision, and generation.
2. The renderer must transfer/pin the current atlas backing or copy exactly one fixed page per
   admitted producer grant. It must never call complete `pixels.clone()`/`to_vec()`.
3. Dirty glyph publication must snapshot an exact atlas revision. Further glyph insertion creates a
   newer generation and cannot mutate the pixels retained by an older in-flight upload.
4. Icon replacement and the JS bridge must use fixed page admission before copying. Saturation,
   malformed dimensions, cap+1, stale generation, and duplicate page must return the exact page or
   source token.
5. GPU application must upload at most one admitted row/page per opportunity. Texture creation,
   row copies, final publication, and displaced texture retirement are distinct cursor phases.
6. Bootstrap uses the same authority and cursor as live updates; no direct whole-atlas shortcut.
7. Cancellation, supersession, device loss, receiver abandonment, app/window close, and panic route
   every retained CPU page, staging owner, and GPU texture through one bounded terminal disposer.
8. Last-valid atlas generation remains usable until the replacement is fully uploaded and atomically
   published.

## Required Discriminators

- item/byte/page/dimension maximum and maximum-plus-one;
- multiplication/row-pitch overflow and partial last page;
- source pointer/page identity and absence of full clone;
- one page/write per grant under low nonzero fuel and near/expired deadline;
- glyph mutation during upload, icon replacement during upload, ABA generation reuse, replayed page,
  duplicate ACK, and stale completion;
- queue saturation, device loss, cancellation, panic, discarded receiver, and app/window close;
- exact observed capacity and backing handback for rejected pages;
- bootstrap/live equivalence and last-valid atlas preservation.

The permanent verifier must reject contiguous prepared atlas vectors, mounted `pixels.clone()`, Wasm
`pixels.to_vec()`, direct bootstrap uploads, full-slice pipeline upload in one step, logical-length
accounting, generation-free publication, and whole-owner Drop.

## Preserved Adjacent Residuals

- glyph rasterization/text shaping itself;
- icon atlas construction from source assets and its entry metadata ownership;
- Vello draw encoding and native submit/present timing;
- full `GpuContext`/surface/device realm retirement;
- native/Wasm/browser timing and visual parity matrix.
