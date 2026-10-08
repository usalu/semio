//! 🏋️ `change-variables` diff — replaces the whole collection: every base row leaves, every new row enters after the new row before it.

use super::ChangeVariables;
use crate::diff::{En1990Diff, En1990VariableAddition, En1990VariableDelta};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeVariables, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.variables == mutation.new_variables {
        return MutationOutcome::empty().warning("mutation.no-op", "variables already has this value.");
    }
    let removed = base.variables.iter().map(|row| row.id.clone()).collect();
    let added = mutation.new_variables.iter().enumerate().map(|(index, row)| En1990VariableAddition { after: index.checked_sub(1).map(|previous| mutation.new_variables[previous].id.clone()), row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { variables: En1990VariableDelta { removed, added, ..En1990VariableDelta::default() }, ..En1990Diff::default() })
}
