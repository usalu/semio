//! ↩️ Inverse (undo) construction for the `delete-wayfinding-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧭wayfinding` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteWayfindingRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.wayfinding.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateWayfindingRequirement(super::super::create_wayfinding_requirement::CreateWayfindingRequirement { wayfinding_requirement: base.wayfinding[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
