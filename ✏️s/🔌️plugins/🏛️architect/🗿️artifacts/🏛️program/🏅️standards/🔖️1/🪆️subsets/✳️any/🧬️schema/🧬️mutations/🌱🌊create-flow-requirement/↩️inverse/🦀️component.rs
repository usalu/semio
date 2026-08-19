//! ↩️ Inverse (undo) construction for the `create-flow-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🌊flows` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateFlowRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteFlowRequirement(super::super::delete_flow_requirement::mutation::DeleteFlowRequirement { id: payload.flow_requirement.header.id.clone() })]
}
