# BMP Primary Editing and Fidelity Next Cut

Read-only source inspection by root while Office/media/native gates run. No BMP production edit or native test was performed in this inspection.

## Current route

The BMP editor main window is an ImageWindowKit preview and re-encodes the whole snapshot synchronously. Typed Details use a compact PatchSnapshot route in the current source, so the September whole-snapshot inventory is stale for BMP. The `SetPixelRegion { pixels }` command is an unaddressed complete-payload replacement: no natural action roster entry, parser branch or visible region controls reach it. The initial document has zero dimensions. A usable bitmap creation and coordinate/region editing flow remains required.

The canonical decoded pixel buffer is RGBA8 with top-origin rows. The PNG command already has bounded region planning and patch batching, but those types are PNG-specific. Copying that entire command into BMP would duplicate raster geometry and refusal semantics. A neutral raster region contract/planner should own coordinates, RGBA values, checked geometry and bounded byte spans; artifact-specific adapters should own revisions, command lifecycle and patch publication. Empty bitmap creation must atomically establish dimensions and payload. Indexed images require palette-aware editing or an explicit format change rather than silent quantization.

## Fidelity prerequisite

BMP decoder supports indexed1/4/8, BI_RGB24/32 and BI_BITFIELDS16/32. Its snapshot currently retains only the basic11 header fields, palette and decoded RGBA pixels. Source comments and exporter document an encode scope of indexed1/4/8 or BI_RGB24,40-byte header. Bitfield masks, extended-header fields and original representation are not fully owned by the snapshot; source inspection therefore cannot support an every-detail save claim. A later native external-decoder oracle must cover transparency and bitfield/custom metadata before claiming natural bitmap editing preserves the file.

The hand-written snapshot text parser strips whitespace, consumes hexadecimal byte pairs and silently ignores an odd final nibble. It also slices UTF-8 by raw byte offsets, which can panic for malformed non-ASCII input. This is a concrete validation gap; invalid source input must refuse atomically and retain the draft. Its fix should be covered by language-neutral valid/odd/non-ASCII fixtures and an independent byte parser plus BMP decoder.

## Execution order

1. Prove source parser refusal cases and repair them without lossy normalization.
2. Define complete bitmap ownership needed for reversible metadata/representation edits and verify it with independent save/reopen fixtures.
3. Extract the shared raster region planner from PNG, preserving its existing oracles.
4. Add creation, coordinate/region controls, format/palette controls and real history/progress/cancel/browser evidence for BMP.

Scope ownership: root reserved BMP/TIFF from the media worker; no production files were changed at this checkpoint. Office, media export and paged ownership remain the three active Sol execution lanes.

## Root Source Parser Execution

Nine neutral source cases and JSON schema are now authored under BMP snapshot fixtures. The native law uses the development-only `hex0.4.3` library for independent admission/byte output, catches native parser panics, and checks decoded pixels and source roundtrip. The source twin passed14 assertions through the registered canonical-architecture target after refreshing the Nx graph. The native red1 run is pending; production parser has not yet changed. Offline workspace lock synchronization passed after adding the already pinned oracle crate. Both source and focused native commands are registered in launch.json. BMP/TIFF natural surface and representation work remains outstanding.

## Actual Red Result and Repair

The fresh native source-hex red1 ran and failed its one law (96 unrelated tests skipped). Both odd-tail cases were accepted incorrectly, and both emoji cases panicked at raw UTF-8 byte boundaries. The law collected every mismatch before failing, so all four defects were observed in this run. The parser now validates complete ASCII hexadecimal pairs before allocating/slicing and consumes every validated pair. Green1 is pending; no runtime success is claimed yet.

## Native Green Result

Green3 passed the actual native source-hex law1/1,96 unrelated tests skipped, cache skipped, Nextest0.015s/Nx4m44s. All9 cases agree with the independent hex decoder; valid bytes decode to the expected RGBA pixel and source roundtrip, odd trailing digits refuse, and non-ASCII input no longer panics. Green1 was blocked by disk exhaustion and green2 by an in-flight shared iterator API; neither was a passing test. The larger BMP representation and natural-canvas work remains open.

## Byte-Authoritative Implementation Checkpoint

The persisted BMP artifact and snapshot now own only the schema identifier plus the exact source bytes. BmpLayout, palette entries and RGBA8 are checked ephemeral projections. Import accepts the declared 40-byte BITMAPINFOHEADER v3 profiles only: indexed BI_RGB 1/4/8, direct BI_RGB 16/24/32 and BI_BITFIELDS 16/32. DIB sizes 12/52/56/108/124 are rejected before document construction so another header family cannot be normalized through the v3 route. No-op export returns the exact owned byte sequence, including file-header reserved words, palette reserved octets, duplicate palette identities, row padding, pre-pixel gap, and trailer.

The old editable header/palette/RGBA second authority and its change/insert/remove/replace mutations were removed from the mounted schema. The public mutations are whole snapshot replacement, path-scoped details patching, revision-guarded indexed-region paint and revision-guarded direct-sample paint. Indexed paint edits packed indices without color re-quantization. Direct paint edits only declared channel masks and preserves unmasked sample precision, padding and unrelated bytes. Both paths expose progress/cancellation callbacks and exact whole-byte inverse through the canonical SetSnapshot event. A format-neutral checked byte-span plan now lives in the Stdio editing contract for future retained sparse application without routing PNG/TIFF through BMP parsing.

