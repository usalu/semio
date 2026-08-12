//! ↩️ Inverse (undo) construction for the `delete-constraint-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🚧constraints` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteConstraintRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.constraints.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateConstraintRecord(super::super::create_constraint_record::mutation::CreateConstraintRecord { constraint_record: existing.clone() })],
        None => Vec::new(),
    }
}
