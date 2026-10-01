//! ↩️ Inverse (undo) construction for the `delete-flow-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🌊flows` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteFlowRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.flows.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateFlowRequirement(super::super::create_flow_requirement::CreateFlowRequirement { flow_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
