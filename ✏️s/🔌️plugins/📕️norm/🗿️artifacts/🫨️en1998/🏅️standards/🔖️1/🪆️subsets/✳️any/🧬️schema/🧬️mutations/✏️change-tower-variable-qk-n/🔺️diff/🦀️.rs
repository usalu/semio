//! ✏️ `change-tower-variable-qk-n` diff: patches `qk_n` of the addressed variable action row.

use crate::change_tower_variable_qk_n::ChangeTowerVariableQkN;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TowerDelta, En1998TowerPatch, En1998VariableActionDelta, En1998VariableActionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTowerVariableQkN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_qk_n.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.towers.iter().find(|row| row.id == payload.tower_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tower {} does not exist.", payload.tower_id), [payload.tower_id.clone()]);
    };
    let Some(r1) = r0.variables.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("variable action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.qk_n == payload.new_qk_n {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The variable action already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::modification(&payload.tower_id, En1998TowerPatch { variables: En1998VariableActionDelta::modification(&payload.id, En1998VariableActionPatch { qk_n: Some(payload.new_qk_n), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
