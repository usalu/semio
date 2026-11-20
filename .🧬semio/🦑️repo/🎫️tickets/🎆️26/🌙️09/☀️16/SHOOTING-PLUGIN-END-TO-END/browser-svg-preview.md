# Browser WGPU SVG Preview

The browser wgpu preview now uses the same bounded SVG projection owner as native preview and export. The wasm raster phase parks until the browser decoder returns, then wakes the existing pane scheduler. Native continues to rasterize on the shared renderer pool. Both paths derive dimensions from the uploader byte budget.

The browser bridge copies SVG from wasm in 256 KiB pages and yields between pages. It changes only the intrinsic width and height in the standalone SVG header. The original view box, mask, layer order, fills and linework remain in the encoded image. The bitmap and RGBA readback use the bounded existing reference-image decoder. When Chromium refuses createImageBitmap on an SVG Blob in the worker, the page's existing async image decode supplies the bitmap.

Page decode retains one active credit and admits up to sixteen queued requests in arrival order. Icon preview data stays in Blob form while waiting for that credit. A cancelled waiting request drops before reading bytes. A cancelled active request asks the page to abort its exact request and retires stale responses. Reference-asset cancellation leaves unrelated previews in the queue. Shell close cancels both kinds.

## Verification

- All 65 focused Vitest tests passed, including three new tests for page transfer, fair fallback, precise cancellation and stale-response bitmap retirement.
- All four Bun reference-image decoder tests passed.
- The real Chromium Worker probe exercised the production SVG bridge and page image decoder. The worker's natural SVG decode refusal and a forced fallback each produced exactly 1,048,576 bytes at 512×512 from an 8,000×8,000 SVG. The center pixel was [68, 203, 135, 128] and the background corner was transparent.
- The repository source scanner accepted browser-target closures for icon-host and host-base. Rust syntax was checked independently. Compilation and browser shell wiring are tracked in the combined managed build.
