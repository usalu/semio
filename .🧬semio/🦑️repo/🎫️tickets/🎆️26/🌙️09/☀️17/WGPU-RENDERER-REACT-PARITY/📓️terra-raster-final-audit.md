# Raster Final Integration Audit

## Scope and method

Read-only source audit of the final natural-resolution reference/paint route:

`browser decode → SceneRasterPool → World pending lease → PreparedRenderUpload::SceneRaster → RasterTextureTable → committed GPU witness → recovery`.

I read `📓️astra-sol-raster-quality.md` and `📓️astra-raster-owner-review.md`, then traced the current Rust and TypeScript producers. I did not run tests, builds, activation, or any git operation. This report records the source snapshot after Root removed raster owners from the World opaque quarantine; World24 was still running when the audit ended.

## Finding: P1 — browser row strips still begin after two full RGBA browser allocations

The browser producer does obey `begin=0 busy / 1 writer / 2 reuse`: the Worker calls `beginReferenceImage` before decode, decodes only for mode `1`, then computes row-aligned `<= 1 MiB` strip boundaries. The claimed bounded readback property is nevertheless false.

`referenceImagePixelsFromBitmap` calls `getImageData(0, 0, width, height)` for the entire image and then makes another full allocation with `Uint8Array.from(...)`. The Worker subsequently takes `decoded.pixels.subarray(...)` views from that complete second buffer and sends them one strip at a time. At the 64 MiB admission ceiling this retains a full `ImageData` backing plus a second full RGBA typed array before the pool has received the first strip.

Locations:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🖼️reference-image-decode/🟦️.ts:215-222`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:712-735`

This violates the ticket’s “bounded `getImageData` strips; never materialize a complete RGBA `Uint8Array` at the wasm boundary” ownership law. It also makes cancellation late: it is checked before streaming, but only after complete readback and duplication have completed.

Replace the `DecodedReferenceImage { pixels }` producer handoff with a strip iterator/callback. Use a strip-height canvas/readback of at most `floor(1 MiB / (width * 4))` rows, draw the matching bitmap region, pass that strip immediately to `pushReferenceImageRows`, check current generation between strips, then yield. The fallback page-decoded `ImageBitmap` must use the same path.

The existing transport source-law actively preserves the problematic shape: it asserts `decoded.pixels.subarray(offset, end)` rather than inspecting the decoder/readback implementation. Extend it, or add a browser integration receipt, to require every `getImageData` rectangle and transferred typed-array view to be row aligned and `<= 1 MiB`; test normal decode, page fallback, cancellation after a non-final strip, `begin=0`, and `begin=2` with a decode-call counter of zero.

## Finding: P2 — a repeated begin for an active writer is advertised as a new writer

`StagedReferenceImageAuthority::begin` returns `WRITER` when it finds the same token/url/descriptor in `Writing` state. A caller that retries `beginReferenceImage` while its first decode is still active is therefore told to decode again; its row zero push fails the exact cursor check. The Worker currently calls begin once per request, so this is not observed on its normal route, but the exported ABI does not uphold “writer” as exclusive ownership under re-entry.

Location:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:10390-10422`

Return `BUSY` for a matching `Writing` entry unless the ABI grows an explicit resume cursor. Keep `REUSED` only for the ready lease. Add a direct ABI/state-machine test: first begin returns `1`; repeated begin before seal returns `0` and performs no decode/readback; after exact seal it returns `2` and performs neither decode nor streaming.

## Confirmed properties

- The Rust pool is the sole CPU pixel owner after admission. `RasterUploadPixels::Scene` reads it through a bounded immutable lease row view and takes its precomputed identity, so the frame upload does not rehash or clone full pixels. See `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:1636-1709`.
- Native moved decode prepares its content identity before the final pool lock/revalidation. Browser rows use exact cursor validation. The World mesh-paint publication verifies current mesh revision, UV revision, and one UV per vertex before it stores a lease. See `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖼️raster-ownership/🦀️.rs:515-624` and `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:10917-10927`.
- `RasterTextureTable` retains the old texture through candidate presentation, commits the release witness only while moving staged texture to live, and drops the GPU witness during bounded retirement. See `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:2149-2305` and `:2424-2490`.
- The current World source fixes the replacement-liveness leak found in the World23 receipt: raster entries now drop directly on replacement/rejection/removal/close while the `Vec<u8>` remains in the pool. The opaque quarantine now carries draw owners only. See `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:1511`, `:10862-10876`, `:10946-10964`, and `:2323-2336`. Its needed regression receipt is `scene-input-residency`'s `more_than_256_distinct_reference_replacements_keep_one_resident_entry` at `🧪️tests/🧲️scene-input-residency/🦀️.rs:102-114`; it had failed at replacement 64 before this repair and was not rerun by this audit.
- Within the supported runtime topology, the global World pool and GPU witness key are sufficient. `RasterTextureTable` is constructed only inside private `GpuContext::from_surface`; native owns one `GpuContext` in `AppPresenter`, and browser owns one optional `GpuContext` in `BrowserRendererBootstrap`. Native and browser are separate processes. This audit found no supported-path second table sharing that process pool. See `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:287-405`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13746-13760`, and `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:694-708`.

## Meaningful verification still missing

The six-reference recovery law at `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs:1712-1765` manually calls `SceneRasterReleaseWitness::commit_gpu`. It proves pool/World policy but bypasses `RasterTextureTable` allocation, row upload, `commit_presented_step`, texture retirement, and its owned `SceneRasterGpuWitness`. Add an actual WGPU integration path that drives six distinct references through the real table with two CPU slots, releases the first actual table texture/witness after CPU eviction, and verifies that the same URL is re-requested, redecode/reupload completes, and plane aspect stays natural.

The reported UI 635/635 and Chromium Pixel16 (reference 13/13, paint 3/3) validate static ownership/source laws and shader bytes respectively. They do not exercise the browser full-allocation peak or the real six-asset recovery path above.
