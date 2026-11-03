//! ↩️ `change-assessment-chord-yield-rad` inverse: restores the previous `chord_rotation_yield_rad`.

use crate::change_assessment_chord_yield_rad::ChangeAssessmentChordYieldRad;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentChordYieldRad, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_chord_yield_rad::ChangeAssessmentChordYieldRad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentChordYieldRad, base: &En1998Snapshot) -> Option<ChangeAssessmentChordYieldRad> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentChordYieldRad { id: payload.id.clone(), new_chord_rotation_yield_rad: r0.chord_rotation_yield_rad })
}
