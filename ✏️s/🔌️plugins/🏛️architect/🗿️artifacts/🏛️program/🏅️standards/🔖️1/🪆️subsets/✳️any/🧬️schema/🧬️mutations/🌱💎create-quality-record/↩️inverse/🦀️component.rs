//! ↩️ Inverse (undo) construction for the `create-quality-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💎quality` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateQualityRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteQualityRecord(super::super::delete_quality_record::mutation::DeleteQualityRecord { id: payload.quality_record.header.id.clone() })]
}
