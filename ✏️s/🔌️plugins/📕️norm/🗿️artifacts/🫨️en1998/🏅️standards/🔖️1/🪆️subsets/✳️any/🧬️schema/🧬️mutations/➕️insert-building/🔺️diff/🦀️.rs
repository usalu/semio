//! Diff for `insert-building`.
use super::InsertBuilding;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertBuilding, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.buildings.iter().any(|existing| existing.id == payload.building.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Building id {} already exists.", payload.building.id), [payload.building.id.clone()]);
    }
    let mut buildings = base.buildings.clone();
    let index = payload.index.min(buildings.len());
    buildings.insert(index, payload.building.clone());
    protocol::MutationOutcome::new(En1998Diff { buildings: Some(buildings), ..Default::default() })
}
