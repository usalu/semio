//! 🔺️ Sparse diff construction for the `delete-environmental-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌿environmental` per Wave C.

use super::mutation::DeleteEnvironmentalRequirement;
use crate::artifacts::program::diff::ProgramEnvironmentalDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteEnvironmentalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.environmental.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No environmental requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
