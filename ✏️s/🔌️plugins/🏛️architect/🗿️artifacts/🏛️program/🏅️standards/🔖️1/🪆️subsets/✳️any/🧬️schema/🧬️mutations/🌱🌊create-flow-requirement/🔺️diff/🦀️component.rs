//! 🔺️ Sparse diff construction for the `create-flow-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌊flows` per Wave C.

use super::mutation::CreateFlowRequirement;
use crate::artifacts::program::diff::ProgramFlowsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateFlowRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.flow_requirement.header.id.clone();
    if base.flows.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A flow requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { flows: Some(ProgramFlowsDelta { added: vec![payload.flow_requirement.clone()], ..Default::default() }), ..Default::default() })
}
