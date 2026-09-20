# Reference Image Decode Ownership

## Result

The measured browser path no longer runs Rust `image::decode` and Triangle resize while the frame transaction owns the runtime mutex. The frame Worker now retains encoded pages only for a reference-image request, suspends through browser-owned `createImageBitmap` decode and resize, and stages validated straight RGBA against the exact Rust request token before sealing the response. The ordinary retained probe later consumes only a matching token and URL.

The browser path and native path now both decode outside the frame/runtime lock: the browser Worker uses `createImageBitmap`, and native transfers one exact token+URL payload to the process WorkerPool Maintenance lane before publishing bounded RGBA. Checkpoint 14 sampled no old `image::load_from_memory`/resize work on the frame Worker and measured only 2.276 ms in staged-reference handoff, versus the checkpoint 13 decode interval of about 2.444 seconds. That proves the measured synchronous decode stall left the frame path in the sealed checkpoint 14 artifact. It does not accept the packet's later SVG/orientation/failure-handling source, the remaining full-resolution gap, or the overall frame budget.

## Owned contract

The neutral schema and fixture own:

- React's authoritative reference-media *routing* set: png/jpg/jpeg/gif/webp/bmp/avif/tif/tiff/svg/pdf; this does not claim every host browser can decode every routed image codec;
- an explicit capability split: browser metadata currently covers PNG/JPEG/GIF/WebP/BMP/SVG, native decode covers PNG/JPEG/GIF/WebP, and AVIF/TIFF/PDF remain open;
- an 8 MiB decoded RGBA item limit;
- four cache items and a 32 MiB decoded cache byte limit;
- browser-default color conversion into sRGB RGBA8;
- straight alpha, source orientation, and medium linear resize;
- the 2275×2560 representative plan's 1365×1536 bounded result;
- same URL plus same SHA-256 reuse;
- same URL plus changed encoded bytes replacement;
- stale generation cancellation;
- a bounded 4096-sample browser oracle with mean and maximum channel-error limits.

The cache key includes the encoded source digest. URL identity alone cannot return pixels from changed bytes. Inserting changed bytes removes the former entry for that URL. Cache ownership is the Worker module lifetime, contains no request or surface references, and stays within both item and byte credits.

The request lifetime remains exact:

1. Rust publishes `referenceImage` on the retained fetch descriptor.
2. The Worker streams the response into the existing fixed 16 MiB owner and keeps a bounded reference-only copy.
3. It checks the exact request before and after digest/decode suspension.
4. It stages decoded pixels against the active request token.
5. Abort discards staged pixels before returning the request owner.
6. The retained probe consumes only the same token and URL; a retired surface cannot publish.

Runtime-lock contention is not classified as cancellation. The browser-current query treats a busy runtime as retryable while the final Rust publication still validates the live surface token and current URL.

## Independent oracles

`RasterDecodeParity` in the existing Infinite ReferenceMedia Storybook story loads the representative JPG through React's real `referenceMediaPort` / THREE `TextureLoader`, draws it at the bounded output size, and compares representative RGBA samples with the browser decoder. It publishes dimensions, mean channel delta, and maximum channel delta in `data-result`.

The Storybook oracle was added but not executed in this packet because root owns the shared browser/build queue. The prior Storybook build completed its bundles but failed its independent post-build inventory guard; it did not contain this new story.

The ticket-local isolated Chromium oracle bundles Three with the production decoder and exercises the actual Worker decode and page-fallback boundaries without rebuilding Storybook. PNG, GIF, WebP, BMP, and SVG matched Three's dimensions and sampled pixels exactly. Normal JPEG and EXIF orientations 6 and 8 matched sampled pixels within the declared tolerance (normal and EXIF-6 mean/max 0/0; EXIF-8 mean 0.000305 and max 1), and both Worker and page cancellation retired successfully. Those three JPEG rows remain intentionally red on dimensions: the production decoder returns the current bounded 1365×1536 raster while Three retains the natural 2275×2560 or oriented 2560×2275 raster. The receipt therefore reports `passed: false`; it is evidence of the remaining resolution defect rather than a failed harness.

