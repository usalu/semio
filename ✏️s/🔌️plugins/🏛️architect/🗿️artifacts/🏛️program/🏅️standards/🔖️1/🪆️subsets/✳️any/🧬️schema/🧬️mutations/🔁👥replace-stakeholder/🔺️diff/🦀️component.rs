//! 🔺️ Sparse diff construction for the `replace-stakeholder` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👥stakeholders` per Wave C.

use super::mutation::ReplaceStakeholder;
use crate::artifacts::program::diff::{ProgramStakeholdersDelta, ProgramStakeholdersPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub fn diff(payload: &ReplaceStakeholder, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.stakeholders.iter().find(|row| row.header.id == payload.stakeholder.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No stakeholder exists with this id.", [payload.stakeholder.header.id.0.clone()]);
    };
    if existing == &payload.stakeholder {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This stakeholder already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.stakeholder).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { patched: vec![ProgramStakeholdersPatchEntry { id: payload.stakeholder.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
