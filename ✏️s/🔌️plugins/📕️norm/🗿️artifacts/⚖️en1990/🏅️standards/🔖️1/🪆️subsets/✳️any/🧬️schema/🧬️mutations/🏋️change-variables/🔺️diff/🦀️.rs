//! 🏋️ `change-variables` diff — replaces the whole collection: every base row leaves from its base index, every new row enters at its index.

use super::ChangeVariables;
use crate::diff::{En1990Diff, En1990VariableDelta, En1990VariableInsertion, En1990VariableRemoval};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeVariables, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.variables == mutation.new_variables {
        return MutationOutcome::empty().warning("mutation.no-op", "variables already has this value.");
    }
    let removed = base.variables.iter().enumerate().map(|(index, row)| En1990VariableRemoval { id: row.id.clone(), index }).collect();
    let inserted = mutation.new_variables.iter().enumerate().map(|(index, row)| En1990VariableInsertion { index, row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { variables: En1990VariableDelta { removed, inserted, ..En1990VariableDelta::default() }, ..En1990Diff::default() })
}
