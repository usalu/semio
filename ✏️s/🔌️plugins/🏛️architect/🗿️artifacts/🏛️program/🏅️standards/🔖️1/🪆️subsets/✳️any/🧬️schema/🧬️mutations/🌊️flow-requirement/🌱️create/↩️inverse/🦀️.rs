//! ↩️ Inverse (undo) construction for the `create-flow-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🌊flows` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateFlowRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteFlowRequirement(super::super::delete_flow_requirement::DeleteFlowRequirement { id: payload.flow_requirement.header.id.clone() })]
}
