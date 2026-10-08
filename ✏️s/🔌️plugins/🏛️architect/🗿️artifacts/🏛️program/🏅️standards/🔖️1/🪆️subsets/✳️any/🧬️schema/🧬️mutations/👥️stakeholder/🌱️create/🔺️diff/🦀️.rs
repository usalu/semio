//! 🔺️ Sparse diff construction for the `create-stakeholder` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👥stakeholders` per Wave C.

use super::CreateStakeholder;
use crate::diff::ProgramStakeholdersDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateStakeholder, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.stakeholder.header.id;
    if base.stakeholders.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A stakeholder already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.stakeholders.len());
    if at > base.stakeholders.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the stakeholder list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta::insertion(at, payload.stakeholder.clone())), ..Default::default() })
}
