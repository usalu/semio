//! ↩️ Inverse (undo) construction for the `rename-quality-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💎quality` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::RenameQualityRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.quality.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameQualityRecord(super::mutation::RenameQualityRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
