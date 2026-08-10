//! iso16757 -> zip
use crate::artifacts::iso16757::Iso16757Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Iso16757Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Iso16757Snapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
