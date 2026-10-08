//! 🔺️ Sparse diff construction for the `create-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::CreateFlexibilityRequirement;
use crate::diff::ProgramFlexibilityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateFlexibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.flexibility_requirement.header.id;
    if base.flexibility.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A flexibility requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.flexibility.len());
    if at > base.flexibility.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the flexibility requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { flexibility: Some(ProgramFlexibilityDelta::insertion(at, payload.flexibility_requirement.clone())), ..Default::default() })
}
