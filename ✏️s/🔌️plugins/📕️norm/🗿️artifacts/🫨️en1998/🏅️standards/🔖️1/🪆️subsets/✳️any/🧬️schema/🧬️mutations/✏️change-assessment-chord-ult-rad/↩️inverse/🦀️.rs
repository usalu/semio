//! ↩️ `change-assessment-chord-ult-rad` inverse: restores the previous `chord_rotation_ultimate_rad`.

use crate::change_assessment_chord_ult_rad::ChangeAssessmentChordUltRad;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentChordUltRad, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_chord_ult_rad::ChangeAssessmentChordUltRad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentChordUltRad, base: &En1998Snapshot) -> Option<ChangeAssessmentChordUltRad> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentChordUltRad { id: payload.id.clone(), new_chord_rotation_ultimate_rad: r0.chord_rotation_ultimate_rad })
}
