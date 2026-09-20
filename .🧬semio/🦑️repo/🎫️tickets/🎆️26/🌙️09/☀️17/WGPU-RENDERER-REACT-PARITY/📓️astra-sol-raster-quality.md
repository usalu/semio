# Full-Resolution Scene Raster Ownership

## Scope

Final state: the schema and neutral model are now implemented through the Rust pool, browser and native decode handoffs, lease-backed prepared/GPU upload, committed GPU acknowledgement, and World reference/paint publication. The chronological phase-one notes below explain why the implementation uses a distinct lease producer.

This packet follows `📓️terra-raster-quality-audit.md`. Phase one is deliberately disjoint from the active native/wasm compiler snapshot: it adds the schema-owned full-quality contract, neutral fixture, page-owned pool model, and ownership laws. It does not yet wire World, prepared rendering, GPU upload, renderer wasm bindings, or frame-worker staging.

The implementation does not enlarge the existing source-owning `PreparedRasterProducer`. Full quality requires a distinct lease producer so the decoded pool remains the only CPU pixel owner.

## Contract decisions

The new `scene-raster-ownership` schema owns:

- 64 MiB per decoded RGBA8 item, four slots, and 256 MiB of decoded-pool credits;
- the existing 16 MiB encoded source and 16,777,216-source-pixel limits;
- one active Worker decode, page fallback, and native decode, with bitmap/canvas/native workspaces accounted separately from the decoded pool;
- 1 MiB readback, upload, and retirement chunks, row-aligned at upload time;
- one live upload per surface, monotonic `uploadedBytes`/`totalBytes`, partial-texture retirement before release, and a committed acknowledgement;
- exact credit handoffs for encoded source, decode workspace, decoded pool, prepared logical bytes, GPU upload bytes, and resident GPU bytes;
- zero-lease-only LRU eviction, all-live retryable backpressure, exact generation/token cancellation, and terminal retirement;
- reference image-map, reference canvas-raster, and mesh-paint decode profiles.

The current renderer host forces references through the image-map path. Its reachable JPEG/PNG reference profile uses Three `NoColorSpace`, as does mesh paint. The lower-level SVG/PDF canvas-raster path uses sRGB but is marked unreachable from the current renderer record. Full-resolution acceptance is therefore PNG/JPEG first. AVIF, TIFF, and PDF remain unimplemented WGPU capabilities; signature admission is not reported as decoding support.

The representative plan remains 2275×2560, or 23,296,000 RGBA bytes. A 1 MiB row-aligned upload chunk carries 115 rows at width 2275, requiring 23 chunks instead of the existing 2,560 one-row writes.

## Neutral pool model

`SceneRasterPool` models the production authority without target dependencies:

- slots transition `vacant → building → ready → retiring → vacant`;
- `SceneRasterWriter` validates slot, non-wrapping `u64` epoch, owner token, consumer generation, and exact cursor;
- each pushed page becomes the pool's immutable owner and feeds the BLAKE3 content digest once;
- admission copies and freezes identity metadata; caller mutation cannot change a building or ready identity;
- sealing returns an exact `SceneRasterLease` with a unique one-use lease ID; a matching second pane registers a distinct lease without a second decode or allocation, and duplicate/copied-token release is inert;
- each ready slot admits at most 64 active leases. A refused 65th lease consumes no ID, changes no lease count, and leaves the admitted pixels and identity untouched;
- constructor limits and admitted identities are copied and frozen at the authority boundary. The neutral model exposes only a bounded copied page read, so callers cannot mutate the pool after its digest is sealed; the Rust implementation will expose a scoped immutable borrow instead of another pixel copy;
- cancellation invalidates the writer immediately and retires only pages it already wrote;
- ready live leases cannot be evicted; a zero-lease LRU first enters bounded retirement and the producer retries after maintenance;
- mesh paint seal validation requires the exact mesh key, mesh revision, and nonzero UV witness.

This TypeScript module is the neutral executable ownership model. The Rust process authority implements the same state machine. A World reference or S2b paint publication holds a CPU lease only while pixels are decoded or uploaded; committed GPU residents retain the immutable raster identity rather than pinning a CPU slot.

