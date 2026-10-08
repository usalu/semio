//! ↩️ Inverse (undo) construction for the `delete-constraint-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🚧constraints` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteConstraintRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.constraints.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateConstraintRecord(super::super::create_constraint_record::CreateConstraintRecord { constraint_record: base.constraints[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
