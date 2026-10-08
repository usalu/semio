//! ↩️ Inverse (undo) construction for the `delete-quality-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💎quality` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteQualityRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.quality.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateQualityRecord(super::super::create_quality_record::CreateQualityRecord { quality_record: base.quality[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
