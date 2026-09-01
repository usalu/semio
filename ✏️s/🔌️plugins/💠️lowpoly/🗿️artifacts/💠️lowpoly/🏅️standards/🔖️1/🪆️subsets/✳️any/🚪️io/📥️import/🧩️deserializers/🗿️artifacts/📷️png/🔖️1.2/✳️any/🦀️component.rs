//! lowpoly <- png
//!
//! Exact inverse of the export leaf: the lowpoly DSL text is read back out of the real `tEXt`
//! chunk (`engine::decode_png`, real PNG codec) the export leaf wrote, hex-decoded, and handed to
//! lowpoly's own `parse_dsl`. The raster pixels are informational only (see the export leaf's doc
//! comment) and are not consulted here.
use crate::artifacts::lowpoly::schema::snapshot::text::parse_dsl;
use crate::artifacts::lowpoly::schema::snapshot::{dec_str, LowpolySnapshot};
use semio_s_plugin_stdio::artifacts::png::engine::decode_png;
use semio_s_plugin_stdio::artifacts::png::PngSnapshot;

pub fn register() {}

pub fn deserialize(from: &PngSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let keyword = crate::artifacts::lowpoly::io::export::serializers::artifacts::png::v1_2::any::LOWPOLY_DSL_TEXT_KEYWORD;
    let hex = from
        .text_chunks
        .iter()
        .find(|c| c.keyword == keyword)
        .map(|c| c.value.as_str())
        .ok_or_else(|| store::TextError::new("png->lowpoly: missing embedded lowpoly DSL tEXt chunk", dsl::TextSpan::at(1, 1)))?;
    let text = dec_str(hex).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    parse_dsl(&text)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    let snap = decode_png(bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    deserialize(&snap)
}
