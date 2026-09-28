# Icon Render Export Primitives Audit

## Scope

This is a read-only implementation audit for the WGPU `Effect::IconRenderExport` gap. No export production source was changed while the WGPU 28 artifact boundary was active.

## Confirmed missing route

`IconRenderExportItem` is the neutral kernel envelope: it contains a destination filename and an opaque `DslValue` request (`framework/🔨️modules/🎠️kernel/🦀️.rs`, `IconRenderExportItem`). Shooting emits one item per requested shot from `exportActiveShot` / `exportAllShots` (`✏️s/🔌️plugins/🎥️shooting/…/🎮️commands/🖨️export/🦀️.rs`).

React owns the complete current behavior in `renderer/…/🏛️ShellHost/🟦️.tsx`: it iterates the items, calls `iconRenderPort.render(request)`, and downloads the returned data URL. The WGPU `ShellState::queue_host_effects` match has a `DownloadMediaExport` arm, but no `IconRenderExport` arm; the effect reaches the final debug-only dropped-effect arm. This is a real lost export, not a presentation-only defect.

## Reusable raster target and readback

The existing prepared WGPU path already renders the full accepted frame offscreen before the swapchain:

- `ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs::PreparedCompositeTarget` owns a texture with `RENDER_ATTACHMENT | TEXTURE_BINDING | COPY_SRC`, an sRGB view for UI, and a non-sRGB view through which World writes presentation-encoded values.
- `ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs::PreparedGpuPresentCursor` completes `ClearScene → InitializeComposite → Commands` before the terminal `Present` step touches the swapchain.
- Reading the underlying `Rgba8Unorm` bytes from this target therefore reads the same presentation bytes the browser displays. The proof must include alpha and at least one nontrivial color because UI writes through the sRGB view while World writes through the encoded view.

The clean production seam is to factor the surface-sized owner into a request-sized prepared render target used by both presentation and export. An export cursor executes the same clear/composite/command ladder, skips `Present`, then copies the completed texture to a staging buffer. It must not resize the live surface, crop the preview surface, or run a second approximate painter.

The readback must use `wgpu::COPY_BYTES_PER_ROW_ALIGNMENT` and strip padding row by row. Two repository implementations prove the mechanics:

- `ui/🖌️render/🎯️targets/🧊️webgpu/🌐️backend/🦀️.rs::PendingReadback` is asynchronous and harvested on a later turn, but is currently test-only and belongs to a different renderer.
- `framework/🔨️modules/🖌️raster/🦀️.rs::read_pixels` validates row alignment and normalization, but calls `device.poll(wait_indefinitely())`; that synchronous wait is unsuitable for the interactive renderer.

The export implementation should retain its staging buffer and `map_async` completion as a job phase. Browser work naturally resumes from later worker pumps. Native may issue a bounded `device.poll(wgpu::PollType::Poll)` opportunity per step; it must never use `wait_indefinitely` on the renderer thread.

## Existing bounded PNG encoder

`framework/🔨️modules/🔲️pixels/📷️png/✍️encode/🦀️.rs::PngEncodeJob` is the correct first-party encoder. Each `advance()` handles at most 4096 filtered bytes, returns exact `PixelProgress`, publishes only after `IEND`, and `cancel()` clears both the output and image pixels. `validate_image` limits the request to `MAX_IMAGE_SIDE = 16_384` and `MAX_IMAGE_PIXELS = 16 * 1024 * 1024`.

The renderer package does not currently declare `semio-framework-pixels` directly, so using the encoder requires a direct workspace-internal dependency; reaching through the Infinite product would violate ownership. The request must be validated against the pixel budget before allocating the GPU target, staging buffer, tight RGBA buffer, or encoder reserve. Current `IconRenderRequest` and the Shooting `UINT` shot fields do not impose this admission limit themselves.

The synchronous `semio_framework_pixels::encode_png` function must not be used for this path because it performs the whole filter/compress operation in one call.

## Exact installed Three SVGRenderer behavior

