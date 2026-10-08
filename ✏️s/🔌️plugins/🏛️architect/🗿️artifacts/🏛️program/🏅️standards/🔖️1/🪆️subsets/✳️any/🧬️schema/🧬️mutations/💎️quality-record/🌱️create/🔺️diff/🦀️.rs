//! 🔺️ Sparse diff construction for the `create-quality-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💎quality` per Wave C.

use super::CreateQualityRecord;
use crate::diff::ProgramQualityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateQualityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.quality_record.header.id;
    if base.quality.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A quality record already exists with this id.", [id.0.clone()]);
    }
    let length = base.quality.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the quality record list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.quality.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { quality: Some(ProgramQualityDelta { added: vec![payload.quality_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
