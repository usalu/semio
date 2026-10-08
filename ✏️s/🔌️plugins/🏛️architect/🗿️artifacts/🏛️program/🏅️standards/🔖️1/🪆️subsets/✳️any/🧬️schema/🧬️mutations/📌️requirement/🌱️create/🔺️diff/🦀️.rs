//! 🔺️ Sparse diff construction for the `create-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📌requirements` per Wave C.

use super::CreateRequirement;
use crate::diff::ProgramRequirementsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.requirement.header.id;
    if base.requirements.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.requirements.len());
    if at > base.requirements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { requirements: Some(ProgramRequirementsDelta::insertion(at, payload.requirement.clone())), ..Default::default() })
}
