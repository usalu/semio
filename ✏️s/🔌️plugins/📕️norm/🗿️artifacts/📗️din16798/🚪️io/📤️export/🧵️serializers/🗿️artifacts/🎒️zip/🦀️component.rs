//! din16798 -> zip
use crate::artifacts::din16798::Din16798Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Din16798Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Din16798Snapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
