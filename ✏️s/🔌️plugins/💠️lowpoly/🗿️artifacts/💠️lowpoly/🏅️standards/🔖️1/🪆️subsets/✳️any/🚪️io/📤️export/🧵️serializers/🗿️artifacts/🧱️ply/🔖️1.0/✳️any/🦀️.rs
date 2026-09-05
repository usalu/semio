//! lowpoly -> ply
//!
//! 🐛️ See the obj export leaf's doc comment for the shared pre-fix defect class (pack-envelope
//! mismatch, always-erroring at runtime) and why real mesh geometry cannot be produced here either
//! (`LowpolyObject.mesh` is only a content-addressed handle, never embedded geometry).
//!
//! Fix: reuse lowpoly's own canonical `.lowpoly` DSL text as a real, valid, geometry-empty PLY
//! document -- hex-encoded into ONE `comment` line (PLY's real position-retained comment slot, see
//! `PlySnapshot.comments`' doc comment), so the bytes really are valid PLY text
//! (`engine::encode_ply`/`decode_ply`, never a second bespoke grammar) that also carries the full
//! lowpoly document losslessly.
use crate::artifacts::lowpoly::schema::snapshot::text::print_dsl;
use crate::artifacts::lowpoly::schema::snapshot::{enc_str, LowpolySnapshot};
use semio_s_plugin_stdio::artifacts::ply::engine::encode_ply;
use semio_s_plugin_stdio::artifacts::ply::PlySnapshot;

pub(crate) const LOWPOLY_DSL_COMMENT_PREFIX: &str = "semio-lowpoly-dsl ";

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<PlySnapshot, store::TextError> {
    let hex = enc_str(&print_dsl(snapshot));
    Ok(PlySnapshot { comments: vec![format!("{LOWPOLY_DSL_COMMENT_PREFIX}{hex}")], ..Default::default() })
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    encode_ply(&serialize(snapshot)?).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
