//! ✏️ `change-bridge-variable-qk-n` diff: patches `qk_n` of the addressed variable action row.

use crate::change_bridge_variable_qk_n::ChangeBridgeVariableQkN;
use semio_s_artifact_norm_en1998_diff::{En1998BridgeDelta, En1998BridgePatch, En1998Diff, En1998VariableActionDelta, En1998VariableActionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBridgeVariableQkN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_qk_n.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.bridges.iter().find(|row| row.id == payload.bridge_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge {} does not exist.", payload.bridge_id), [payload.bridge_id.clone()]);
    };
    let Some(r1) = r0.variables.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("variable action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.qk_n == payload.new_qk_n {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The variable action already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::modification(&payload.bridge_id, En1998BridgePatch { variables: En1998VariableActionDelta::modification(&payload.id, En1998VariableActionPatch { qk_n: Some(payload.new_qk_n), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
