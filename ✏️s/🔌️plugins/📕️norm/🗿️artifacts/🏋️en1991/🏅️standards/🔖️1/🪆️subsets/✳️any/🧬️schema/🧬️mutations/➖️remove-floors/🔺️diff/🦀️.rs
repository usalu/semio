//! 🔺️ Diff for `remove-floors`.
use super::RemoveFloors;
use crate::artifact_schema::diff::En1991FloorsList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &RemoveFloors, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.floors.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    let mut values = base.floors.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1991Diff { floors: Some(En1991FloorsList { values }), ..Default::default() })
}