Canonical queued commands:

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/ui-react:build --skip-nx-cache
```

The isolated oracle command is:

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts' nx exec '--projects=workspace' -- bun '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️reference-decode/📜️script.ts' browser-codec-oracle
```

## Verification

- Neutral schema/cache/orientation/codec-metadata law: 5 passed, 0 failed, 23 assertions.
- Strict standalone TypeScript check for the decoder module: passed with repository library diagnostics skipped.
- Focused browser-frame transport suite: 40 passed, 0 failed, including refusal-to-wake ordering, one page-decode credit, and exact cancellation.
- Isolated Chromium codec oracle: PNG/GIF/WebP/BMP/SVG exact; JPEG/EXIF sampled pixels within tolerance; Worker and page cancellation passed; overall intentionally red on the current full-resolution dimension gap.
- Rust parser checks: World, renderer, browser Worker, and the repaired navbar fixture all passed.
- Native25, the next activation, the Storybook parity result, and the full-resolution ownership design remain root/follow-up gates.

## Navbar receipt

Native22's only red was fixture drift in the new advancing-origin law. `world_pane_shell().sync_dock_tabs()` supplied one generic hit, while the test appended one owned TopLeft tab and asserted three owned TopLeft tabs. The fixture now adds two explicit TopLeft rows before the resumed multi-glyph row. Production navbar geometry was unchanged.

## Paint-contract boundary

Checkpoint13 shows React's plan muted/translucent while WGPU is hard black/gray. Decode correctness cannot accept that scene appearance. This packet owns decoded dimensions, orientation, color conversion, alpha bytes, resampling, reuse, and retirement. Reference material opacity/tint and shader color-space consumption remain an explicit paint-contract gap requiring a separate scene material packet and paired runtime acceptance.

## Files

- `framework/products/os/modules/renderer/engine/schema/reference-image-decode/schema.json`
- `framework/products/os/modules/renderer/engine/fixtures/reference-image-decode/fixture.json`
- `framework/products/os/modules/renderer/engine/targets/wgpu/reference-image-decode.ts`
- `framework/products/os/modules/renderer/engine/targets/wgpu/frame-worker.ts`
- `framework/products/os/modules/renderer/engine/targets/wgpu/browser-worker.rs`
- `framework/products/os/modules/renderer/engine/targets/wgpu/renderer.rs`
- `framework/products/os/modules/infinite/world.rs`
- `framework/products/os/modules/infinite/stories/reference-media.story.tsx`

## P1 Audit Remediation

The audit addendum's three negative paths are now closed in source.

### Source allocation

The browser owner parses PNG IHDR or JPEG SOF dimensions before `createImageBitmap`. A source over
16,777,216 pixels (64 MiB straight RGBA) is rejected before browser decode allocation. The first
`createImageBitmap` call receives the final bounded resize dimensions, so it does not first request
an unbounded full-size bitmap and then create a second bitmap.

JPEG EXIF orientation is parsed from the TIFF orientation tag. Orientations 5–8 swap header width and
height before target-size selection; `imageOrientation: "from-image"` therefore receives dimensions
matching the browser-natural image rather than distorting 90°/270° sources. The neutral fixture owns
orientation and includes a quarter-turn vector.

React owns the routing contract in `framework/ui/react/🟦️.tsx`: `referenceMediaKindFromUrl` routes
png/jpg/jpeg/gif/webp/bmp/avif/tif/tiff to THREE `TextureLoader`, SVG to a browser `Image` raster,
and PDF to pdf.js. No pre-existing schema narrows World references to PNG/JPEG. Routing is distinct
from a successful decode: PNG/JPEG/GIF/WebP/BMP are ordinary Chromium image codecs, AVIF support is
browser/version dependent, and TIFF may be refused by the browser that React delegates to. PDF is
not delegated: React explicitly rasterizes it through pdf.js. The neutral fixture names these fields
`reactRoutedExtensions`/`reactRoutedMediaTypes` so it does not misstate route admission as a render
verdict.

