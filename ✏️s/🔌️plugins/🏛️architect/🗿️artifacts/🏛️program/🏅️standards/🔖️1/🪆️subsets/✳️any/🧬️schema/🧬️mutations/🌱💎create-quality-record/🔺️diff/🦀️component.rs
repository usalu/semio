//! 🔺️ Sparse diff construction for the `create-quality-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💎quality` per Wave C.

use super::mutation::CreateQualityRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramQualityDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.quality` on apply.
pub fn diff(payload: &CreateQualityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { quality: Some(ProgramQualityDelta { added: vec![payload.quality_record.clone()], ..Default::default() }), ..Default::default() }
}
