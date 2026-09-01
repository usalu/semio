//! lowpoly <- ply
//!
//! Exact inverse of the export leaf: the lowpoly DSL text is read back out of the single
//! `comments` entry the export leaf wrote (real `engine::decode_ply`, never a second bespoke
//! grammar) and handed to lowpoly's own `parse_dsl`.
use crate::artifacts::lowpoly::schema::snapshot::text::parse_dsl;
use crate::artifacts::lowpoly::schema::snapshot::{dec_str, LowpolySnapshot};
use semio_s_plugin_stdio::artifacts::ply::engine::decode_ply;
use semio_s_plugin_stdio::artifacts::ply::PlySnapshot;

pub fn register() {}

pub fn deserialize(from: &PlySnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let prefix = crate::artifacts::lowpoly::io::export::serializers::artifacts::ply::v1_0::any::LOWPOLY_DSL_COMMENT_PREFIX;
    let hex = from
        .comments
        .iter()
        .find_map(|c| c.strip_prefix(prefix))
        .ok_or_else(|| store::TextError::new("ply->lowpoly: missing embedded lowpoly DSL comment", dsl::TextSpan::at(1, 1)))?;
    let text = dec_str(hex).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    parse_dsl(&text)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    let snap = decode_ply(bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    deserialize(&snap)
}
