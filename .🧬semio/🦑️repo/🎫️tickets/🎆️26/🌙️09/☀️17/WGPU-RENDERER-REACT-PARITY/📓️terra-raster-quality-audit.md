# Terra Raster Quality Audit

## Scope and verdict

Read-only audit of the current asynchronous reference-image path and the proposed 64 MiB decoded-raster design. This review did not run builds, browser or native gates; it relies on the current source and the ticket's existing receipts.

The current browser and native decoders correctly keep third-party decode off the runtime/frame lock, and their request owners are substantially fenced by exact token, generation/revision, URL and retirement checks. The proposed natural-resolution target is blocked by more than the 8 MiB decoder limit. A 2275×2560 RGBA8 plan is 23,296,000 bytes and must cross four coupled ceilings: decode, staged handoff, World publication and prepared/GPU admission. Raising only one ceiling would reintroduce copies, whole-image rehashing and per-row uploads.

The implementation packet must replace the `Vec<u8>` handoff with immutable, page-addressable raster leases. It must not raise `PREPARED_RASTER_ITEM_BYTES` and reuse the existing source-owning producer as the way to support 64 MiB images.

## Current ownership facts

### Browser reference decode

1. The frame Worker serializes one asset pump. It reserves a fixed 16 MiB encoded response, streams pages into the Rust response owner and separately clones every reference page into `referenceChunks` (`🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:647-696`). `concatenateReferenceImageSource` then makes another contiguous encoded allocation (`🖼️reference-image-decode/🟦️.ts:184-194`).
2. Header parsing happens before `createImageBitmap`; PNG, JPEG/EXIF, GIF, WebP, BMP and SVG are recognised and sources over 16,777,216 pixels are rejected (`🖼️reference-image-decode/🟦️.ts:118-181`). Browser decode requests the already-bounded output extent and closes its bitmap after Worker readback (`:196-223`).
3. Currentness is checked around streaming, digest, decode, staging and sealing (`🎞️frame-worker/🟦️.ts:673-739`). A failed current reference is rejected as one asset miss; cancellation aborts the response and stale staged result; either terminal outcome wakes the next asset (`:741-760`).
4. The browser-to-page SVG fallback transfers the encoded `ArrayBuffer` to the page and receives a transferable `ImageBitmap` back. It admits one live decode, bounds encoded input to 16 MiB and decoded dimensions to 64 MiB, aborts the exact request, and closes a bitmap that becomes stale (`🚚️browser-frame-transport/🟦️.ts:827-855,880-882,1003-1005`; `🎞️frame-worker/🟦️.ts:606-639`).
5. The browser cache is module-scoped, stores four decoded `Uint8Array`s and evicts by LRU without consumer leases (`🖼️reference-image-decode/🟦️.ts:262-292`). It is an 8 MiB-per-item cache today, not a process-wide shared raster authority.

### Rust token, staging and native decode

1. `WorldAssetRequestToken` contains slot, epoch, generation and revision. `WorldAssetIoAuthority` validates all of them when reserving, sealing, taking and cancelling the exact owner (`♾️infinite/🌍️world/🦀️.rs:14495-14500,14719-14826,14872-14910`). A cancelled owner closes pages incrementally and the `Drop` contracts require terminal handback (`:14655-14687,14912-14935`).
2. Browser `stageReferenceImage` is still one full `Uint8Array` call. Rust verifies width × height × 4, copies it with `pixels.to_vec()`, and keeps up to four live 8 MiB images/32 MiB (`🧊️renderer/🦀️.rs:11229-11262`). `StagedReferenceImageAuthority` keys the handoff by token plus URL and refuses a fifth live entry rather than evicting it (`:10370-10406`). The source law covers that fifth-token case (`🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:570-588`).
3. A wasm retained probe consumes only its matching staged token and URL. A missing staged result becomes a per-asset miss, with no synchronous Rust decode fallback (`🧊️renderer/🦀️.rs:10982-11018`). This is the correct failure boundary to retain.
4. Native gathers the encoded response once, schedules one maintenance-lane job, fences before and after the non-preemptible `image` decode, and waits for terminal output before close releases the probe (`🧊️renderer/🦀️.rs:10410-10464,11265-11329`). Native now obtains and applies EXIF orientation before bounded resize (`♾️infinite/🌍️world/🦀️.rs:15416-15429`).

### Current World, prepared and GPU ownership