Browser metadata is bounded before allocation for PNG, JPEG/EXIF, GIF, all three WebP header forms,
BMP, and the SVG root tag (the metadata scan is capped at 64 KiB). The Blob receives the sniffed MIME
type before `createImageBitmap`, so browser-owned GIF/WebP/SVG decoders remain reachable. The shared
Rust probe admits the complete React signatures without changing the narrower UI-image/map-tile
lanes. AVIF/TIFF metadata and PDF page rasterization remain concrete browser gaps. Native's `image`
feature set decodes PNG/JPEG/GIF/WebP; native SVG/BMP/AVIF/TIFF/PDF remain concrete gaps. These are
reported as residual parity work, not rejected from the feature contract.

### Failure and cancellation

Malformed, unsupported, over-limit, and browser-decode failures use `rejectAssetResponse`: the exact
World request records one asset miss, closes, and returns without quarantining the Worker. Busy runtime
ownership is a bounded retry, with abort as the terminal fallback.

Currentness is checked before and after every streamed page suspension, digest, bitmap decode, staging
retry, and seal retry. Retirement aborts the fetch controller when relevant and never publishes stale
pixels.

### Staging

Rust staging is byte-accounted at four items and 32 MiB. A fifth live token receives backpressure;
no live staged token is evicted. The Worker wakes the renderer and retries admission. The wasm probe
has no synchronous Rust decode fallback if a staged token is absent: it records the individual miss
and retires the response.

### Native worker

Native Ready probes transfer their bounded encoded payload into one exact decode job on the process
WorkerPool Maintenance lane. Decode and resize run without the runtime/frame lock. Completion wakes
the host, revalidates the exact token and current URL, then performs only bounded decoded publication
on the frame path.

Admission contention retains the original bytes and retries. Cancellation fences work before and
after the third-party decode; an unsubmitted job terminalizes immediately, while close waits for a
scheduled job's terminal handback. The underlying `image` decode is a single bounded third-party
call and is not interruptible mid-call, so cancellation during that call suppresses publication at
its result gate rather than preempting the library.

Added focused laws:

- 4096×4096 accepted and 4097×4097 rejected before browser decode;
- malformed header rejected;
- EXIF orientation 6 swaps dimensions;
- fifth staged live token refused without evicting token one;
- unsubmitted native decode cancellation reaches terminal with its input retired.

The browser oracle now injects EXIF orientations 6 and 8 into the actual asymmetric plan JPEG at
runtime, loads each Blob URL through THREE `TextureLoader`, and compares dimensions and sampled
pixels against the WGPU browser decoder. It also executes the 4097² source rejection and retires a
decode through the result-gated cancellation callback. This source is ready for the next root-owned
Storybook/browser run; no fresh browser verdict or post-change CPU profile is claimed.

## Independent Audit Addendum — 2026-09-20

This read-only audit found that the normal browser path does perform `createImageBitmap`, resize, and
`OffscreenCanvas` readback in `🎞️frame-worker/🟦️.ts`; it is therefore off the page/UI isolate on a
successful current request. The source and the current two-test cache law do not establish the claimed
failure, cancellation, or all-load off-frame properties.

### P1 — Source-pixel ceiling is applied after full browser decode

`decodeReferenceImage` calls `createImageBitmap(source)` before it reads dimensions or computes the
8 MiB target (`🎯️targets/🧊️wgpu/🖼️reference-image-decode/🟦️.ts:20-30`). The 16 MiB encoded-response
credit is not a decoded-pixel credit. A small compressed PNG/JPEG with dimensions beyond the old
64 MiB source-RGBA ceiling is fully materialized by the Worker before it is resized.

Minimal vector: a 4097×4097 PNG or JPEG (67,141,636 RGBA bytes). The existing native law rejects
that exact vector before decode at `🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:535-550`, but the
browser branch decodes then scales it to the 8 MiB target. Larger compressed image bombs can consume
far more worker memory and create cross-target acceptance drift. A `createImageBitmap` rejection
then reaches `pumpAsset`'s non-cancellation catch and faults the entire Worker
(`🎞️frame-worker/🟦️.ts:636-666`) instead of recording one missing reference as the native probe law
does.

