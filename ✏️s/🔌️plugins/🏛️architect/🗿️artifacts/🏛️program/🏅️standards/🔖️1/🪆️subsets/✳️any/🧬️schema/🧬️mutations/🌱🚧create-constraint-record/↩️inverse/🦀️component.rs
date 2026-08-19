//! ↩️ Inverse (undo) construction for the `create-constraint-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🚧constraints` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateConstraintRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteConstraintRecord(super::super::delete_constraint_record::mutation::DeleteConstraintRecord { id: payload.constraint_record.header.id.clone() })]
}
