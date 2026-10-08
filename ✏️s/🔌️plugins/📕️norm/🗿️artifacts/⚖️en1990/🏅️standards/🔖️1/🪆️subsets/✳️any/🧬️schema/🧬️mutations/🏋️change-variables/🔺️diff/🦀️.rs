//! 🏋️ `change-variables` diff — replaces the whole collection: every base row is removed back to front, then every new row is inserted in order.

use super::ChangeVariables;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990VariableEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeVariables, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.variables == mutation.new_variables {
        return MutationOutcome::empty().warning("mutation.no-op", "variables already has this value.");
    }
    let removed = (0..base.variables.len()).rev().map(|index| En1990VariableEdit::remove(index, base.variables[index].id.clone()));
    let inserted = mutation.new_variables.iter().cloned().enumerate().map(|(index, row)| En1990VariableEdit::insert(index, row));
    MutationOutcome::new(En1990Diff { variables: removed.chain(inserted).collect(), ..En1990Diff::default() })
}
