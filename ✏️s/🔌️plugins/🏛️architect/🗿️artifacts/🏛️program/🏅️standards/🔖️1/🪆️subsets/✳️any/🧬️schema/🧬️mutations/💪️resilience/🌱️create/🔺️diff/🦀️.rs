//! 🔺️ Sparse diff construction for the `create-resilience-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💪resilience` per Wave C.

use super::CreateResilienceRequirement;
use crate::diff::ProgramResilienceDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateResilienceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.resilience_requirement.header.id;
    if base.resilience.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A resilience requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.resilience.len());
    if at > base.resilience.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the resilience requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { resilience: Some(ProgramResilienceDelta::insertion(at, payload.resilience_requirement.clone())), ..Default::default() })
}
