//! ➕️ `insert-building` diff: inserts the building row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_building::InsertBuilding;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertBuilding, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.buildings.iter().any(|existing| existing.id == payload.building.id) {
        let key = payload.building.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("building {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.buildings.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::insertion(index, payload.building.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the building list; inserted at {index}.", payload.index.unwrap_or(index)))
}
