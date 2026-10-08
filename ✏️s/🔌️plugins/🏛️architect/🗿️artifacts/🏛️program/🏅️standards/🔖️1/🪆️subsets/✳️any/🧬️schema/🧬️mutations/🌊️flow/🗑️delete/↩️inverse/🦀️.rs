//! ↩️ Inverse (undo) construction for the `delete-flow-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🌊flows` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteFlowRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.flows.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateFlowRequirement(super::super::create_flow_requirement::CreateFlowRequirement { flow_requirement: base.flows[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
