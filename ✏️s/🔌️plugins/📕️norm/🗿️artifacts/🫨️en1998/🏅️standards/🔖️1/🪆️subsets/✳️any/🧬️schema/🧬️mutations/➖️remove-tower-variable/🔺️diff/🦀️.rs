//! ➖️ `remove-tower-variable` diff: removes the addressed variable action row.

use crate::remove_tower_variable::RemoveTowerVariable;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TowerDelta, En1998TowerPatch, En1998VariableActionDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveTowerVariable, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.towers.iter().find(|row| row.id == payload.tower_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tower {} does not exist.", payload.tower_id), [payload.tower_id.clone()]);
    };
    let Some(index) = r0.variables.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("variable action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::modification(&payload.tower_id, En1998TowerPatch { variables: En1998VariableActionDelta::removal(&r0.variables, index), ..Default::default() }), ..Default::default() })
}
