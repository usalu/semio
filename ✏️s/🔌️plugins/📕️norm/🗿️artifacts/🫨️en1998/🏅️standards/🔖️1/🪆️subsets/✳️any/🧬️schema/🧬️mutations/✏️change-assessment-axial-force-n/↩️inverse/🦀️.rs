//! ↩️ `change-assessment-axial-force-n` inverse: restores the previous `axial_force_n`.

use crate::change_assessment_axial_force_n::ChangeAssessmentAxialForceN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentAxialForceN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_axial_force_n::ChangeAssessmentAxialForceN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentAxialForceN, base: &En1998Snapshot) -> Option<ChangeAssessmentAxialForceN> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentAxialForceN { id: payload.id.clone(), new_axial_force_n: r0.axial_force_n })
}
