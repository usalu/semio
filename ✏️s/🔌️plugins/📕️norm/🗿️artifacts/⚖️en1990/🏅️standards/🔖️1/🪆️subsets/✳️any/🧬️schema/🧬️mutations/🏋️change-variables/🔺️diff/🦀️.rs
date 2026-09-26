//! 🔺️ `change-variables` sparse diff.

use super::ChangeVariables;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeVariables, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.variables == &mutation.new_variables {
        return MutationOutcome::empty().warn("mutation.no-op", "variables already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        variables: Some(mutation.new_variables.clone()),
        ..En1990Diff::default()
    })
}