A language-neutral canonical byte fixture matrix now covers all eight accepted profiles plus top-down storage. It includes a 24-bit file with nonzero padding/reserved fields/gap/trailer; 1/4/8-bit indexed files with duplicate-color, distinct-index entries and nonzero palette reserved octets; BI_RGB 16/32; and BI_BITFIELDS 16/32. Five foreign DIB sizes are rejection fixtures. Rust laws assert exact no-op bytes, changed-range confinement, duplicate-index identity, stale/cancel refusal and unmasked-bit preservation. The development-only image decoder independently reopens every accepted fixture and verifies all indexed planes. SQLite owns one exact BLOB carrier and an independent Bun SQLite law edits one raw sample without rebuilding surrounding bytes.

Preview23 exposed 61 production compile errors from the first partial mount. The paired leaf descriptors/codecs, aggregate protocol registries, derived layout consumers, guarded SQLite owner and editor/viewer projections were repaired as one source cut. Preview24 is the first compiler validation of that repaired production boundary and is still pending at this checkpoint. The rewritten native tests and oracle laws have not run yet. Natural accessible editor actions and retained job progress remain the next implementation step; no completeness claim is made.

## Canonical BMP Editing Checkpoint

The production snapshot remains byte-authoritative. The mounted editor now exposes revision-guarded indexed and direct region paint commands through retained work, publishes exact SetSnapshot inverses, and reports progress/cancellation. Direct paint preserves unmasked sample precision and every unaddressed byte; indexed paint preserves duplicate palette-entry identity. The canonical history fixture set contains exact before/after DSL bytes, mutation JSON and outcome JSON for both paint routes. The registered package router includes `component-app-assembly`, so the ordinary BMP native target compiles the actual editor and viewer rather than a reduced library surface.

The native BMP target passed 42/42 tests in `🗑️generated/bmp-canonical-native-9.log`. Its pre-native twins passed 14 source-hex checks and four paint-region checks. The independent image-rs target passed 4/4 in `🗑️generated/bmp-image-rs-oracle-3.log`: all admitted v3 profiles reopen, exact paint outputs reopen, the neutral opaque RGBA8 conversion reopens, and the supported 8-bit raw indexed plane preserves duplicate-index selection. image-rs 0.25.10 panics internally when asked for raw 1-bit or 4-bit index planes because its row buffer remains packed while the indexed-color branch copies `width` unpacked bytes. The first-party exact-byte laws continue to cover 1/4/8-bit indexed identity; the independent raw-index assertion is limited to image-rs's working 8-bit path rather than hiding the upstream panic.

## Neutral Semio Image Boundary

Semio image import no longer reads removed semantic BMP fields. It obtains a checked layout and RGBA8 projection from canonical bytes, records the actual source `bmp.profile`, `bmp.bitsPerPixel` and `bmp.rowOrder`, retains resolution metadata, and marks the resolved frame as 8-bit per channel. Indexed/direct source identity remains explicit metadata; the neutral resolved image does not pretend it can reconstruct palette indices, sample precision, padding, gaps or trailers.

Semio image export now calls one explicit `bmp_direct_rgb24_from_rgba8` constructor. It validates dimensions, checked byte counts, signed v3 header limits and DPI metadata, authors a bottom-up 40-byte BITMAPINFOHEADER plus aligned BGR rows, and reparses the produced bytes before returning a snapshot. Direct RGB24 cannot encode alpha, so every zero or partial alpha input is refused; only alpha 255 is admitted. `rgba8-direct-rgb24.json` is the language-neutral three-case policy fixture and `rgba8-opaque-direct-rgb24.bmp` is its hand-authored accepted byte result. The native law compares the constructor output byte-for-byte with that fixture, and image-rs independently reopens it as `[10,20,30,255]`.

## Browser Preview Boundary

The editor and viewer no longer pass raw BMP bytes to the browser or turn codec errors into an empty image. `bmp_png_preview` first validates the canonical layout, refuses an RGBA projection above a 64 MiB display ceiling, decodes an ephemeral RGBA8 projection, and encodes it with the first-party `semio-framework-pixels` PNG codec. Both windows render that PNG through ImageWindowKit. A projection refusal keeps the artifact mounted and renders ImageWindowKit's shared localized unavailable state; the native laws verify exact projected pixels and the German fallback text.

## Files in This Checkpoint

- BMP canonical IO and laws: `🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs`, its unit law, and the canonical byte/history fixtures.
- BMP independent oracle and routing: `🏅️standards/🔖️v3/🪆️subsets/✳️any/🔮️oracles/🦀️.rs`, `📦️packages/🦀️rust/📜️script.ts`, `📦️packages/🦀️rust/📋️project.json`, `📦️packages/🦀️rust/Cargo.toml`, and the BMP launch entry.
- BMP editor/viewer main windows, their unit laws, and locale-aware render callers.
- Semio image BMP import/export leaves and their unit laws.
- Exact byte snapshot schema, mutation/diff leaves, native/SQLite backing and editor command files mounted earlier in this cut.

## Honest Remaining BMP Surface

The exact byte authority, profile coverage, direct/indexed region paint, history, cancellation, save, SQLite carrier and browser preview boundaries are implemented and native-tested. Natural header, DPI, palette-entry and profile-conversion controls are not yet present as dedicated accessible editor controls. Those operations must use addressed byte patches with revision guards and exact inverses, and profile conversion must explicitly author a new representation rather than rebuilding an unrelated source on ordinary edits. Details/source access remains available, but it is not a substitute for those controls. No all-authoring-complete claim is made.
