//! 🔺️ Diff for `remove-floors`.
use super::RemoveFloors;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991FloorDelta};
pub fn diff(payload: &RemoveFloors, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.floors.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1991Diff { floors: En1991FloorDelta::removal(&base.floors[payload.index].id), ..Default::default() })
}
