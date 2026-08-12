//! 🔺️ Sparse diff construction for the `replace-quality-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💎quality` per Wave C.

use super::mutation::ReplaceQualityRecord;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramQualityDelta, ProgramQualityPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceQualityRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.quality.iter().find(|row| row.header.id == payload.quality_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.quality_record).expect("diff_patch always produces a full patch");
    ProgramDiff { quality: Some(ProgramQualityDelta { patched: vec![ProgramQualityPatchEntry { id: payload.quality_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
