//! forms -> zip
use crate::artifacts::forms::FormsSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &FormsSnapshot) -> Result<Vec<u8>, String> {
    Ok(<FormsSnapshot as store::DocumentDsl>::render_dsl(snapshot).into_bytes())
}
