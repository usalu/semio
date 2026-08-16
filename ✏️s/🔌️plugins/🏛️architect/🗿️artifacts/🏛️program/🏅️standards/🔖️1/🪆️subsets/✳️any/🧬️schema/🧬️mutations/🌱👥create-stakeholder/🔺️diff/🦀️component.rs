//! 🔺️ Sparse diff construction for the `create-stakeholder` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👥stakeholders` per Wave C.

use super::mutation::CreateStakeholder;
use crate::artifacts::program::diff::ProgramStakeholdersDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateStakeholder, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.stakeholder.header.id.clone();
    if base.stakeholders.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A stakeholder already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { added: vec![payload.stakeholder.clone()], ..Default::default() }), ..Default::default() })
}
