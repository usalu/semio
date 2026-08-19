//! ↩️ Inverse (undo) construction for the `rename-activity` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏃activities` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::RenameActivity, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.activities.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameActivity(super::mutation::RenameActivity { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
