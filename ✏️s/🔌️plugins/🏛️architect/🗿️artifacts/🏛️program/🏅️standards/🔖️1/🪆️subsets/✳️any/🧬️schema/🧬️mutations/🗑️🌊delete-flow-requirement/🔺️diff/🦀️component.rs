//! 🔺️ Sparse diff construction for the `delete-flow-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌊flows` per Wave C.

use super::mutation::DeleteFlowRequirement;
use crate::artifacts::program::diff::ProgramFlowsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteFlowRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.flows.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No flow requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { flows: Some(ProgramFlowsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
