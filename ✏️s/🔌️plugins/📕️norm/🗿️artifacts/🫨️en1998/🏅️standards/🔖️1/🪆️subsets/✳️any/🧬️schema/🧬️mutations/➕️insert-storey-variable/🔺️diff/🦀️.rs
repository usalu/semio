//! ➕️ `insert-storey-variable` diff: inserts the variable action row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_storey_variable::InsertStoreyVariable;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998StoreyDelta, En1998StoreyPatch, En1998VariableActionDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertStoreyVariable, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    let Some(r1) = r0.storeys.iter().find(|row| row.id == payload.storey_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("storey {} does not exist.", payload.storey_id), [payload.storey_id.clone()]);
    };
    if r1.variables.iter().any(|existing| existing.id == payload.variable.id) {
        let key = payload.variable.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("variable action {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(r1.variables.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { storeys: En1998StoreyDelta::modification(&payload.storey_id, En1998StoreyPatch { variables: En1998VariableActionDelta::insertion(index, payload.variable.clone()), ..Default::default() }), ..Default::default() }), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the variable action list; inserted at {index}.", payload.index.unwrap_or(index)))
}
