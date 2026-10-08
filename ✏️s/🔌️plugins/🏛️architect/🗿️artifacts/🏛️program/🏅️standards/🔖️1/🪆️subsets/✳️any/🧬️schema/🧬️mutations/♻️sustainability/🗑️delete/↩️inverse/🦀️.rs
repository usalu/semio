//! ↩️ Inverse (undo) construction for the `delete-sustainability-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `♻️sustainability` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteSustainabilityRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.sustainability.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateSustainabilityRequirement(super::super::create_sustainability_requirement::CreateSustainabilityRequirement { sustainability_requirement: base.sustainability[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
