//! ↩️ Inverse (undo) construction for the `create-change-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📝changes` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateChangeRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteChangeRecord(super::super::delete_change_record::mutation::DeleteChangeRecord { id: payload.change_record.header.id.clone() })]
}
