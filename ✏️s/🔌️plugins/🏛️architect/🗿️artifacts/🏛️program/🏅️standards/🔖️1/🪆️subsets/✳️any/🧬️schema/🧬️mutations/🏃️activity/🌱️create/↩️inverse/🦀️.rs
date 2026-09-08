//! ↩️ Inverse (undo) construction for the `create-activity` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏃activities` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateActivity, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteActivity(super::super::delete_activity::DeleteActivity { id: payload.activity.header.id.clone() })]
}