## Shared wiring plan after compiler release

1. Add the Rust process pool and exact lease/writer types without changing World residents yet.
2. Replace browser `stageReferenceImage` with begin/push/seal row-strip calls. Worker readback uses bounded `getImageData` strips and never materializes one full 64 MiB `Uint8Array` at the wasm boundary.
3. Move native decode output into the pool on its maintenance lane with no frame-path clone; cancellation fences publication and retirement.
4. Change pending `reference_pixels` and `mesh_paint_textures` publications to exact upload leases. Mesh paint seals only against current mesh revision and UV topology. The existing painted draw/material/shader endpoint remains intact.
5. Add a lease-backed prepared producer carrying precomputed identity and page cursors. It performs no full clone and no second `RasterContentIdentity::mix_bytes` pass.
6. Upload row-aligned 1 MiB chunks, retain the committed prior texture during replacement, retire partial resources on cancellation, and send the committed acknowledgement that stops World from re-offering a resident raster.

Credits remain held through their exact handoff or terminal outcome. Pool bytes remain charged through last-lease release and retirement; prepared/GPU upload credits remain charged until commit acknowledgement or partial-resource retirement; resident GPU credits remain charged through texture retirement.

The CPU pool is a decode and upload transit owner, not the durable owner of every visible texture. A successful GPU commit records the immutable raster identity in the resident texture table and releases the upload's CPU lease. The previous GPU identity remains resident until replacement commit and texture retirement. This permits more than four distinct visible GPU textures to commit sequentially through four CPU slots; it also preserves concurrent same-source reuse while two panes are decoding or uploading. Device loss reacquires or decodes from the source identity instead of pinning all decoded pixels indefinitely.

Native moved publication splits digest preparation from the final pool mutation. The maintenance lane briefly validates the writer, hashes the moved RGBA allocation outside the pool mutex, then the final seal revalidates the exact writer, length, and prepared identity before publishing. Decode, allocation, and the 23–64 MiB identity scan therefore never run while frame uploads or acquisitions are excluded by the shared pool lock.

## Final production integration

The Rust pool implements the neutral writer, one-use lease, immutable scoped row read, zero-lease LRU, and terminal retirement contract. RasterUploadPixels::Scene carries a borrowed lease through preparation and upload. Its content identity is precomputed by the pool, so the frame does not scan or clone the full allocation. The GPU table uploads row-aligned chunks, retains the old texture until replacement commit, and owns the CPU release witness and committed GPU witness through cancellation, replacement, and retirement.

The browser ABI is beginReferenceImage, pushReferenceImageRows, then sealReferenceImage. Begin returns an explicit busy, writer, or exact-reuse mode. The Worker asks before decoding. Exact reuse skips browser decode and all pixel streaming. Writer mode decodes once, validates the oriented natural dimensions, and transfers row-aligned strips no larger than 1 MiB with a current-generation check, wake, and macrotask boundary between strips. The previous production ReferenceImageDecodeCache was removed because it retained a second complete decoded allocation after publication and could not express Rust lease retirement. Cancellation uses the exact staged-owner discard path.

Native decode runs on the mandatory WorkerPool. It keeps the encoded owner until submission, decodes and orients off the frame thread, prepares the full content identity outside the pool mutex, then revalidates the writer and exact allocation under the short publication lock. A cancelled or stale generation cannot publish. A pool-backpressured result stays owned and retries after one real pool maintenance step.

World reference_pixels and mesh_paint_textures retain WorldSceneRaster identity plus at most one pending CPU lease. The reference identity preserves the natural 2275×2560 dimensions used by plane aspect. GPU commit releases the CPU lease and records the exact key/identity witness. Later renders offer nothing while that witness is live. If both CPU identity and GPU witness disappear, the visible-reference request filter reopens the same URL instead of treating an identity-only World row as ready. The recovery law drives six distinct references through two CPU slots, drops the oldest GPU witness after CPU eviction, reserves the source again, republishes it, and preserves its natural aspect.

### GPU table authority

