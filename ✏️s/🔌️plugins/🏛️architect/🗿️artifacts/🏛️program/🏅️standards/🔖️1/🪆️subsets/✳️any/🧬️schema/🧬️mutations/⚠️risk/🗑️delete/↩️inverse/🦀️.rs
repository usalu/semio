//! ↩️ Inverse (undo) construction for the `delete-risk` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚠️risks` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteRisk, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.risks.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateRisk(super::super::create_risk::CreateRisk { risk: base.risks[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