The actual authority is the installed `node_modules/three/examples/jsm/renderers/SVGRenderer.js` and its `Projector.js`, used by `ui/🎯️targets/⚛️react/🟦️.tsx::renderIconSvg`.

Its important semantics are narrower than WebGL PBR:

- Geometry is transformed to clip space, near/far clipped, perspective-divided, viewport-scaled, and emitted as triangle paths.
- Objects and faces are stably painter-sorted by `renderOrder`, depth (far to near), and object id. There is no depth buffer.
- Only `DoubleSide` bypasses front-face culling in the installed Projector. Near/far clipping can fan one source triangle into several SVG triangles.
- `MeshStandardMaterial`, `MeshPhongMaterial`, and `MeshLambertMaterial` all use the same flat face-lighting formula: accumulated ambient plus directional/point diffuse, multiplied by material color and optional vertex color, then material emissive is added.
- The material's `metalness`, `roughness`, shadow configuration, normal map, and texture maps are not read by SVGRenderer. The module's own documentation states that textures and shadows are unsupported.
- `vertexColors` multiplies one projected face color; opacity becomes `fill-opacity`; `wireframe` selects a stroked path. MeshBasic is unlit. MeshNormal uses the normal transformed to view space.
- Adjacent faces with the same final style are concatenated into one SVG `<path>`. `overdraw = 0.5` expands triangle edges before serialization.
- The outer SVG carries exact requested width, height, and viewBox. Background is the scene/background style. The React icon port removes the default clear style for a transparent request and wraps ellipse output with a clip path.

Consequently an SVG export cannot be an encoded PNG placed inside an SVG and cannot share the raster readback path. A faithful WGPU implementation needs a bounded CPU vector serializer driven by the same accepted mesh/material/light/camera packet.

## SVG job phases

The smallest clean serializer is a retained `IconSvgExportJob` with bounded phases:

1. Acquire the same accepted mesh lease, primitive/material table, camera, environment, edge lease, and presentation values used by the icon preview.
2. Traverse one primitive/index page at a time and transform one bounded triangle batch to clip space.
3. Near/far clip and front-face filter each triangle; compute the installed Three flat-lighting color for the request's `SvgFlatLit` profile.
4. Append fixed-size render records `(renderOrder, depth, objectId, triangleId, path, style)`. Admission must reject a record/byte count beyond an explicit export budget before unbounded growth.
5. Stable-sort the admitted records using Three's `(renderOrder, depth descending, object id)` order. This sort must itself be incremental or use admitted bounded pages.
6. Serialize/merge adjacent equal styles incrementally, add requested background and ellipse `clipPath`, then close the SVG only on success.
7. Append the separately derived 1.001-scale edge lines with the request's stroke visibility/color semantics at the same stage as React's edge objects.

The current GLB material carrier plan is a prerequisite for absent `request.material`: the serializer must receive primitive ranges, material color/emissive/opacity/side/vertex-color information instead of flattening the GLB to one environment material. Texture references still belong in the neutral carrier for PNG/WebGL parity, but the actual installed SVGRenderer ignores material maps; the SVG oracle should freeze that deliberate behavior rather than invent texture mapping.

## Job, cancellation, progress, and publication

Use the existing `semio-framework-job` vocabulary and renderer `frame-job` pattern. One `IconExportJob` should own:

`Validate → AwaitAsset → BuildAcceptedPacket → RenderOrProject → Readback (PNG only) → Encode/Serialize → Deliver → Close`.

Progress must be a real witness: asset pages accepted, render command pages completed, rows normalized, `PngEncodeJob.completed`, or SVG triangles/bytes serialized. Polls that observe an unchanged GPU mapping or unavailable asset report Pending without incrementing progress. Cancellation drops the candidate target/staging buffer, calls the PNG encoder's `cancel()` when present, closes mesh/texture leases incrementally, and never publishes partial bytes. A submitted GPU copy cannot be retracted, but its mapped result can be retired without delivery.

Items should be processed sequentially or through the existing globally admitted job pool. Exporting `N` shots must not create `N` independent GPU targets and full RGBA buffers without an owner-controlled bound.

