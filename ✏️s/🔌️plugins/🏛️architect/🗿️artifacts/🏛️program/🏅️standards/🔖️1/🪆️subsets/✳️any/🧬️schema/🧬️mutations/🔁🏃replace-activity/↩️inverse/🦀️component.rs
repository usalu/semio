//! ↩️ Inverse (undo) construction for the `replace-activity` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏃activities` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceActivity, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.activities.iter().find(|row| row.header.id == payload.activity.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceActivity(super::mutation::ReplaceActivity { activity: existing.clone() })],
        None => Vec::new(),
    }
}
