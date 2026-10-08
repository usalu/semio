//! ↩️ Inverse (undo) construction for the `delete-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📌requirements` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.requirements.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateRequirement(super::super::create_requirement::CreateRequirement { requirement: base.requirements[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
