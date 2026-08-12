//! 🔺️ Sparse diff construction for the `create-survey` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗳️surveys` per Wave C.

use super::mutation::CreateSurvey;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSurveysDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.surveys` on apply.
pub fn diff(payload: &CreateSurvey, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { surveys: Some(ProgramSurveysDelta { added: vec![payload.survey.clone()], ..Default::default() }), ..Default::default() }
}
