//! ↩️ Inverse (undo) construction for the `delete-option-evaluation` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚖️options` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteOptionEvaluation, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.options.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateOptionEvaluation(super::super::create_option_evaluation::CreateOptionEvaluation { option_evaluation: base.options[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
