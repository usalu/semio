//! ↩️ Inverse (undo) construction for the `delete-quantity-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔢quantities` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteQuantityRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.quantities.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateQuantityRequirement(super::super::create_quantity_requirement::CreateQuantityRequirement { quantity_requirement: base.quantities[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
