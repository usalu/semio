//! 🔺️ Sparse diff construction for the `create-safety-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🦺safety` per Wave C.

use super::CreateSafetyRequirement;
use crate::diff::ProgramSafetyDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateSafetyRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.safety_requirement.header.id;
    if base.safety.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A safety requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.safety.len());
    if at > base.safety.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the safety requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { safety: Some(ProgramSafetyDelta::insertion(at, payload.safety_requirement.clone())), ..Default::default() })
}
