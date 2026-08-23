# P3k World Reference Raster Producer Caller Census — 2026-08-23

## Decision

The next source packet after the accepted Canvas/Paint prepared-raster producer is the Infinite
World reference-image producer. It is a narrow ownership handoff packet, not a claim that the
complete World render/rebuild path or the PNG/JPEG semantic codec is bounded.

## Live Boundary

`World3dState::reference_pixels` retains decoded `(width, height, Vec<u8>)` owners in the fixed
`WorldDynamicRegistry`. Scene building borrows one of those owners and calls
`World3dBuildContext::ensure_world_plane_texture`. That method currently performs
`pixels.to_vec()` and pushes a second whole decoded-image allocation into
`PreparedRenderUpload::Raster` before any prepared-page grant.

The accepted downstream Raster authority writes at most one fixed page per GPU grant. The live
defect is therefore the borrowed-cache-to-prepared-owner transition: a large cached image is cloned
in one worker opportunity, and the build context stores uploads and request keys in dynamic
`Vec`/`HashSet` owners.

`apply_reference_image_bytes` remains a public producer of the first decoded cache owner. Its
PNG/JPEG codec is not page-resumable, so the codec-owned backing remains an explicit adjacent
semantic-codec residual. This packet must not hide that residual or introduce a second copy.

## Required Implementation Contract

1. Replace `ensure_world_plane_texture(&[u8])` with a retained reference-image upload lease tied to
   an exact cache token/epoch. Never copy the complete cached slice.
2. Admit the exact key, dimensions, page count, logical bytes, and final-page length before the
   producer can be retained. Multiplication overflow and cap+1 must return the exact owner/token.
3. Advance at most one fixed page or one fixed metadata unit per granted call. Check cancellation,
   deadline, source epoch, scene revision, and upload generation before each page handoff.
4. Keep the cache entry pinned for the lease lifetime. Replacement, eviction, scene supersession,
   and document/window close must either wait for the lease or detach one page/item per close grant;
   no borrowed pointer may outlive the exact cache generation.
5. Replace the build context's dynamic raster-request and upload queues at this path with fixed
   admitted authorities. Saturation must retain and return the exact candidate rather than silently
   dropping, overwriting, or allocating.
6. Publish a by-value fixed-page authority to `PreparedRenderUpload::Raster`; the accepted
   downstream generation checks and row/page upload semantics remain unchanged.
7. Remove the World `pixels.to_vec()` witness. Do not rewrite the codec, atlas, glyph, Vello,
   surface-map, or full World draw-rebuild paths in this packet.

## Required Discriminators

- maximum and maximum-plus-one key, width/height, bytes, pages, queue items, and cache leases;
- multiplication overflow and a final partial page;
- source pointer/page identity proving no complete clone;
- low nonzero fuel and near/expired deadline before the first and middle pages;
- cache replacement/eviction during an active lease;
- stale scene revision, generation replay, ABA token reuse, cancellation, and close mid-transfer;
- queue saturation, panic/fault, receiver abandonment, and exact rejected-owner handback;
- one-page-per-grant and bounded incremental retirement;
- last accepted Raster generation remains present after any rejected or cancelled replacement.

The permanent verifier must reject a reintroduced `pixels.to_vec()`, dynamic World raster upload
queues on this route, an unpinned borrowed slice, missing epoch/revision/generation checks,
post-allocation admission, whole-owner terminal drop, and a page loop inside one grant.

## Residuals Preserved After This Packet

- the first whole PNG/JPEG codec-owned decoded backing;
- glyph and icon atlas cloning/full uploads;
- dynamic EngineCanvas surface authorities;
- Vello/build/submit/present timing and full `GpuContext` realm retirement;
- the broader World state JSON decode and draw-rebuild cursors;
- native/Wasm/browser stress and timing acceptance.
