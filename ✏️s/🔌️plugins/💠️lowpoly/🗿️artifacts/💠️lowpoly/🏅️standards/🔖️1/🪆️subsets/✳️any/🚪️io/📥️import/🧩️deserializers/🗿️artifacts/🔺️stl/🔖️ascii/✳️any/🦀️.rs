//! lowpoly <- stl
//!
//! 🐛️ See the export leaf's doc comment: honest stub, not a silent pack-envelope lie. Real STL
//! import would need to synthesize a `LowpolyObject` (and a resolvable mesh child artifact) from
//! parsed triangles -- an out-of-scope architecture change, not a pure `&StlSnapshot -> …` mapping.
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::stl::StlSnapshot;

pub fn register() {}

pub fn deserialize(_from: &StlSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    Err(store::TextError::new("stl->lowpoly: importing real STL geometry into a lowpoly document needs mesh-child-artifact creation, not available at this layer -- not implemented", dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(_bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    Err(store::TextError::new("stl->lowpoly: importing real STL geometry into a lowpoly document needs mesh-child-artifact creation, not available at this layer -- not implemented", dsl::TextSpan::at(1, 1)))
}
