//! ↩️ Inverse (undo) construction for the `delete-operational-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📋operations` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteOperationalRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.operations.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateOperationalRequirement(super::super::create_operational_requirement::CreateOperationalRequirement { operational_requirement: base.operations[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
