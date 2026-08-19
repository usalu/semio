//! ↩️ Inverse (undo) construction for the `replace-quality-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💎quality` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceQualityRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.quality.iter().find(|row| row.header.id == payload.quality_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceQualityRecord(super::mutation::ReplaceQualityRecord { quality_record: existing.clone() })],
        None => Vec::new(),
    }
}
