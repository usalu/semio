//! lowpoly <- gltf
//!
//! 🐛️ See the export leaf's doc comment: honest stub, not a silent pack-envelope lie. Real glTF
//! import would need to synthesize `LowpolyObject`s (and resolvable mesh child artifacts) from
//! parsed nodes/meshes -- an out-of-scope architecture change, not a pure `&GltfSnapshot -> …`
//! mapping.
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::gltf::GltfSnapshot;

pub fn register() {}

pub fn deserialize(_from: &GltfSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    Err(store::TextError::new("gltf->lowpoly: importing real glTF geometry into a lowpoly document needs mesh-child-artifact creation, not available at this layer -- not implemented", dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(_bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    Err(store::TextError::new("gltf->lowpoly: importing real glTF geometry into a lowpoly document needs mesh-child-artifact creation, not available at this layer -- not implemented", dsl::TextSpan::at(1, 1)))
}
