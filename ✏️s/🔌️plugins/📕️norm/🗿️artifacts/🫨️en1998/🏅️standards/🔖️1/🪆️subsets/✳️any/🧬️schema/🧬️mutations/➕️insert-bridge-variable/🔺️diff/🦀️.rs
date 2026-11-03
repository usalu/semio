//! ➕️ `insert-bridge-variable` diff: inserts the variable action row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_bridge_variable::InsertBridgeVariable;
use semio_s_artifact_norm_en1998_diff::{En1998BridgeDelta, En1998BridgePatch, En1998Diff, En1998VariableActionDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertBridgeVariable, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.bridges.iter().find(|row| row.id == payload.bridge_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge {} does not exist.", payload.bridge_id), [payload.bridge_id.clone()]);
    };
    if r0.variables.iter().any(|existing| existing.id == payload.variable.id) {
        let key = payload.variable.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("variable action {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(r0.variables.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::modification(&payload.bridge_id, En1998BridgePatch { variables: En1998VariableActionDelta::insertion(index, payload.variable.clone()), ..Default::default() }), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the variable action list; inserted at {index}.", payload.index.unwrap_or(index)))
}
