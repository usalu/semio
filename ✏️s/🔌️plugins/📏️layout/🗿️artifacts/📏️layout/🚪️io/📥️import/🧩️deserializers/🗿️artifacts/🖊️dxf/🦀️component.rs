//! layout <- import
use crate::artifacts::layout::LayoutSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<LayoutSnapshot, String> {
    let _ = bytes;
    let mut snap = LayoutSnapshot::default();
    snap.id = "import-📏️layout".into();
    snap.title = Some("Imported".into());
    Ok(snap)
}
