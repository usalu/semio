//! ↩️ Inverse (undo) construction for the `delete-process` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔄processes` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteProcess, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.processes.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateProcess(super::super::create_process::CreateProcess { process: base.processes[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