Production has one RasterTextureTable per live renderer runtime. The only production constructor is inside GpuContext::from_surface; native AppHost owns one GpuContext, and browser BrowserRendererBootstrap owns one optional GpuContext. Native and browser are separate target processes. The global World raster pool therefore records GPU readiness against that runtime's one table. Unit harnesses may construct isolated tables, but they use isolated pools and do not participate in World readiness. A second simultaneous production table sharing the pool is outside the supported topology and would require an explicit table authority in the resident key.

Within the supported topology, table teardown drops every SceneRasterGpuWitness. World readiness then observes neither CPU nor GPU ownership and reopens the source. The unchanged-content table path also mints a witness from the offered lease when an already-live texture predates GPU acknowledgement; it commits the exact key/identity before releasing the reused CPU lease.

### Colour profile and texture storage

All scene raster uploads use Rgba8UnormSrgb, matching the texture storage used by the verified S2 painted and S3 reference pixel oracles. The profile remains semantic. Reachable image references and mesh paint are NoColorSpace; their shaders sample sRGB storage as linear and explicitly apply world3d_linear_to_srgb to recover the source byte values. The currently unreachable canvas-raster profile selects the already-linear sample branch when its host route supplies the appearance flag.

Using Rgba8Unorm for the NoColorSpace profiles would make the existing recovery branch encode the byte values a second time. A Rust draw law checks all three raster profiles retain sRGB storage. Root's Pixel16 receipts execute the actual Chromium production WebGPU shaders with the restored storage contract: reference-visual-wgpu passed 13/13 and s2-painted-wgpu passed 3/3, with RGBA8 maximum delta zero.

## Verification

Focused command:

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts' nx exec '--projects=workspace' -- bun x vitest run '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖼️scene-raster-ownership/🟦️.ts' --config '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts'
```

Result: the permanent Vitest route passed 11/11. Ajv validates the language-neutral fixture. The laws cover natural dimensions/chunk count, distinct two-pane leases and duplicate release, the atomic active-lease capacity refusal, immutable constructor limits, immutable copied reads, immutable admitted identity, epoch exhaustion, all-live refusal, exact cancellation and bounded retirement, zero-lease LRU replacement, row-aligned upload progress, commit acknowledgement, cancelled partial upload retirement, six distinct GPU residents making progress through four CPU slots, and mesh revision/UV seal validation.

The browser transport route passed 41/41 after the final ABI transition. Its production-source law checks begin-before-decode, writer-only decode/streaming, explicit exact reuse, generation checks per strip, 1 MiB row math, wake/yield progress, exact seal, Rust wasm bindings, and absence of the former production decode cache. Every touched Rust source parses through rustfmt --emit stdout. Git diff whitespace validation is clean for the packet.

The first Rust milestone adds the UI-owned fixed pool authority with four slots, 64 active leases per slot, streamed browser admission, moved native admission, one-pass content identity, scoped immutable row reads, and exact retirement. Stream writers return a new exact cursor after each admitted row chunk, so replaying a copied prior cursor cannot append bytes twice. Moved writers reject the copied `seal` path before changing their slot and retain authority for a later zero-copy seal or cancellation. Mesh-paint profiles require a nonzero mesh/UV seal at pool admission.

Rust currently owns each raster as one contiguous `Vec<u8>`. Retirement therefore holds the full slot credit and releases that one backing allocation in one maintenance opportunity. It does not claim page-sized physical retirement progress; the chunked guarantees apply to browser readback, streamed pool writes, and GPU upload. The uneven reservation law keeps a live 16-byte slot readable while terminal cancellation releases only the adjacent 8-byte slot credit.

Root's UI 41 receipt passes 635/635, including the final pool, GPU-credit, and storage-profile laws. Pixel16 passes all 13 reference and all 3 painted production WebGPU samples byte-exact. World, native renderer, wasm activation, and a fresh 2275×2560 application profile remain root-owned pending receipts. No Cargo, wasm, generator, or browser build was started by Sol.

## Files

- `🧬️schema/🖼️scene-raster-ownership/🔣️.json`
- `🧫️fixtures/🖼️scene-raster-ownership/🔣️.json`
- `💾️scene-raster-ownership/🟦️.ts`
- `🧪️tests/🖼️scene-raster-ownership/🟦️.ts`
- `🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`