The fix boundary is to validate encoded image dimensions against a declared source-pixel limit before
browser bitmap construction, and to map malformed/unsupported/over-limit reference images to the
existing per-asset miss/retirement path. The output RGBA equality checks at
`🌐️browser-worker/🦀️.rs:152-160` and `🧊️renderer/🦀️.rs:11088-11100` are correct but occur too late
to protect decoder allocation.

### P1 — Four staged images can restore synchronous frame decode

The Worker cache is bounded to four 8 MiB entries (32 MiB), but the Rust staging queue separately
caps itself at four *items* and silently evicts the oldest (`🧊️renderer/🦀️.rs:11075-11085`). When a
later retained probe finds its token+URL absent, it falls back to `apply_reference_image_bytes`
(`🧊️renderer/🦀️.rs:10851-10893`), which performs the synchronous Rust decode/resize.

Minimal vector: load five distinct reference images whose retained page probes have not completed
before later fetches stage their decoded results. The fifth `stageReferenceImage` evicts the first
decoded RGBA result. When the first probe reaches `Ready`, `take_staged_reference_image` returns
`None` and lines 10873-10892 decode its retained encoded bytes on the frame path. This contradicts
the unconditional off-frame claim and can reintroduce the frame-stall failure the browser owner was
intended to remove. The staging queue needs a byte-accounted admission/retirement protocol that
cannot evict a sealed live token; the browser path should wait or reject the individual asset, never
fall back to synchronous frame decode.

### P2 — Cancellation reaches the result gate but does not cancel work promptly

The request is checked before digest and again before staging (`🎞️frame-worker/🟦️.ts:637-645`), and
Rust stages by the exact `WorldAssetRequestToken` then matches token plus URL on consumption
(`🧊️renderer/🦀️.rs:11088-11115`). `abortAssetResponse` also discards a staged token
(`🌐️browser-worker/🦀️.rs:236-255`). Those gates prevent a settled stale image from publishing.

They do not connect document/reference retirement to `assetAbort`: the controller is only aborted on
Worker close (`🎞️frame-worker/🟦️.ts:417-430`). A replacement during streaming, SHA-256, or bitmap
decode continues to read and allocate until the next currentness check. If seal backs off over a
macrotask and retirement happens before its retry, the Rust seal rejects; that non-`AbortError`
travels to the same whole-Worker fault path at lines 652-666. Add an exact-current check before each
page/decode/seal retry and classify a retired owner as normal per-asset cancellation.

### Coverage Gap

The current TypeScript test exercises only schema target sizing and direct cache `get`/`put`
(`🧪️tests/🖼️reference-image-decode/🟦️.ts:9-24`). It does not execute `createImageBitmap`, source
dimension rejection, a decode failure, retirement during a suspension, seal retry after retirement,
or five live staged tokens. No browser or build command was run in this audit; root owns those queues.

## Independent Handoff Audit — 2026-09-20

Read-only review of the current browser/native implementation. No commands were run. The Native23
suite's reported 1,183 passing tests are implementation-owner evidence, not a result of this audit.

### Earlier P1s Closed

- Browser source dimensions are read before `createImageBitmap` in
  `🎯️targets/🧊️wgpu/🖼️reference-image-decode/🟦️.ts:58-90`; a 4,097×4,097 PNG or JPEG is refused
  before bitmap allocation. The one bitmap call receives the final bounded resize dimensions
  (`:105-127`).
- `🎞️frame-worker/🟦️.ts:647-697` checks currentness around digest, decode, staging, and seal. A
  malformed, unsupported, over-limit, or bitmap-decode reference image takes the per-asset reject
  path rather than `asset-stream-fault`.
- `StagedReferenceImageAuthority::try_stage` in `🧊️renderer/🦀️.rs:10375-10385` refuses a fifth
  live token without removing an earlier one. Browser staging returns backpressure at
  `:11220-11230`; the probe no longer falls back to synchronous frame decode when a staged image is
  absent.
- Native uses one exact `NativeReferenceDecodeJob`, submitted only to the process WorkerPool
  Maintenance lane (`🧊️renderer/🦀️.rs:10405-10427`). Submission contention retains the input and
  restores phase zero. Cancellation atomically terminalizes an unsubmitted job, and close cancels a
  submitted job then waits for phase two before releasing its probe (`:11137-11169`). The third-party
  decode remains non-preemptible while running, but its result is fenced before publication.

