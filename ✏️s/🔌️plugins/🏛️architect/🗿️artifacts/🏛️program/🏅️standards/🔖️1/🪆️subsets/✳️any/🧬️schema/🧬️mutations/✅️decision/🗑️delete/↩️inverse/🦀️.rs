//! ↩️ Inverse (undo) construction for the `delete-decision` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `✅decisions` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteDecision, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.decisions.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateDecision(super::super::create_decision::CreateDecision { decision: base.decisions[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
