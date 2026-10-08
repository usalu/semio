//! 🔺️ Sparse diff construction for the `replace-quality-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💎quality` per Wave C.

use super::ReplaceQualityRecord;
use crate::diff::ProgramQualityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceQualityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.quality_record.header.id;
    let Some(position) = base.quality.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No quality record exists with this id.", [id.0.clone()]);
    };
    if base.quality[position] == payload.quality_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This quality record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramQualityDelta::removal(&base.quality, position);
    delta.absorb(ProgramQualityDelta::insertion(position, payload.quality_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { quality: Some(delta), ..Default::default() })
}
