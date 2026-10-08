//! ↩️ Inverse (undo) construction for the `delete-stakeholder` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `👥stakeholders` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteStakeholder, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.stakeholders.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateStakeholder(super::super::create_stakeholder::CreateStakeholder { stakeholder: base.stakeholders[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
