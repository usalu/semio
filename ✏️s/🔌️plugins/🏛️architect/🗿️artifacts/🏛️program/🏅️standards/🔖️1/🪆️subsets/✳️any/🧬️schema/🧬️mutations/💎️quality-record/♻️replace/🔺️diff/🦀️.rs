//! 🔺️ Sparse diff construction for the `replace-quality-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💎quality` per Wave C.

use super::ReplaceQualityRecord;
use crate::diff::ProgramQualityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceQualityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.quality_record.header.id;
    let Some(position) = base.quality.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No quality record exists with this id.", [id.0.clone()]);
    };
    if base.quality[position] == payload.quality_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This quality record already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.quality.len()).then(|| base.quality.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { quality: Some(ProgramQualityDelta { removed: vec![id.0.clone()], added: vec![payload.quality_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
