# P3j Prepared Raster Producer Census — 2026-08-23

## Scope

This census covers only the prepared-pixel ownership boundary immediately upstream of the accepted fixed raster table. The accepted table, atlas/icon/glyph/surface/Vello paths, platform submission, and runtime architecture are outside this packet.

## Baseline consumer boundary

`PreparedRenderUpload::Raster` is consumed by `Gpu::apply_prepared_upload_step`, which delegates to the accepted generation-keyed raster upload authority. That consumer already limits a GPU write opportunity to one row/page of at most 16 KiB. Its current input, however, is still one contiguous `Vec<u8>`.

At the census boundary there were exactly two production constructors of `PreparedRenderUpload::Raster`:

1. Product renderer glue moves a Canvas/Paint `PendingRasterUpload` into the prepared input.
2. Infinite World `World3dBuildContext::ensure_world_plane_texture` clones a cached reference image slice with `pixels.to_vec()`.

## Canvas and Paint producer path

Production queue callers are:

- Scene canvas rendering at three call sites in `Scenes/🧊️component.rs`.
- Interpreter image rendering at five call sites in `Interpreter/🧊️component.rs`.
- One Interpreter dimension probe also calls `decode_canvas_image` and discards the complete decoded pixel owner merely to read width and height.

The live owner chain is:

`data URL -> base64 decode Vec -> image DynamicImage -> RGBA Vec -> pixels[..expected].to_vec() -> pending Vec -> PreparedRenderUpload::Raster Vec -> raster table page writes`.

The queue preflights key/source lengths and the final byte count, but materializes the encoded and decoded whole-image owners before fixed prepared-page admission. It then creates a second complete decoded-pixel owner with `to_vec`. `PendingRasterUploadCursor` transfers that whole owner in one grant and uses a LIFO `Vec::pop` queue.

The current `image` crate PNG/JPEG decoder requires one complete caller-provided output slice. Its public PNG/JPEG decoders do not expose rectangular/page decode. Replacing that semantic codec is not part of this adjacent packet; the single codec-owned decoded backing remains an explicit residual. This packet must remove the second contiguous clone and ensure conversion from the codec owner to prepared ownership is page-retained and one page per grant.

## Infinite World producer path

`apply_reference_image_bytes` decodes into one RGBA `Vec<u8>` and publishes it in the fixed `reference_pixels` registry. During scene construction, the reference-plane path borrows that cache and calls `ensure_world_plane_texture`, which creates a second complete owner using `pixels.to_vec()` before any prepared-page admission.

`apply_reference_image_bytes` has no in-repository production caller, but it is a public renderer entrypoint and its populated registry is consumed by the live World scene path. The prepared clone is therefore reachable and must not remain as a default path.

## Selected implementation boundary

The smallest complete adjacent group selected for this packet is the Canvas/Paint prepared-raster producer and handoff:

- Replace the contiguous `PreparedRenderUpload::Raster` payload with a fixed-capacity page authority whose page size is at most 16 KiB.
- Add exact item/byte/dimension/key preflight before the prepared page authority accepts ownership.
- Make Canvas conversion from its one codec-owned backing resumable, advancing one page per `PendingRasterUploadCursor` grant and moving the backing rather than cloning it.
- Retire cancel/stale/fault owners one key/page/scalar per grant and preserve the last accepted raster generation.

Infinite World's cached-slice clone is a distinct producer with different borrowed-source lifetime and cache ownership. It remains the next adjacent packet rather than being partially adapted here. The non-streaming PNG/JPEG decode backing and the World reference cache's original decoded backing likewise remain visible residuals for a later semantic codec/cache packet. This packet does not claim those backend decode calls are interaction-bounded.

## Required discriminators

Permanent source fixtures for the selected group must reject a reintroduced Canvas slice `to_vec`, dynamic page growth, ordinary backing drop before credit release, and generation-free handoff. Semantic fixtures must cover exact page/item/byte cap and cap+1, page pointer identity, stale-generation/ABA rejection, mid-producer cancellation, one page per grant, exact terminal retirement, and by-value completed handoff. The census retains the World `to_vec` witness as an explicit residual rather than claiming it was remediated.
