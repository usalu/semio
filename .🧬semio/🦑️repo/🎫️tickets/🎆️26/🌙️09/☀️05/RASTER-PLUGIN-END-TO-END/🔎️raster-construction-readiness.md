# Raster Construction Readiness

The declaration app Rust source census reports 238,303 bytes, 32 files, and 3,171 active lines for production; artifact-app-testing and its test configuration each report 238,303 bytes, 32 files, and 3,177 active lines. The limits are 262,144 bytes, 32 files, and 4,000 active lines. The file count is at its limit. This is a source census, not a compilation result.

The manifest media owner `🧰️framework/🔨️modules/🛂️manifest/🎞️media/🦀️.rs` defines `MediaClass::TwoD` and `MediaForm::Vector`; those variants exist.

The updated paired-refusal law in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🧪️tests/🦀️.rs` creates an `OriginalJobDecodeOwner`, issues `turn_with_decoder`, and retires it through the funded close loop. It tests pairing and custody with zero child progress; it does not establish reconciliation of nonzero child effects.

The current Raster editor `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎞️reserved-import/🦀️.rs` supplies the asset returned by `🚪️io/🧩️composite-and-media/🦀️.rs::raster_image_layer_and_asset` to `AddLayerAsset`. The former declares a `SemioImageSnapshot`, while the latter's field is `RasterImagePayload`. The owned conversion `RasterImagePayload::from_owned(image, grant)` quotes `owned_birth_demands()` and preserves the image on refusal. This is a source-level type mismatch pending compiler confirmation.

`PngDecodeJob::decoded_properties()` gives width and height as u32 and color type and bit depth as u8; obtain it before `take_result`. The latter yields an optional `(RasterImage, RetainedCloneProgress)`, and RasterImage owns width, height and RGBA pixel bytes.

`SemioImageSnapshot` contains schema String, width and height u32, colorspace SemioColorspace, bit_depth u8, frames Vec<SemioImageFrame>, optional ICC bytes and metadata entries. Each frame contains delay_ms u32 and rgba8 Vec<u8>; each metadata entry contains key and value Strings. Existing PNG semantics preserve original color type and bit depth, text metadata, and one RGBA frame; the incremental decoder does not itself provide text metadata. See 🔎️png-metadata-parity.md.

`RasterImagePayload::from_owned` returns `RetainedWorkOutcome<(Self, RetainedCloneProgress), (ValueError, SemioImageSnapshot)>`. `AddLayerAsset` holds asset_id String and asset RasterImagePayload. `CreateLayer` holds parent_id Option<String>, index usize and layer Box<RasterLayerNode>. The layer helper `create_pixel_layer` eagerly allocates its identifier and other strings. The asset helper exposes `raster_id_bytes(prefix)` and `write_raster_id(prefix, &mut String)` for preallocated identifier construction; the convenience `create_raster_id` allocates eagerly.
