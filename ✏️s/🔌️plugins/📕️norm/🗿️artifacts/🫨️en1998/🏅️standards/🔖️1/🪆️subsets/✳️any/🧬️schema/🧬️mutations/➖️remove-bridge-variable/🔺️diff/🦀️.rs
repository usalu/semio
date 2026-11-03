//! ➖️ `remove-bridge-variable` diff: removes the addressed variable action row.

use crate::remove_bridge_variable::RemoveBridgeVariable;
use semio_s_artifact_norm_en1998_diff::{En1998BridgeDelta, En1998BridgePatch, En1998Diff, En1998VariableActionDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveBridgeVariable, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.bridges.iter().find(|row| row.id == payload.bridge_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge {} does not exist.", payload.bridge_id), [payload.bridge_id.clone()]);
    };
    let Some(index) = r0.variables.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("variable action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::modification(&payload.bridge_id, En1998BridgePatch { variables: En1998VariableActionDelta::removal(&r0.variables, index), ..Default::default() }), ..Default::default() })
}
