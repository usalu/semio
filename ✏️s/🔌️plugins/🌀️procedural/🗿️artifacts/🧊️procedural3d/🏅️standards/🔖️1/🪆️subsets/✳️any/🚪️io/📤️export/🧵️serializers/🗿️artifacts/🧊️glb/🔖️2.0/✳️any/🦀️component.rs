//! procedural3d -> glb
use crate::artifacts::procedural3d::Procedural3dSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Procedural3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Procedural3dSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
