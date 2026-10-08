//! 🔺️ Sparse diff construction for the `create-wayfinding-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧭wayfinding` per Wave C.

use super::CreateWayfindingRequirement;
use crate::diff::ProgramWayfindingDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateWayfindingRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.wayfinding_requirement.header.id;
    if base.wayfinding.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A wayfinding requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.wayfinding.len());
    if at > base.wayfinding.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the wayfinding requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { wayfinding: Some(ProgramWayfindingDelta::insertion(at, payload.wayfinding_requirement.clone())), ..Default::default() })
}
