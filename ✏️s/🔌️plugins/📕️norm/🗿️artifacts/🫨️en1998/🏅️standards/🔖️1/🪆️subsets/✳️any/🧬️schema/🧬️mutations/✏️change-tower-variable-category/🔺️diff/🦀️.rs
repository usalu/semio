//! ✏️ `change-tower-variable-category` diff: patches `category` of the addressed variable action row.

use crate::change_tower_variable_category::ChangeTowerVariableCategory;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TowerDelta, En1998TowerPatch, En1998VariableActionDelta, En1998VariableActionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTowerVariableCategory, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.towers.iter().find(|row| row.id == payload.tower_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tower {} does not exist.", payload.tower_id), [payload.tower_id.clone()]);
    };
    let Some(r1) = r0.variables.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("variable action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.category == payload.new_category {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The variable action already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::modification(&payload.tower_id, En1998TowerPatch { variables: En1998VariableActionDelta::modification(&payload.id, En1998VariableActionPatch { category: Some(payload.new_category), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
