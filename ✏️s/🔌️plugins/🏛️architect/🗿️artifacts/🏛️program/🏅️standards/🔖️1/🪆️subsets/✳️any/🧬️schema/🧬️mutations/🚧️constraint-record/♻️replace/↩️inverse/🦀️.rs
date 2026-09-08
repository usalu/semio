//! ↩️ Inverse (undo) construction for the `replace-constraint-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🚧constraints` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceConstraintRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.constraints.iter().find(|row| row.header.id == payload.constraint_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceConstraintRecord(super::ReplaceConstraintRecord { constraint_record: existing.clone() })],
        None => Vec::new(),
    }
}
