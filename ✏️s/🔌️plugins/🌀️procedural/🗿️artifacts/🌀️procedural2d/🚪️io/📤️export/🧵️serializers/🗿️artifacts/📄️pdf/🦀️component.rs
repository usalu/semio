//! procedural2d -> pdf
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Procedural2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Procedural2dSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
