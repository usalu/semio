//! ↩️ Inverse (undo) construction for the `delete-function` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚙️functions` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteFunction, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.functions.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateFunction(super::super::create_function::CreateFunction { function: base.functions[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
