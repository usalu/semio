//! ✏️ `change-bridge-variable-category` diff: patches `category` of the addressed variable action row.

use crate::change_bridge_variable_category::ChangeBridgeVariableCategory;
use semio_s_artifact_norm_en1998_diff::{En1998BridgeDelta, En1998BridgePatch, En1998Diff, En1998VariableActionDelta, En1998VariableActionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBridgeVariableCategory, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.bridges.iter().find(|row| row.id == payload.bridge_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge {} does not exist.", payload.bridge_id), [payload.bridge_id.clone()]);
    };
    let Some(r1) = r0.variables.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("variable action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.category == payload.new_category {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The variable action already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::modification(&payload.bridge_id, En1998BridgePatch { variables: En1998VariableActionDelta::modification(&payload.id, En1998VariableActionPatch { category: Some(payload.new_category), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
