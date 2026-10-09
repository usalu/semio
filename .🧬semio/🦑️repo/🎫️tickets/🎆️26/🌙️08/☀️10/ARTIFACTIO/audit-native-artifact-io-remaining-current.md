# Current Native Artifact I/O Remaining Audit

Read-only source audit on 2026-10-08. No source edits, git operations, runtime claims, or native test claims. Broad Rust semantic-schema scan followed by verified call paths; not a proof of universal absence. Reports cover current source independently of earlier fixes.

## Confirmed Remaining Draw Image Boundary

Base: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

- `🧬️schema/🦀️.rs:1138` `decode_drawing_image_asset_luma` reads serialized image source from `DrawingImageAsset.data`, strips `data:` framing (1140), calls `base64_standard_decode` (1144), then `semio_framework_pixels::decode_png` (1145). `resolve_trace_layer_segments` invokes it at1188. This is actual PNG physical decoding during semantic trace resolution, not ToValue/FromValue or diagnostic rendering.
- `🧬️schema/🎬️scene/🔍️trace/🦀️.rs:50` creates `ImageDecodeJob` from encoded `RasterSceneAsset.data`; `🧬️schema/🎬️scene/📷️raster/🦀️.rs:131` does the same. Framework callee `🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🦀️.rs:24` creates `BinarySourceJob`; at31, decoded physical bytes feed `PngDecodeJob`. Thus the cooperative jobs still embed physical decoders in semantic schema jobs.
- `🗿️artifacts/🖍️drawing/🦀️.rs:140` `DrawingImageAsset` publicly retains MIME + UTF8 encoded `data`. Snapshot assets at `🧬️schema/📸️snapshot/🦀️.rs:33` and `DrawingAssetsDelta` at `🧬️schema/🔺️diff/🦀️.rs:43` preserve this physical representation as semantic state. `🧬️schema/🎬️scene/📋️prepare/🦀️.rs:117` copies the encoded source into raster plan state.
- `🧬️schema/🦀️.rs:626` and `🧬️schema/🎬️scene/👁️view/🦀️.rs:25` construct `data:{mime};base64,{data}` physical URLs in scene projection.

Extraction target: artifact `🚪️io` owns MIME/source framing, base64, and PNG admission plus physical scene/export source encoding. The semantic image asset should carry intrinsic image extent/sample state or an admitted semantic asset identity; trace/raster jobs consume already admitted samples. Preserve cooperative budget/cancellation at the admission job and at image processing stages. Moving only the helper to io while schema still directly invokes it leaves the dependency direction wrong.

## Bounded Retained-Physical-State Follow-Up

MP3 is a concrete serialized-state design remaining in semantic schema, though its present documentation explicitly defines a container-level artifact rather than decoded audio. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:38` retains whole 128-byte `Id3v1Tag.raw`, and `Mp3Frame.payload` at72 retains encoded MPEG payload. Physical `🚪️io/🦀️.rs:323` writes `tag.raw` verbatim; at320 writes compressed frame payload verbatim. ID3v1 has semantic fields that can be decoded, so its whole-trailer snapshot representation is stronger evidence than compressed audio payload. Extraction target is typed title/artist/album/year/comment/genre schema plus io fixed-width encoding. A compressed elementary stream may be an explicit domain payload boundary; do not mechanically equate every Vec<u8> with a breach.

Broad scans also found explicitly opaque BCF attachments, WAV raw chunks, MP4 NAL units/configuration, AVI stream extras, BMP gaps/trailers, and glTF buffer words. These require artifact-specific ownership decisions and were not promoted to automatic findings. Raw samples, hashing to_le_bytes, numeric lexeme domain parsing, pure DslValue conversion, cfg(test) fixture parsing, and causal assignment-index encode methods were excluded.

No additional confirmed direct text/binary parser call beyond Draw was found in the inspected schema paths. Existing fixed families were not reported as open findings. This audit does not claim that all artifact paths are clean.
