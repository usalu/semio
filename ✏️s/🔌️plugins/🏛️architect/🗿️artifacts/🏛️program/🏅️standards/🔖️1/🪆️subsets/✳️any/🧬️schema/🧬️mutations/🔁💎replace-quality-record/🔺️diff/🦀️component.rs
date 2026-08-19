//! 🔺️ Sparse diff construction for the `replace-quality-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💎quality` per Wave C.

use super::mutation::ReplaceQualityRecord;
use crate::artifacts::program::diff::{ProgramQualityDelta, ProgramQualityPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceQualityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.quality.iter().find(|row| row.header.id == payload.quality_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No quality record exists with this id.", [payload.quality_record.header.id.0.clone()]);
    };
    if existing == &payload.quality_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This quality record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.quality_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { quality: Some(ProgramQualityDelta { patched: vec![ProgramQualityPatchEntry { id: payload.quality_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
