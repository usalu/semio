//! 🔺️ Sparse diff construction for the `create-environmental-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌿environmental` per Wave C.

use super::CreateEnvironmentalRequirement;
use crate::diff::ProgramEnvironmentalDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateEnvironmentalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.environmental_requirement.header.id;
    if base.environmental.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An environmental requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.environmental.len());
    if at > base.environmental.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the environmental requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { environmental: Some(ProgramEnvironmentalDelta::insertion(at, payload.environmental_requirement.clone())), ..Default::default() })
}
