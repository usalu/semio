//! 🔺️ Sparse diff construction for the `create-sustainability-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♻️sustainability` per Wave C.

use super::CreateSustainabilityRequirement;
use crate::diff::ProgramSustainabilityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateSustainabilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.sustainability_requirement.header.id;
    if base.sustainability.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A sustainability requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.sustainability.len());
    if at > base.sustainability.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the sustainability requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { sustainability: Some(ProgramSustainabilityDelta::insertion(at, payload.sustainability_requirement.clone())), ..Default::default() })
}
