//! ➕️ `insert-system` diff: inserts the resisting system row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_system::InsertSystem;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998SystemDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertSystem, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    if r0.systems.iter().any(|existing| existing.id == payload.system.id) {
        let key = payload.system.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("resisting system {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(r0.systems.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { systems: En1998SystemDelta::insertion(index, payload.system.clone()), ..Default::default() }), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the resisting system list; inserted at {index}.", payload.index.unwrap_or(index)))
}
