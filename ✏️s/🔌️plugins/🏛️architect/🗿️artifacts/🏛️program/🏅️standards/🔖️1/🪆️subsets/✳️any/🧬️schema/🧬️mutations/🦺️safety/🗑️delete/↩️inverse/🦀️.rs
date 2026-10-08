//! ↩️ Inverse (undo) construction for the `delete-safety-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🦺safety` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteSafetyRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.safety.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateSafetyRequirement(super::super::create_safety_requirement::CreateSafetyRequirement { safety_requirement: base.safety[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
