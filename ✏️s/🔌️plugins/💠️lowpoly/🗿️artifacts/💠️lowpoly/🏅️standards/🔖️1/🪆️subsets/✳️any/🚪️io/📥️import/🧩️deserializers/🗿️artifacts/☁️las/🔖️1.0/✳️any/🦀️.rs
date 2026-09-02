//! lowpoly <- las
//!
//! 🐛️ See the export leaf's doc comment: honest stub, not a silent pack-envelope lie. Real LAS
//! import would need to synthesize a `LowpolyObject` (and a resolvable mesh child artifact) from
//! parsed points -- an out-of-scope architecture change, not a pure `&LasSnapshot -> …` mapping.
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::las::LasSnapshot;

pub fn register() {}

pub fn deserialize(_from: &LasSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    Err(store::TextError::new("las->lowpoly: importing real LAS point-cloud data into a lowpoly document needs mesh-child-artifact creation, not available at this layer -- not implemented", dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(_bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    Err(store::TextError::new("las->lowpoly: importing real LAS point-cloud data into a lowpoly document needs mesh-child-artifact creation, not available at this layer -- not implemented", dsl::TextSpan::at(1, 1)))
}