1. Reference and mesh-paint residents are independent World registries of `(width, height, Vec<u8>)` (`♾️infinite/🌍️world/🦀️.rs:1644-1661`). `apply_decoded_reference_image` and `apply_decoded_mesh_paint_image` each reject more than half of the 16 MiB prepared item limit, therefore 8 MiB (`:10772-10791,15386-15447`).
2. Every visible reference clones its entire resident pixel vector per render, then sends it to a source-owning `PreparedRasterProducer` (`:10811-10814,12781-12788`). Painted meshes do the same (`:12586-12629`). Two panes have two World registries and therefore two retained vectors today.
3. The prepared producer has a 16 MiB item ceiling, 32 MiB process producer ceiling and reserves peak source workspace before materialising pages (`🎟️prepared/🦀️.rs:401-409,1018-1081`). Its preparation cursor mixes every pixel byte before moving the backing allocation (`:1170-1206`). The sealed profile's 444.529 ms `RasterContentIdentity::mix_bytes` self time is direct evidence that this work is material at 8 MiB.
4. The prepared transaction also has a 32 MiB upload limit (`🎟️prepared/🦀️.rs:12-24`). GPU upload takes at most 16 KiB per step (`🖍️draw/🦀️.rs:2103-2196`). A 2275-pixel RGBA row is 9,100 bytes, so this is one row per write: 2,560 writes for a 2,560-row natural plan.

## Prioritized findings

### P1 — A cap-only change cannot admit full quality

The current 8 MiB limit appears in browser target sizing (`🖼️reference-image-decode/🟦️.ts:1-19`), browser/Rust staging (`🧊️renderer/🦀️.rs:11229-11250`), World reference and paint publication (`♾️infinite/🌍️world/🦀️.rs:10781-10791,15393-15447`), prepared items/producers (`🎟️prepared/🦀️.rs:406-409`) and transaction upload accounting (`:21-24`). Any isolated increase fails at a later seam. Raising the existing prepared source producer to 64 MiB would additionally charge its retained source and peak workspace, preserving the duplicate-workspace design the proposal intends to remove.

**Required change:** introduce a separate raster-lease producer. It accounts metadata/page cursors and logical GPU bytes, while the decoded pool owns the only CPU RGBA backing allocation. Do not pass a `Vec<u8>` through `World3dState`, `PreparedRasterProducer` or the wasm binding.

### P1 — Whole-image clone, rehash and 16 KiB upload scaling break the proposed budget

`reference_underlay_upload` and the S2b paint branch clone every frame (`♾️infinite/🌍️world/🦀️.rs:10811-10814,12627-12629`). Prepared production then performs a full byte mix and GPU emits one 9,100-byte row at a time for the representative plan (`🎟️prepared/🦀️.rs:1204`; `🖍️draw/🦀️.rs:2183-2196`). This is incompatible with a 64 MiB item even if its admission limits are enlarged.

**Required change:** the pool computes a content identity while writing decode pages. Preparation and GPU upload consume immutable pool pages through an exact lease. Use a schema-owned row-aligned chunk budget materially larger than 16 KiB, with chunk rows derived from `floor(chunkBytes / (width * 4))`; expose exact `uploadedBytes` and `totalBytes`. The texture table must acknowledge committed residency so World stops re-offering a completed lease. On replacement, keep the committed old texture visible until the new upload commits; on cancellation, close the partial upload cursor before releasing its lease.

### P1 — The existing handoff cannot be made chunked by changing only its limit

Browser decode returns a full `Uint8Array` from `getImageData`, then `stageReferenceImage` crosses wasm and Rust copies it (`🖼️reference-image-decode/🟦️.ts:215-223`; `🧊️renderer/🦀️.rs:11229-11250`). Native gives the completion owner a full `DecodedReferenceImage` vector (`🧊️renderer/🦀️.rs:10430-10435`). At 64 MiB these are extra full allocations, outside a nominal four-slot 256 MiB pool.

**Required change:** replace `stageReferenceImage(width,height,pixels)` with exact `beginRaster`, `pushRasterRows` and `sealRaster` operations. Browser Worker readback must call `getImageData` for bounded row strips and copy each strip directly into the reserved pool slot; it must never call the current full-image helper for a full-quality result. Native may move the `image` crate's final RGBA allocation into its pool slot, but must not duplicate it at completion. Model a separate active-decode workspace credit: a full `ImageBitmap` and Worker canvas can each be 64 MiB even when pool pages are bounded. The 256 MiB pool is only valid if that transient decoder workspace is separately admitted and the active decode count is bounded.

