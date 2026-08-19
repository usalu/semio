//! 🔺️ Sparse diff construction for the `rename-stakeholder` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👥stakeholders` per Wave C.

use super::mutation::RenameStakeholder;
use crate::artifacts::program::diff::{ProgramStakeholdersDelta, ProgramStakeholdersPatchEntry};
use crate::artifacts::program::registers::StakeholderPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameStakeholder, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.stakeholders.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No stakeholder exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This stakeholder already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = StakeholderPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { patched: vec![ProgramStakeholdersPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
