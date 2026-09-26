//! Diff for `remove-building`.
use super::RemoveBuilding;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &RemoveBuilding, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.buildings.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building #{}", payload.index), [payload.index.to_string()]);
    }
    let mut buildings = base.buildings.clone();
    buildings.remove(payload.index);
    protocol::MutationOutcome::new(En1998Diff { buildings: Some(buildings), ..Default::default() })
}