### P1 — S2b paint has the endpoint but not a full-quality owner contract

S2b's `apply_decoded_mesh_paint_image` is a useful validated endpoint: it checks mesh UV completeness and input byte geometry. It has no source digest, mesh generation/revision, page cursor or lease, and stores a separate full vector under a mesh key (`♾️infinite/🌍️world/🦀️.rs:10772-10791`). Its render branch clones that vector every frame (`:12586-12629`).

**Required change:** make `mesh_paint_textures` contain `SceneRasterLease` plus the exact mesh key, mesh revision, raster identity and paint decode profile. A seal succeeds only if the current mesh still has matching UV topology and revision. The S2b painted-material draw may consume that lease only after the mesh and texture are resident; the existing `mesh-paint:{meshKey}` GPU key must include/revalidate the raster identity. A reference underlay and paint may share a source result only when the full decode profile matches. S2b explicitly distinguishes the paint map's `NoColorSpace` semantics from the reference plane's sRGB path, so consumer kind alone is not sufficient identity.

### P2 — Same-URL replacement is a cache law, not current React behaviour

The decoder fixture proves `ReferenceImageDecodeCache` replaces an already fetched URL when its new digest is supplied (`🧫️fixtures/🖼️reference-image-decode/🔣️.json:84-105`; `🧪️tests/🖼️reference-image-decode/🟦️.ts:24-33`). World does not re-fetch a URL once `reference_pixels` contains it (`♾️infinite/🌍️world/🦀️.rs:12764-12774`), and the React plane effect reloads only when its URL/media-kind/page dependency changes (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3993-4013`). Thus current React parity does not require polling an unchanged URL.

The new infrastructure may deliberately support a same-URL/new-bytes replacement, but it needs an explicit source revision, advertised digest, or chosen revalidation policy. Keep that policy separate from the full-quality parity gate. Once new bytes are admitted, key identity by encoded digest plus decode-contract version; URL remains display/request metadata.

### P2 — React routing is proven; AVIF, TIFF and PDF WGPU rendering is not

React routes PNG/JPEG/GIF/WebP/BMP/AVIF/TIFF to `THREE.TextureLoader`, SVG through a page `Image` raster, and PDF through `pdfjs-dist` page rendering (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:616-713`). The source establishes the route and PDF implementation. It does not establish that a particular browser accepts AVIF or TIFF through `TextureLoader`.

WGPU's browser metadata/parser accepts only PNG/JPEG/GIF/WebP/BMP/SVG (`🖼️reference-image-decode/🟦️.ts:118-181`). Its retained prefix probe admits TIFF, PDF and `ftyp` containers as opaque candidates (`🧊️renderer/🦀️.rs:2879-2891`), but the decoder cannot assign those inputs a media type or dimensions; they are subsequently refused. The fixture accurately names AVIF, TIFF and PDF as unimplemented WGPU formats (`🧫️fixtures/🖼️reference-image-decode/🔣️.json:30-59`). Native `image` decode lists PNG/JPEG/GIF/WebP only. Do not promote prefix admission to codec support.

**Required change:** preserve this capability matrix in the full-quality schema. Full-quality acceptance applies first to the implemented JPEG/PNG parity path. Add AVIF/TIFF/PDF only with a real decoder/rasterizer, bounded metadata reader, cancellation/failure law, and an executable React comparison for the target browser.

## Schema-first implementation packet

Create a neutral `scene-raster-ownership` schema and fixture before target code. It should own the following values and vectors:

| Area | Required schema fields |
| --- | --- |
| Decoded pool | `decodedItemBytes: 67108864`, `slotCapacity: 4`, `poolBytes: 268435456`, slot epoch, byte occupancy, least-recently-used eviction limited to zero-lease ready slots, retryable all-live backpressure |
| Decode input | `encodedByteCapacity`, `sourcePixelCapacity: 16777216`, MIME/metadata capability, decode profile/version, exact owner token, source digest, orientation, alpha, colour conversion and resampling |
| Decode workspace | `activeWorkerDecodeCapacity`, `activePageDecodeCapacity`, `activeNativeDecodeCapacity`, `readbackChunkBytes`, maximum temporary bitmap/canvas workspace, per-chunk cancellation check |
| Consumers | `referenceUnderlay` and `meshPaint` profiles, mesh key/revision/UV witness for paint, exact surface generation for underlay, material colour-space metadata |
| Upload | `oneLiveUploadPerSurface`, row-aligned `uploadChunkBytes`, progress byte counters, GPU logical-byte credit, partially-created-texture retirement and committed acknowledgement |
| Terminal vectors | source format refusal, all-live pool, cancel during decode, cancel during pool push, cancel during GPU upload, close during page fallback, stale token at seal, changed identity replacement |

