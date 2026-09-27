# PNG Pixel Region Workflow

## Existing gap

The PNG edit window renders `ImageWindowKit::window_kind()`, so the primary canvas is read-only. The shared editable image kit advertises `set-pixel-region` without arguments or an owning retained factory, and the PNG editor's old `SetPixelRegion { pixels }` branch replaces the full pixel vector directly. That branch is neither reachable from an action nor cancellable and does not preserve bounded publication for large images.

PNG snapshots already carry canonical row-major RGBA8 pixels independently of source bit depth and color type. `PatchPixelsMutation` is the correct durable primitive: it validates a byte range, computes an exact inverse from the authoritative base, and has text/binary codecs.

## Interaction contract

`set-pixel-region` is a typed mutation action with eight required fields: zero-based `x`, `y`, positive `width`, `height`, and byte channels `red`, `green`, `blue`, `alpha`. The window action is visible in the image toolbar/action surface; the manifest provides localized English/German labels and descriptions and typed numeric controls. The host action form is keyboard operable and exposes every field by label.

The command validates finite integer arguments, channel bounds, checked coordinate addition, image bounds, canonical `width * height * 4` storage, and the PNG editor's 32 MiB raster ceiling before work starts. The ceiling admits a 3840 × 2160 RGBA8 raster at 33,177,600 bytes while retaining a hard upper bound.

## Retained execution

A dedicated retained job prepares `PatchPixelsMutation` rows incrementally. For rows no wider than the mutation chunk budget it groups complete rows and changes only the requested columns. For unusually wide rows it chunks the requested span. Each mutation and its inverse remain below the one-item store limit. Work publishes the complete mutation vector only after preparation, so cancellation cannot leave a partially painted region. Progress stages are bilingual.

## Neutral and third-party oracle

The language-neutral fixture defines RGBA8 before/command/after vectors, including partial-alpha and boundary cases. The TypeScript witness applies the first-party region operation and compares it to `pngjs`'s independent `PNG.bitblt` composition, then encodes and decodes the result with `pngjs` to confirm native PNG pixel bytes.

## Propagation plan

1. Reuse the region command and retained work pattern for BMP and TIFF once each family exposes or derives a canonical RGBA8 editing projection.
2. JPEG needs a decoded RGB editing projection plus explicit lossy-export policy; retain source metadata independently from pixels.
3. GIF needs frame-index and palette-aware region arguments; one retained job publishes frame-local compact pixel/palette mutations.
4. SVG should keep vector-native geometry editing as its primary workflow and use raster region editing only for embedded raster images.
5. Shared `ImageWindowKit` should remain presentation-only until a domain-neutral interaction record can carry typed coordinate/color state without claiming every image format has the same pixel semantics.

## Implementation checkpoint

The PNG main window now replaces the bare image-kit action with the typed action above. Its retained factory prepares bounded `PatchPixelsMutation` records, reports preparation progress, and publishes nothing when cancelled. The retained preparation steps and 32 MiB admission bound do not yet prove bounded end-to-end publication: inverse, diff, and apply still cross an atomic shared-store seam. Rust coverage for ordinary UHD admission is authored but has not run in the current native build; the shared cooperative seam must land before this is treated as an end-to-end performance proof.

The uncached `@semio-tech/stdio-png` Nx TypeScript suite passed on 2026-09-27: two tests, six assertions, including independent `pngjs` bitblt and encode/decode checks. That suite covers the language-neutral small raster cases; it does not execute the new native UHD admission test.

Compact `PatchSnapshot` leaves are mounted for PNG, JPEG document, TIFF document, MP4, and WAV. Their aggregate schemas, mutation variants, text/binary tags, inverse routes, and catalog declarations now carry the shared typed patch. Generic Details edits publish compact path mutations while existing native metadata/sample/pixel mutations remain preferred. The custom hex text codecs decode ASCII byte pairs and reject malformed Unicode without slicing UTF-8 at invalid boundaries.

Semio Mesh and BREP no longer expose a payload-free `set-vertex`. Their window and app actions require stable entity ids and a finite target point (plus primitive and vertex index for Mesh), provide English/German labels, reject missing or malformed payloads, and fault on stale targets instead of moving an implicit vertex to the origin.

The native UHD case paints the bottom-right pixel of a 3840 × 2160 RGBA8 raster and asserts a bounded encoded patch while retaining every preceding pixel. It remains source-only pending the warmed full native build and the shared cooperative publication hook.
