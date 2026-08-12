//! 🔺️ Sparse diff construction for the `replace-survey` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗳️surveys` per Wave C.

use super::mutation::ReplaceSurvey;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSurveysDelta, ProgramSurveysPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceSurvey, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.surveys.iter().find(|row| row.header.id == payload.survey.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.survey).expect("diff_patch always produces a full patch");
    ProgramDiff { surveys: Some(ProgramSurveysDelta { patched: vec![ProgramSurveysPatchEntry { id: payload.survey.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
