//! 🔺️ Sparse diff construction for the `delete-quality-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💎quality` per Wave C.

use super::mutation::DeleteQualityRecord;
use crate::artifacts::program::diff::ProgramQualityDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteQualityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { quality: Some(ProgramQualityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
