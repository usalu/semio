//! 🔺️ Sparse diff construction for the `create-quality-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💎quality` per Wave C.

use super::CreateQualityRecord;
use crate::diff::ProgramQualityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateQualityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.quality_record.header.id.clone();
    if base.quality.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A quality record already exists with this id.", [id.0]);
    }
    protocol::MutationOutcome::new(ProgramDiff { quality: Some(ProgramQualityDelta { added: vec![payload.quality_record.clone()], ..Default::default() }), ..Default::default() })
}
