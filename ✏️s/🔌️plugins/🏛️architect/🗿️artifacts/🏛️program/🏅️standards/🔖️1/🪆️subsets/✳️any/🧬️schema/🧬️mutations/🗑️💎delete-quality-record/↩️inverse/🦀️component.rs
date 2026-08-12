//! ↩️ Inverse (undo) construction for the `delete-quality-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💎quality` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteQualityRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.quality.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateQualityRecord(super::super::create_quality_record::mutation::CreateQualityRecord { quality_record: existing.clone() })],
        None => Vec::new(),
    }
}
