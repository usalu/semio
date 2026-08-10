//! forms <- zip
use crate::artifacts::forms::engine::{empty_forms_snapshot, create_form_id};
use crate::artifacts::forms::FormsSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<FormsSnapshot, String> {
    let _ = bytes;
    let mut snap = empty_forms_snapshot();
    snap.id = create_form_id("zip-import", b"zip");
    snap.title = Some(format!("Imported zip"));
    Ok(snap)
}
