//! lowpoly <- obj
//!
//! 🐛️ See the export leaf's doc comment for the pre-fix defect (`decode_pack::<LowpolySnapshot>`
//! against `ObjSnapshot::encode_pack` bytes always hit a pack-envelope mismatch) and why real mesh
//! geometry cannot be recovered here either (`LowpolyObject.mesh` is only a content-addressed
//! handle, never embedded geometry).
//!
//! Exact inverse of the export leaf: the lowpoly DSL text is read back out of the single
//! `unknown_statements` entry the export leaf wrote (real `engine::decode_obj`, never a second
//! bespoke grammar) and handed to lowpoly's own `parse_dsl`.
use crate::artifacts::lowpoly::schema::snapshot::text::parse_dsl;
use crate::artifacts::lowpoly::schema::snapshot::{dec_str, LowpolySnapshot};
use semio_s_plugin_stdio::artifacts::obj::engine::decode_obj;
use semio_s_plugin_stdio::artifacts::obj::ObjSnapshot;

pub fn register() {}

pub fn deserialize(from: &ObjSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let prefix = crate::artifacts::lowpoly::io::export::serializers::artifacts::obj::v3_0::any::LOWPOLY_DSL_COMMENT_PREFIX;
    let hex = from
        .unknown_statements
        .iter()
        .find_map(|u| u.raw.strip_prefix(prefix))
        .ok_or_else(|| store::TextError::new("obj->lowpoly: missing embedded lowpoly DSL comment", dsl::TextSpan::at(1, 1)))?;
    let text = dec_str(hex).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    parse_dsl(&text)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let snap = decode_obj(text).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    deserialize(&snap)
}
