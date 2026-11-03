//! ✏️ `change-storey-variable-category` diff: patches `category` of the addressed variable action row.

use crate::change_storey_variable_category::ChangeStoreyVariableCategory;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998StoreyDelta, En1998StoreyPatch, En1998VariableActionDelta, En1998VariableActionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeStoreyVariableCategory, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    let Some(r1) = r0.storeys.iter().find(|row| row.id == payload.storey_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("storey {} does not exist.", payload.storey_id), [payload.storey_id.clone()]);
    };
    let Some(r2) = r1.variables.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("variable action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r2.category == payload.new_category {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The variable action already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { storeys: En1998StoreyDelta::modification(&payload.storey_id, En1998StoreyPatch { variables: En1998VariableActionDelta::modification(&payload.id, En1998VariableActionPatch { category: Some(payload.new_category), ..Default::default() }), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
