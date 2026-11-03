//! 🏋️ `change-variable-group` diff: patches `group` of the addressed variable action row.

use crate::change_variable_group::ChangeVariableGroup;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990VariableDelta, En1990VariablePatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeVariableGroup, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let Some(r0) = base.variables.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Variable action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.group == payload.new_group {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The variable action already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { variables: En1990VariableDelta::modification(&payload.id, En1990VariablePatch { group: Some(payload.new_group.clone()), ..Default::default() }), ..Default::default() })
}
