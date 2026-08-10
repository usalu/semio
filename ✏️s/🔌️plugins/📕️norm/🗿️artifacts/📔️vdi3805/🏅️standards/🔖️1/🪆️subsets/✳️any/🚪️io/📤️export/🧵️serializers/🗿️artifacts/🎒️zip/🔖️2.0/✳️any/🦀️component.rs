//! vdi3805 -> zip
use crate::artifacts::vdi3805::Vdi3805Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Vdi3805Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Vdi3805Snapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
