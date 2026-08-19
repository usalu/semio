//! ↩️ Inverse (undo) construction for the `replace-process` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔄processes` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceProcess, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.processes.iter().find(|row| row.header.id == payload.process.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceProcess(super::mutation::ReplaceProcess { process: existing.clone() })],
        None => Vec::new(),
    }
}
