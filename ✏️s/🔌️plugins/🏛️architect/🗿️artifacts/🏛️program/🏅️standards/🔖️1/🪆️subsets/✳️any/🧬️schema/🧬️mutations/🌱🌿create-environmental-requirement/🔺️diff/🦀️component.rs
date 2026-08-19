//! 🔺️ Sparse diff construction for the `create-environmental-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌿environmental` per Wave C.

use super::mutation::CreateEnvironmentalRequirement;
use crate::artifacts::program::diff::ProgramEnvironmentalDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateEnvironmentalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.environmental_requirement.header.id.clone();
    if base.environmental.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An environmental requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { added: vec![payload.environmental_requirement.clone()], ..Default::default() }), ..Default::default() })
}
