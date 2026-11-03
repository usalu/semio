//! ➖️ `remove-storey-variable` diff: removes the addressed variable action row.

use crate::remove_storey_variable::RemoveStoreyVariable;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998StoreyDelta, En1998StoreyPatch, En1998VariableActionDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveStoreyVariable, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    let Some(r1) = r0.storeys.iter().find(|row| row.id == payload.storey_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("storey {} does not exist.", payload.storey_id), [payload.storey_id.clone()]);
    };
    let Some(index) = r1.variables.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("variable action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { storeys: En1998StoreyDelta::modification(&payload.storey_id, En1998StoreyPatch { variables: En1998VariableActionDelta::removal(&r1.variables, index), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
