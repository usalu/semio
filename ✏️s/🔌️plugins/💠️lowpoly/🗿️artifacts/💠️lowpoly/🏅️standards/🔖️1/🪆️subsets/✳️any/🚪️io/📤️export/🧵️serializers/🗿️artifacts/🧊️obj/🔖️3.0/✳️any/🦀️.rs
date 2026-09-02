//! lowpoly -> obj
//!
//! 🐛️ Pre-fix content round-tripped `LowpolySnapshot::encode_pack` bytes (envelope id
//! `lowpoly.lowpoly`) straight into `ObjSnapshot::decode_pack`, which unconditionally rejects any
//! envelope id other than its own `stdio.obj` (see that type's `decode_pack_with`) -- this always
//! threw `PackError::Schema("pack envelope mismatch: ...")` at runtime despite compiling and
//! looking real. Root cause: `LowpolyObject.mesh` is only a content-addressed
//! `store::ArtifactChild<SemioMeshSnapshot>` HANDLE (see that field's doc comment) -- the live
//! half-edge mesh geometry is not a field of `LowpolySnapshot` at all, so no synchronous function
//! of `&LowpolySnapshot` alone can ever produce real OBJ vertices/faces.
//!
//! Fix: reuse lowpoly's own canonical `.lowpoly` DSL text (same codec TXT export reuses) as a
//! real, valid, geometry-empty OBJ document -- hex-encoded into ONE `unknown_statements` entry
//! (OBJ's real per-line "nothing on disk silently dropped" retention slot, see
//! `ObjUnknownStatement`'s doc comment), so the bytes really are valid Wavefront OBJ text
//! (`engine::encode_obj`/`decode_obj`, never a second bespoke grammar) that also carries the full
//! lowpoly document losslessly. Geometry-empty is honest, not silent: real mesh export needs an
//! out-of-scope architecture change (resolving the mesh child artifact through a store/session
//! handle no `serialize`/`serialize_bytes` signature here receives).
use crate::artifacts::lowpoly::schema::snapshot::text::print_dsl;
use crate::artifacts::lowpoly::schema::snapshot::{enc_str, LowpolySnapshot};
use semio_s_plugin_stdio::artifacts::obj::engine::encode_obj;
use semio_s_plugin_stdio::artifacts::obj::schema::snapshot::ObjUnknownStatement;
use semio_s_plugin_stdio::artifacts::obj::ObjSnapshot;

pub(crate) const LOWPOLY_DSL_COMMENT_PREFIX: &str = "# semio-lowpoly-dsl ";

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<ObjSnapshot, store::TextError> {
    let hex = enc_str(&print_dsl(snapshot));
    Ok(ObjSnapshot { unknown_statements: vec![ObjUnknownStatement { line_index: 0, raw: format!("{LOWPOLY_DSL_COMMENT_PREFIX}{hex}") }], ..Default::default() })
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(encode_obj(&serialize(snapshot)?).into_bytes())
}
