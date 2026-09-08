//! ↩️ Inverse (undo) construction for the `rename-process` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔄processes` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::RenameProcess, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.processes.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameProcess(super::RenameProcess { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
