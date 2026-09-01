//! lowpoly -> png
//!
//! 🐛️ See the obj export leaf's doc comment for the shared pre-fix defect class (pack-envelope
//! mismatch, always-erroring at runtime).
//!
//! Fix: unlike mesh geometry (never a field of `LowpolySnapshot`, see the obj leaf), a paint
//! layer's RGBA pixels ARE real inline data on `LowpolyObject.paint_layers[].pixels` -- so this
//! export is genuinely real, not a carrier trick: the first paint layer of the first object that
//! has one becomes the actual PNG raster (real `engine::encode_png`), each
//! `LOWPOLY_PAINT_TEXTURE_SIZE`-square. The full document still round-trips losslessly through a
//! real `tEXt` chunk (PNG's own real metadata mechanism, §11.3.4) carrying lowpoly's own DSL text
//! hex-encoded -- the same reuse principle as the txt/obj/ply leaves, never a second grammar. A
//! document with no paint layer anywhere falls back to a 1x1 opaque-white placeholder raster (PNG
//! requires a non-zero IHDR width/height) since the tEXt chunk alone still carries full fidelity.
use crate::artifacts::lowpoly::schema::snapshot::text::print_dsl;
use crate::artifacts::lowpoly::schema::snapshot::{enc_str, LowpolySnapshot};
use crate::artifacts::lowpoly::LOWPOLY_PAINT_TEXTURE_SIZE;
use semio_s_plugin_stdio::artifacts::png::engine::encode_png;
use semio_s_plugin_stdio::artifacts::png::schema::snapshot::{PngChunkMarker, PngTextChunk, PngTextKind};
use semio_s_plugin_stdio::artifacts::png::PngSnapshot;

pub(crate) const LOWPOLY_DSL_TEXT_KEYWORD: &str = "semio-lowpoly-dsl";

fn primary_paint_raster(snapshot: &LowpolySnapshot) -> (u32, u32, Vec<u8>) {
    let size = LOWPOLY_PAINT_TEXTURE_SIZE as u32;
    let expected_len = LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4;
    for object in &snapshot.objects {
        if let Some(layer) = object.paint_layers.first() {
            if layer.pixels.len() == expected_len {
                return (size, size, layer.pixels.clone());
            }
        }
    }
    (1, 1, vec![255, 255, 255, 255])
}

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<PngSnapshot, store::TextError> {
    let (width, height, pixels) = primary_paint_raster(snapshot);
    let hex = enc_str(&print_dsl(snapshot));
    let text_chunk = PngTextChunk { keyword: LOWPOLY_DSL_TEXT_KEYWORD.into(), value: hex, compressed: false, kind: PngTextKind::Text, language_tag: String::new(), translated_keyword: String::new() };
    Ok(PngSnapshot {
        width,
        height,
        pixels,
        text_chunks: vec![text_chunk],
        chunk_order: vec![PngChunkMarker::Ihdr, PngChunkMarker::Text { index: 0 }, PngChunkMarker::Idat, PngChunkMarker::Iend],
        ..Default::default()
    })
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    encode_png(&serialize(snapshot)?).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
