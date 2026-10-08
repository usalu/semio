//! ➕️ `insert-building` diff — inserts the row at its position, clamped to the end of the collection; an id the document already holds is a `mutation.duplicate-id`.

use super::InsertBuilding;
use crate::diff::{En1998Diff, En1998BuildingDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &InsertBuilding, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.buildings.iter().any(|existing| existing.id == payload.building.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Building id {} already exists.", payload.building.id), [payload.building.id.clone()]);
    }
    let index = payload.index.min(base.buildings.len());
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::insertion(&base.buildings, index, payload.building.clone()), ..Default::default() })
}
