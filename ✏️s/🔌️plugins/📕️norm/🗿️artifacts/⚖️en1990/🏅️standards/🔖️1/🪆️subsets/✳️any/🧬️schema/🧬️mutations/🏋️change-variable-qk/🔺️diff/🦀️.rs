//! 🏋️ `change-variable-qk` diff: patches `qk` of the addressed variable action row.

use crate::change_variable_qk::ChangeVariableQk;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990VariableDelta, En1990VariablePatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeVariableQk, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_qk.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.variables.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Variable action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.qk == payload.new_qk {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The variable action already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { variables: En1990VariableDelta::modification(&payload.id, En1990VariablePatch { qk: Some(payload.new_qk), ..Default::default() }), ..Default::default() })
}
