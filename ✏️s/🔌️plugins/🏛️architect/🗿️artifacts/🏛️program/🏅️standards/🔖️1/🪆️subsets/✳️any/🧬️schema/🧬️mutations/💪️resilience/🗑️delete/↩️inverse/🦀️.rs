//! ↩️ Inverse (undo) construction for the `delete-resilience-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💪resilience` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteResilienceRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.resilience.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateResilienceRequirement(super::super::create_resilience_requirement::CreateResilienceRequirement { resilience_requirement: base.resilience[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
