//! 🔺️ Sparse diff construction for the `rename-quality-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💎quality` per Wave C.

use super::mutation::RenameQualityRecord;
use crate::artifacts::program::diff::{ProgramQualityDelta, ProgramQualityPatchEntry};
use crate::artifacts::program::registers::QualityRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameQualityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.quality.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No quality record exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This quality record already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = QualityRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { quality: Some(ProgramQualityDelta { patched: vec![ProgramQualityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
