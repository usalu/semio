//! ↩️ Inverse (undo) construction for the `replace-storage-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗄️storage` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceStorageRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.storage.iter().find(|row| row.header.id == payload.storage_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceStorageRequirement(super::ReplaceStorageRequirement { storage_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
