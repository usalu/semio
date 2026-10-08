//! ↩️ Inverse (undo) construction for the `delete-regulatory-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📜regulatory` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteRegulatoryRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.regulatory.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateRegulatoryRequirement(super::super::create_regulatory_requirement::CreateRegulatoryRequirement { regulatory_requirement: base.regulatory[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