### P1 — Native EXIF Orientation Is Not Applied

The browser honors the fixture's `"orientation": "from-image"` contract: it parses JPEG EXIF,
swaps dimensions for orientations 5–8, and calls `createImageBitmap` with
`imageOrientation: "from-image"` (`🖼️reference-image-decode/🟦️.ts:58-90,105-115`). The native
WorkerPool job calls `decode_reference_image_bytes`, whose implementation is only
`ImageReader::decode()` followed by resize and `to_rgba8()`
(`♾️infinite/🌍️world/🦀️.rs:15352-15355`). It neither reads decoder orientation nor calls
`DynamicImage::apply_orientation`.

This is not automatic in the locked `image` 0.25.10 dependency: its own API documents obtaining a
decoder orientation and then explicitly applying it. A legitimate asymmetric 3×2 JPEG with EXIF
orientation 6 therefore produces browser 2×3 oriented pixels but native 3×2 unrotated pixels. The
neutral fixture already names this exact expected geometry (`🧫️fixtures/🖼️reference-image-decode/🔣️json`,
`EXIF quarter turn`), while only the browser source test exercises it.

Apply the decoder orientation before `bounded_reference_image` in the native WorkerPool decode path,
then add an asymmetric EXIF-6 byte-vector law that asserts dimensions and corner-pixel placement for
both native decode and the browser oracle.

### P1 — Rejected Browser Reference Does Not Wake Follow-on Work

`pumpAsset` catches a reference failure, performs `rejectAssetResponse` or `abortAssetResponse`, and
then only clears `assetPumping` (`🎞️frame-worker/🟦️.ts:683-701`). The explicit wake is sent only on a
successful seal (`:675-682`). The only scheduling sites are boot and an incoming successful frame
message (`:363,578,594-598`). A handled reference failure therefore has no local wake that would
render the miss or poll the next queued asset.

Minimal reproduction: queue two reference requests, make the first malformed or unsupported, and let
the host go idle after its last frame. The first request records its per-asset miss and closes without
faulting the Worker, but the second request is not polled until unrelated input, resize, or some other
external frame arrives. A one-line wake after terminal reference rejection is the bounded remedy; it
returns control to the normal frame path, which schedules the next asset pump. Add an integration
law that forces a reference decode failure, asserts one wake, and proves that a following valid
request is polled and published without external input.

## Format and Handoff Correction — 2026-09-20

The audit correction is implemented at the producer handoff rather than as a UI-only exception.

- `ReferenceImageDimensions` now carries the sniffed media type. Frame Worker Blob creation uses that
  type, and source metadata is read before browser allocation for PNG, JPEG, GIF, WebP, BMP, and SVG.
- The renderer's ReferenceImage format probe admits React's image/SVG/PDF signatures. PNG/JPEG keep
  their full structural scanners; SVG keeps bounded UTF-8/root validation; the additional signatures
  use the sealed byte-count cursor before their target decoder. This is prefix admission only, not
  evidence that the downstream browser/native codec rendered the bytes. UI images and raster map tiles retain
  their existing PNG/JPEG/SVG scope.
- Native decode now asks the `image` decoder for orientation, applies it before resize/publication,
  and checks the 16,777,216-source-pixel ceiling before full decode. The asymmetric orientation-1
  versus EXIF-6 law asserts both 3×2→2×3 geometry and the exact clockwise corner mapping. That law is
  source-ready and awaits the next root-owned native run.
- A handled browser reference refusal now posts exactly one Worker wake after the exact response is
  rejected or aborted. This exposes the miss and lets the normal frame owner poll a following asset
  without unrelated input. A browser Worker source law owns the ordering; the next activation is the
  runtime gate.
- The two earlier staging/cancellation laws now close the `WorldAssetIoAuthority` claims they used
  only to mint exact tokens. Native24's Drop failures were fixture owner leaks, not production
  staging leaks; the laws retain their capacity/cancellation assertions and now prove terminal
  handback.
