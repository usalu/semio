//! 🔺️ Sparse diff construction for the `rename-quality-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💎quality` per Wave C.

use super::mutation::RenameQualityRecord;
use crate::artifacts::program::diff::{ProgramQualityDelta, ProgramQualityPatchEntry};
use crate::artifacts::program::registers::QualityRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameQualityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = QualityRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { quality: Some(ProgramQualityDelta { patched: vec![ProgramQualityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
