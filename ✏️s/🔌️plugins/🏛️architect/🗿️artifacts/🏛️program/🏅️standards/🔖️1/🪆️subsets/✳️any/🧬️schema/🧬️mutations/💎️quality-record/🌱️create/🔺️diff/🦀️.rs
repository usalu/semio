//! 🔺️ Sparse diff construction for the `create-quality-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💎quality` per Wave C.

use super::CreateQualityRecord;
use crate::diff::ProgramQualityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateQualityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.quality_record.header.id;
    if base.quality.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A quality record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.quality.len());
    if at > base.quality.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the quality record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { quality: Some(ProgramQualityDelta::insertion(at, payload.quality_record.clone())), ..Default::default() })
}
