//! 🏋️ `change-variable-category` diff: patches `category` of the addressed variable action row.

use crate::change_variable_category::ChangeVariableCategory;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990VariableDelta, En1990VariablePatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeVariableCategory, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let Some(r0) = base.variables.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Variable action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.category == payload.new_category {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The variable action already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { variables: En1990VariableDelta::modification(&payload.id, En1990VariablePatch { category: Some(payload.new_category.clone()), ..Default::default() }), ..Default::default() })
}
