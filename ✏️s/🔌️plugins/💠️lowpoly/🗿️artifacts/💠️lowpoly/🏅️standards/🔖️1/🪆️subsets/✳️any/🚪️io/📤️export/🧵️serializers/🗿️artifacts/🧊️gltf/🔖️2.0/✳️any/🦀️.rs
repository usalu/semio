//! lowpoly -> gltf
//!
//! 🐛️ Same pre-fix pack-envelope-mismatch defect class as the stl leaf (see that leaf's doc
//! comment for the shared root cause) -- always errored at runtime despite compiling and looking
//! real. `GltfSnapshot` is a full typed glTF 2.0 document (nodes/meshes/accessors/scenes must be
//! internally consistent); synthesizing a valid one needs real mesh geometry, unavailable at the
//! `&LowpolySnapshot -> …` layer (see the stl leaf's doc comment for why). Left as an HONEST stub
//! pending that architecture work -- see this ticket's `📝️io-implementation-result.md` handoff.
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::gltf::GltfSnapshot;

pub fn register() {}

pub fn serialize(_snapshot: &LowpolySnapshot) -> Result<GltfSnapshot, store::TextError> {
    Err(store::TextError::new("lowpoly->gltf: real mesh geometry is unavailable at the LowpolySnapshot layer (mesh is a content-addressed handle, not embedded geometry) -- not implemented", dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    serialize(snapshot).map(|_| Vec::new())
}