- Worker bitmap decode owns PNG, JPEG, GIF, WebP, and BMP. SVG uses a bounded page-image fallback
  because the tested Chromium Worker cannot decode the valid SVG Blob with `createImageBitmap`, while
  the page `Image` path used by React can. The frame protocol permits one live page decode, transfers
  the encoded `ArrayBuffer` to the page and the resulting `ImageBitmap` back to the Worker, performs
  RGBA readback only in the Worker, and closes stale bitmaps. Generation change, exact request
  cancellation, and Worker close abort that one request. Invalid, concurrent, and over-credit work
  receives a named per-asset refusal rather than faulting the Worker.

The isolated browser oracle bundles Three plus the production decoder without a Storybook rebuild.
It compares valid PNG, GIF, WebP, BMP, SVG, normal JPEG, EXIF-6 JPEG, and EXIF-8 JPEG dimensions and sampled
sRGB pixels, and executes both Worker and page-decode cancellation. It was run with:

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts' nx exec '--projects=workspace' -- bun '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️reference-decode/📜️script.ts' browser-codec-oracle
```

The command writes its reviewable receipt to
`🗑️generated/astra-reference-decode/browser-codec-oracle/result.json`. Every codec row persists its
decoder, Three, and oracle result even when an earlier row fails. The latest receipt is green for
PNG/GIF/WebP/BMP/SVG and cancellation and red only for the current bounded JPEG/EXIF dimensions.

## Checkpoint 14 Visual Inspection

The fresh paired `05-settings-general.png` images prove that the same reference plan is resident in
both WGPU panes; this is not a missing-decode state. They also confirm the paint-contract gap remains:
React renders the plan muted and translucent into the cream floor/grid, while WGPU paints a distinct
gray rectangular plane with near-black wall strokes and much stronger contrast. Camera framing and
the settings overlay differ, so these screenshots are qualitative material evidence rather than a
pixel decoder oracle. The sealed checkpoint 14 artifacts also predate this format/orientation/wake
batch. Decode parity therefore cannot be inferred from this screenshot, and opacity/tint/shader
color-space acceptance remains separate work.

Reviewed artifacts:

- `🗑️generated/astra-runtime/paired-checkpoint-14/react/05-settings-general.png`
- `🗑️generated/astra-runtime/paired-checkpoint-14/wgpu/05-settings-general.png`
- `🗑️generated/astra-runtime/paired-checkpoint-14/{react,wgpu}/02-dismiss-tour.png`

## Checkpoint 14 Performance Receipt

Root's sealed checkpoint 14 CDP profile keeps all five generated artifact hashes unchanged. The old
`image::load_from_memory`/resize pipeline is absent from sampled frame-Worker work. Staged reference
handoff accounts for 2.276 ms inclusive, and the maximum Worker tick is 100.1 ms, compared with the
checkpoint 13 reference decode interval of about 2,444 ms. This is strong attribution that the
measured synchronous decode stall left the frame path.

The profile also identifies the next ownership costs: `close_one` accumulated 1,043.5 ms of self
time and `RasterContentIdentity::mix_bytes` accumulated 444.5 ms across the run. These are aggregate
profile costs rather than a single-frame budget. The profile predates the later SVG page fallback,
native EXIF, and terminal-handback fixes, and it still renders downsampled references. It therefore
does not accept full visual parity or final frame latency.

## Full-Quality Raster Ownership Design for Audit

This section is a proposed contract for Terra review. It is not implemented in this packet.

### Current limiting path

- `PREPARED_RASTER_ITEM_BYTES` is 16 MiB, `PREPARED_RASTER_PRODUCER_BYTES` and
  `PreparedRenderLimits.max_upload_bytes` are 32 MiB, and World restricts one reference texture to
  half of the item credit: 8 MiB.
- The representative 2275×2560 plan requires 23,296,000 RGBA bytes, so the current decoder reduces it
  to 1365×1536. This is the isolated oracle's only remaining failure for normal and EXIF JPEG rows.
- `reference_underlay_upload` clones the complete pixel vector for each visible surface/render.
  `ensure_world_plane_texture` transfers that clone into `PreparedRasterProducer`, whose preparation
  cursor mixes one 16 KiB page per step before it can move the backing allocation.
- GPU `ensure_raster_step` then writes one row-aligned page of at most 16 KiB per step. At width 2275,
  a row occupies about 9.1 KiB, so a natural-size plan requires roughly 2,560 queue writes. The GPU
  path is progressive, but the source clone, content-identity prepass, and page granularity make the
  natural-resolution path too expensive.

### Proposed schema

- Admit at most 64 MiB of decoded RGBA per raster, which exactly covers a 4096×4096 RGBA8 image.
- Own a fixed decoded pool with four slots and a proposed 256 MiB process credit. Terra must validate
  these process and per-step budgets before implementation.
- Make upload chunk bytes, one-live-upload-per-surface credit, decode/upload progress, terminal
  refusal, and cancellation vectors schema-owned.
- Key source reuse by encoded content digest plus a decode-contract version. A URL is metadata; the
  same URL with changed bytes must allocate a new identity and retire the prior unleased resident.
- Preserve dimensions, color conversion, straight alpha, orientation, and resampling in the identity
  contract. Reference opacity/tint remains material state rather than decoded-pixel identity.

### Shared decoded-raster authority

Use one target-neutral result and lease boundary for reference images and mesh paint:

```text
DecodedSceneRaster { identity, width, height, pixels }
SceneRasterLease { slot, epoch, identity }
```

The decode request also owns the exact request token, surface or mesh owner, generation, source MIME,
and content digest. Browser Worker/page decoding and native WorkerPool decoding publish through the
same pool contract. Settings can preserve `mesh_paint_textures` as the generation-owned resident
endpoint while replacing its bespoke `DecodedMeshPaintImage` value with an exact pool lease.

Pool slots move through `building`, `ready`, and `retiring`. A matching digest shares one resident
byte owner across panes and, when the decode contract matches, reference and paint consumers. Lease
counts are explicit scalars. Eviction selects only an unleased least-recently-used slot; if all slots
are live, admission refuses with retryable backpressure. Retirement releases at most one configured
chunk per maintenance step so the last 64 MiB owner cannot deallocate on a frame transaction.

### Decode and publication staging

The browser must replace the single full-vector `stageReferenceImage(width,height,pixels)` call with
`begin`, bounded `push-page`, and `seal` operations. Each page checks the exact owner and generation,
reports monotonic progress, and becomes inert after cancellation. Native WorkerPool completion uses
the same cursor instead of handing one full `Vec<u8>` back to the frame owner. The existing bitmap
fallback keeps one decode credit and transfers `ImageBitmap`; it must stream Worker readback into the
pool cursor rather than materialize another full frame-path copy.

### Prepared and GPU ownership

World state holds a lightweight exact lease instead of a pixel vector. `PreparedRasterProducer`
references that lease and its already-known content identity, eliminating the per-render clone and
the second full content-mixing pass. Preparation and GPU upload read resident pages through the exact
lease, keep a row-aligned bounded cursor, and expose `uploadedBytes` and `totalBytes`.

Logical item/upload admission can then cover a 64 MiB raster without charging a duplicate 2× source
workspace, because the pool owns the bytes once and the prepared ledger owns only metadata, leases,
and GPU logical bytes. Terra should select a larger schema-owned row-aligned upload chunk; retaining
the current 16 KiB page would issue about 2,560 writes for the representative plan. A newer generation
or close aborts its GPU cursor and retires any partially created texture. The committed-texture
acknowledgement lets World stop re-offering the lease.

### Required acceptance

- The representative reference remains 2275×2560 in WGPU and React; EXIF-6/8 use 2560×2275.
- Two panes share one resident content identity, one decode, and no complete source clone.
- The same URL with changed encoded bytes invalidates the prior identity.
- Decode, pool staging, and GPU-upload cancellation each reach terminal state without publishing a
  stale texture or retaining a partial GPU resource.
- A 4096×4096 source is accepted; 4097×4097 is refused before decode allocation.
- Decode and upload progress are monotonic, bounded, and associated with the exact generation.
- A live lease is never evicted; all-live capacity produces an observable refusal/backpressure
  outcome.
- Reference material opacity/tint/color-space receives separate paired visual acceptance.
