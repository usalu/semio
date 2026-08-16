//! 🔺️ Sparse diff construction for the `delete-stakeholder` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👥stakeholders` per Wave C.

use super::mutation::DeleteStakeholder;
use crate::artifacts::program::diff::ProgramStakeholdersDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteStakeholder, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.stakeholders.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No stakeholder exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