## Byte delivery

WGPU already has the correct final doors:

- Browser `host_io_call({ op: "download-media-export", ... }, Some(bytes))` transfers bytes from the worker to `host-io/🟦️.ts`, which calls `downloadBytes`.
- Native `download_media_export_worker` opens a save dialog and writes bytes.

The current helper accepts `(data: &str, encoding)` and decodes through `kernel::media_export_bytes`. Factor a sibling `download_media_export_bytes(filename, mime, &[u8])`; make the existing envelope decoder call that door, and let the icon job call it after acceptance. This avoids base64/string round trips and retains the current browser/native authority. SVG should publish UTF-8 bytes with `image/svg+xml`; PNG publishes encoded bytes with `image/png`.

## Required proof packet

1. A language-neutral export lifecycle fixture covering request validation, ordered multi-item processing, PNG/SVG MIME and extension, cancellation before render/readback/encode, and no partial delivery.
2. Production WebGPU readback against a small exact RGBA fixture, including an unaligned width, transparent pixel, and nontrivial sRGB color.
3. PNG bytes decoded by the existing first-party decoder and an independent PNG library, matching the accepted RGBA bytes exactly.
4. SVG output compared with the installed Three `SVGRenderer` for clipping, face order, standard-material flat lighting, vertex color, opacity, transparent/background, ellipse clip, and 1.001-scale edges. Add a mapped material case proving Three intentionally ignores the map.
5. Browser worker-to-page transfer and native byte-writer laws proving exact byte identity and cancellation.
6. A Shooting end-to-end effect law proving every item produces exactly one accepted download and a cancelled/faulted item produces none.

## Remaining limits

- No export runtime implementation or build was attempted during this audit.
- The renderer has no current direct pixels-crate dependency.
- A faithful SVG implementation depends on the accepted GLB primitive/material carrier; the neutral/two-primitive GLB oracle exists, while the carrier remains intentionally deferred behind the active artifact boundary.
- Shadows and texture maps must not be claimed for SVG because the actual installed Three SVGRenderer does not implement them.

## Request-owned prepared scene checkpoint

The decoded asset handoff now has an explicit bounded owner rather than relying on a preview surface. `World3dMeshAsset` retires its primitive records, pending texture leases, and paged mesh lease one unit at a time; `World3dMeshAppearance` exposes the matching terminal-empty witness. Rejected asset delivery can therefore stay in the bounded export registry until every owner is released.

`IconExportScenePreparation` parses and validates the same request used by the preview, checks integral positive dimensions against the first-party image budget before target allocation, resolves the renderer transport URL, and publishes the complete request-owned mesh/appearance into an isolated World state at the mesh revision. It reuses `icon_render_world_scene` with scale one, so the authored camera, full material inheritance/override, environment, transparent/explicit clear, source aspect, and rectangle/ellipse viewport mask remain one authority for preview and export.

The scene preparation cursor advances the World scene bridge, draw rebuild, snapshot, resource transfer, and `PreparedRenderJob` through bounded turns. It refuses a packet whose World pass lost the requested viewport mask. Success returns `IconExportPreparedScene`, which owns both the prepared packet and the World state whose mesh/texture leases back that packet. The packet may transfer once to the GPU export cursor; only after the GPU cursor reaches terminal may the caller begin bounded retirement of the retained World state. Cancellation and every rejection arm incrementally release inputs, jobs, outcomes, packets, rejected resources, build resources, and dynamic World owners without publishing a partial packet.

PNG and SVG remain distinct accepted formats at this boundary. The prepared raster packet is appropriate only for PNG; the format accessor prevents the Shell batch from silently routing an SVG request through PNG. SVG still requires the separately audited vector serializer.

The source checkpoint includes one production materializer law that hands the actual neutral two-primitive GLB asset into request-size scene preparation, reaches a prepared packet with mesh and embedded raster uploads, transfers the packet once, and proves packet plus World retirement reach exact terminal empty. Its compiler/native receipt is pending the shared Cargo queue; no passing claim is made yet.
