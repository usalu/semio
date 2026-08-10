//! layout -> svg
use crate::artifacts::layout::LayoutSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &LayoutSnapshot) -> Result<Vec<u8>, String> {
    Ok(<LayoutSnapshot as store::DocumentDsl>::render_dsl(snapshot).into_bytes())
}
