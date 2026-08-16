//! 🔺️ Sparse diff construction for the `delete-survey` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗳️surveys` per Wave C.

use super::mutation::DeleteSurvey;
use crate::artifacts::program::diff::ProgramSurveysDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteSurvey, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { surveys: Some(ProgramSurveysDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
