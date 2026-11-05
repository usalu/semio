# Browser Source Preparation Review

Read-only review of Paint2dHost editing `📥️source/🟦️.ts`, its schema/neutral fixtures, editing unit tests, integration and the SDK base64 export. Tests were not executed; source-admission FD3 failure remains the verification limitation documented in `📥️auxiliary-diagnosis.md`.

## Reviewed Behavior

The strict shared Base64DecodeCursor validates before result allocation, enforces maximum decoded output 67,108,864 bytes, and accepts 32,768 work-unit grants. Each incomplete grant yields to a timer. Its progress total is encoded string length in both phases, so validation completed/(2 total), then (total+decode completed)/(2 total), is monotonic. Empty input safely ends at an explicit 1. The helper checks cancellation before work, around each yield, around each cursor progress callback, and after explicit final progress before taking output. A callback-triggered final cancellation cannot publish bytes.

`pixelReadbackView` preserves the exact ArrayBuffer, byte offset and byte length of the browser-owned clamped readback. It does not copy or widen a subview. The output check narrows the decoded ArrayBuffer view before creating the returned Uint8Array; no external runtime dependency is added.

The added blank helper validates extent/target before allocation and checks cancellation from initial, intermediate and final progress callbacks. Mask filling occurs in bounded byte grants; transparent pixels use zero-initialized owned allocation. The original full-image synchronous mask fill is removed at the decodeLayer integration point. SDK `🧰️framework/📦️packages/🟦️typescript/🚪️io/🔤️base64/🟦️.ts` explicitly reexports Base64DecodeCursor from the existing codebase-owned module, and its package manifest already exposes io/base64.

Neutral fixtures and authored tests cover empty/padded/binary data, canonical refusal, yielding, monotonic phase progress, cancellation before/within both phases/at completion, exact subview ownership, blank pixels/masks and mask-fill grant boundaries. Independent base64-js and Sharp checks are authored. Their presence is not a passing result. A decoded-output capacity refusal test is not presently in the new fixture; the cap is delegated to the existing cursor admission law.

## Scope Limits and Existing Follow-Up

The helper bounds JavaScript validation/decoding/fill loops; it does not make JSON.parse of assets, output allocation, Blob creation, browser createImageBitmap, drawImage or getImageData individually cancellable. decodeLayer checks cancellation after the bitmap resolves and closes the bitmap in finally. Avoid reporting complete large-image responsiveness or live browser acceptance from this change alone.

An existing UI progress reset remains in editing TSX: select() reports combination in 0.5–1, then publishSelection() reports RLE preparation in 0–1. This is separate from the new source helper and not a defect in its two-phase formula. A complete monotonic workflow needs fixed ranges across source, flood/shape, combination and RLE publication. The final-authoritative command continues to require its existing revision/cancellation guards.

No blocking correctness defect was found in the newly added helper by static review. Runtime/type-check status is unverified.

## Mounted Source Tests Follow-Up

Reviewed the four new Color picker/Magic wand mounted cases. Found one concrete harness mismatch: the original bitmap oracle called Blob.arrayBuffer, absent from the installed jsdom Blob implementation. Parent replaced it with FileReader.readAsArrayBuffer; the corrected source was reread. Canvas getContext mock uses the installed DOM overload returning RenderingContext|null and returns 2D context only for intrinsic 2×1 readback/mask canvases, leaving the 100×100 overlay null. Its implemented drawImage/getImageData/createImageData/putImageData methods cover those source and mask paths. No additional static harness/type defect was identified.

The test center maps to intrinsic pixel (1,0), matching fixture foreground #ff4008 and one selected span at 1. The timer gate is armed after a genuinely incomplete cursor step; its next timer is the source helper yield, so Cancel is observed during preparation before createImageBitmap. Cancellation fixture bytes intentionally need not be a valid PNG because browser decode must never be reached. Cleanup restores the timer, cursor spy and globals.

Current authored additions expand to 20 source unit cases (4 valid, 6 invalid, yielding, 4 cancellation stages, readback, 2 blank targets, blank grant boundary, invalid blank input) and 4 mounted cases. None was run in this review. Mounted bitmap creation and canvas readback remain mocked: Sharp independently decodes the actual Blob bytes, while the canvas supplies authored raw pixels. These tests establish wiring expectations, not real-browser bitmap/canvas acceptance.

SDK Base64DecodeCursor is explicitly exported. Current absolute source/fixture/schema paths are 136/147/148 UTF-16 code units; the mounted test path is 155 and SDK export path 78, all below 256 in this checkout. No source copies were added to the ticket.
# Encoded Source Byte Admission

The Raster import route does not establish a 64 MiB encoded-file ceiling. `🚪️io/🧩️composite-and-media/🦀️.rs::raster_image_layer_and_asset` base64-decodes the full payload and calls `semio_image_from_png_bytes`; `🚪️io/🌉️semio-bridge/🦀️.rs:153` delegates directly to stdio PNG's ordinary decode. `🧬️schema/🖼️asset-content-and-patch/🦀️.rs::RasterImageAsset` stores an unconstrained `Vec<u8>` carrier.

The ordinary PNG decoder is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🧭️layout-and-projection/🦀️.rs:74`; it delegates to `decode_png_image`, whose internal `NativeDecodeControl` owns a 512 MiB allocation budget (`io/🔓️native-decode/🦀️.rs:6`). This is a decoder allocation budget, not a source file-size admission contract. The same layout file's 64 MiB constant belongs to `png_preview` RGBA output, not encoded PNG input.

A separate controlled native interface (`io/🚦️native/🦀️.rs:7`) explicitly receives `SqliteDatabaseLimits` and checks `data.len() <= limits.max_file_bytes`. Raster's ordinary bridge does not invoke that interface. Its existence supports explicit caller admission rather than inventing a fixed encoded-byte cap in browser preparation.

Recommendation: require an explicit maximum output-byte admission from the caller of base64 preparation, or pass the already admitted exact decoded encoded-file byte length. Keep the 16 MP/64 MiB RGBA extent policy separate. No shared applicable fixed encoded PNG cap was found on the actual Raster import path; do not replace the browser limit with the unrelated 512 MiB allocation budget.

