//! 🔺️ Sparse diff construction for the `rename-survey` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗳️surveys` per Wave C.

use super::mutation::RenameSurvey;
use crate::artifacts::program::diff::{ProgramSurveysDelta, ProgramSurveysPatchEntry};
use crate::artifacts::program::registers::SurveyPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameSurvey, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = SurveyPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { surveys: Some(ProgramSurveysDelta { patched: vec![ProgramSurveysPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
