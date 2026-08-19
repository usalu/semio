//! ↩️ Inverse (undo) construction for the `delete-storage-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗄️storage` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteStorageRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.storage.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateStorageRequirement(super::super::create_storage_requirement::mutation::CreateStorageRequirement { storage_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
