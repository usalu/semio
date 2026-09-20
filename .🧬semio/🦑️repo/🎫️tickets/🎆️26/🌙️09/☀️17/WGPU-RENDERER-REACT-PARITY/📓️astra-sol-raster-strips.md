# Browser Raster Strips and Exclusive Writer Reentry

## Result

The browser reference path no longer creates a complete `ImageData` and then duplicates it with `Uint8Array.from`. `decodeReferenceImage` now returns the browser-owned `ImageBitmap`; `streamReferenceImageBitmapRows` draws at most one row-aligned strip into an `OffscreenCanvas`, reads at most 1 MiB, passes the zero-copy `Uint8Array` view directly to `pushReferenceImageRows`, yields one macrotask, and checks the exact generation before and after every yield. The final generation check happens after the final strip and before seal.

Worker and page fallback decode both produce an `ImageBitmap` through `referenceImageBitmapForStage` and then enter the same strip streamer. Stage mode `2` returns before either decoder callback is invoked. The removed `DecodedReferenceImage` / `ReferenceImageDecodeCache` TypeScript API leaves no browser production handback containing a complete CPU RGBA frame.

The representative 2275×2560 plan retains its natural 23,296,000-byte result. Its 9,100-byte rows admit 115 rows per strip: 22 strips of 1,046,500 bytes and a final 273,000-byte strip. Natural size, browser-default sRGB conversion, straight alpha, and source orientation remain intact. The EXIF scanner now retains the first non-default orientation instead of allowing a later embedded orientation-1 APP1 block to overwrite it, matching Chromium and Three for the real plan fixture.

`StagedReferenceImageAuthority::begin` now reports `BUSY` (`0`) for an exact entry still in `Writing`. Only the first begin reports `WRITER` (`1`), and only an exact sealed `Ready` entry reports `REUSED` (`2`). The direct native state law asserts `1 → 0 → push/seal → 2`.

## Contract and tests

The language-neutral decoder schema and fixture now own the 64 MiB natural-resolution decoded ceiling, the 1 MiB row-strip readback ceiling, a row-strip mode, a yield after every strip, and expected strip rows/counts for every neutral case. The old cache fields and cache test were removed with the full-frame TypeScript cache API.

Focused results:

- Decoder schema/metadata/orientation laws: 4 passed, 0 failed, 34 assertions.
- Browser-frame transport laws: 41 passed, 0 failed.
- Strict standalone decoder TypeScript check: passed.
- Strict standalone frame Worker TypeScript check: passed with the Web Worker async iterable libraries enabled.
- Isolated Chromium/Three oracle: 8/8 codec rows passed with exact dimensions and sampled RGBA (`PNG`, `GIF`, `WebP`, `BMP`, `SVG`, normal JPEG, EXIF 6, EXIF 8).
- Chromium strip instrumentation: 23 contiguous readbacks/pushes/yields; maximum readback 1,046,500 bytes; final readback 273,000 bytes; every callback sequence was `readback → push → yield`.
- Chromium cancellation: generation retired after the first non-final strip; exactly one push and one yield occurred before `AbortError`.
- Chromium page fallback: one forced primary refusal, one page decode, one bounded strip through the shared streamer.
- Chromium exact reuse: stage mode `2` returned no bitmap with zero primary and zero fallback decode calls.

The root-owned native census includes the new state law. My attempted focused Cargo command was stopped during compilation at the root coordinator's request before any test result; no native pass is claimed here. No activation or application artifact generation was run.

## Source-coherent marker

Production:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🖼️reference-image-decode/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📖️stories/🎭️reference-media/🧪️.story.tsx`

Contract and verification:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🖼️reference-image-decode/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🖼️reference-image-decode/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖼️reference-image-decode/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️reference-decode/📜️script.ts`

## Commands

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts' nx exec '--projects=workspace' -- bun test './🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖼️reference-image-decode/🟦️.ts'
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts' nx exec '--projects=workspace' -- bun x vitest run '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts' --config '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts'
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts' nx exec '--projects=workspace' -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️reference-decode/📜️script.ts' browser-codec-oracle
```
