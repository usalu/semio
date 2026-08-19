//! ↩️ Inverse (undo) construction for the `create-activity` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏃activities` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateActivity, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteActivity(super::super::delete_activity::mutation::DeleteActivity { id: payload.activity.header.id.clone() })]
}
