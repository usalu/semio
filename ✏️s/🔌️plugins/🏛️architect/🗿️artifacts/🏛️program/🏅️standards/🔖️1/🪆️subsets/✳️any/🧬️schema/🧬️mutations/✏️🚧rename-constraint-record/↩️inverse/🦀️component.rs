//! ↩️ Inverse (undo) construction for the `rename-constraint-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🚧constraints` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::RenameConstraintRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.constraints.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameConstraintRecord(super::mutation::RenameConstraintRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