Use these target-neutral types:

```text
SceneRasterIdentity {
  sourceDigest, decodeProfileVersion, outputWidth, outputHeight,
  orientation, alphaMode, colourProfile, resampling
}
SceneRasterLease { slot, epoch, identity }
SceneRasterWriter { slot, epoch, ownerToken, consumerGeneration, cursor }
```

The pool is the only CPU owner of decoded pixels. Its state machine is `vacant → building → ready(leases) → retiring → vacant`; a writer is valid only in `building` and exact token/generation match. Ready leases are immutable and lease-counted. A cancellation marks the writer terminal, invalidates later chunks and incrementally retires only its written pages. It never evicts or rewrites a ready live lease. Decoding and GPU upload remain separate cursors: a completed decode does not imply a committed texture.

Bridge changes should be made in this order:

1. Add schema/fixture and language-agnostic ownership laws, then target-neutral pool/lease types.
2. Change browser, page fallback and native jobs to `begin/push/seal` pool writing. Preserve the current exact `WorldAssetRequestToken`/URL fences and terminal return path.
3. Replace World's two pixel-vector registries with leases. Keep S2b's UV/material gate at paint seal, and store its paint decode profile in the identity.
4. Add a lease-backed prepared upload producer that reuses the precomputed identity, performs no `mix_bytes`, has no source clone, and closes only bounded pages per maintenance step.
5. Change texture upload to consume pool rows in schema-owned chunks. Existing `RasterTextureUploadCursor` must receive the exact lease/identity and close its partial GPU resource before releasing it. Emit a committed acknowledgement to World.
6. Change current 8/16/32 MiB limits together only after the lease path exists: decoder output, staging, World endpoint, prepared logical upload and GPU allocation accounting must use the same schema values.

## Required acceptance

- A normal 2275×2560 JPEG remains 2275×2560; EXIF-6/8 remain 2560×2275. The browser comparison uses React's real `referenceMediaPort` and tests dimensions as well as sampled pixels.
- 4096×4096 RGBA8 is accepted; 4097×4097 is rejected before decoder allocation. The existing source ceiling is already 16,777,216 pixels and should remain explicit.
- Two panes with the same content and decode profile have one decoded pool identity, one decode and distinct counted leases; releasing either pane does not evict the other.
- A painted mesh and reference can share only a matching decode profile. A changed mesh revision, missing UVs, or stale paint owner cannot publish a lease.
- There is no whole-image `Vec` clone, `Uint8Array.from`, `pixels.to_vec`, per-render `pixels.clone`, or `RasterContentIdentity::mix_bytes` on a decoded resident path. Instrument source bytes, pool bytes, decode-workspace bytes and GPU logical bytes separately.
- Decode, page fallback, pool-writing and GPU-upload cancellation close their exact owners to terminal state. A stale result never publishes; a partial GPU texture never survives its cancelled cursor.
- A changed-byte replacement is tested only under the explicit new revision/revalidation policy; unchanged URL behaviour remains independently documented as current React behaviour.
- Capability receipts keep AVIF/TIFF/PDF as unimplemented until each has an executable target-specific decoder/rasterizer parity result.

## Evidence and limits

Existing source laws cover pre-decode PNG/JPEG pixel rejection, exact four-entry staging backpressure, native unsubmitted-job cancellation, one page-decode credit and current SVG cancellation (`🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:527-603`; `🧪️tests/📨️browser-frame-transport/🟦️.ts:419-445`). They do not yet prove 64 MiB chunk ownership, two-pane pool sharing, retained lease acknowledgement, full-quality S2b paint, or GPU upload cancellation. No gate was rerun for this audit.


## Astra Host Reachability Qualification

The actual renderer World3dHost currently maps all reference records to source.mediaKind=image. It does not forward page/mediaKind/pose fields from the lower-level R3F API. Its ordinary TextureLoader result does not set colorSpace, so image references retain NoColorSpace; only explicit SVG/PDF raster canvases set SRGBColorSpace. Full-quality lease identity must preserve this distinction. Lower-level PDF rasterizer availability alone is not proof that the renderer host exposes a PDF reference lane. See `📓️astra-reference-visual-packet.md` for the separate consumer/material/id/opacity packet.
