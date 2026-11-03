//! ↩️ `change-assessment-chord-demand-rad` inverse: restores the previous `chord_rotation_demand_rad`.

use crate::change_assessment_chord_demand_rad::ChangeAssessmentChordDemandRad;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentChordDemandRad, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_chord_demand_rad::ChangeAssessmentChordDemandRad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentChordDemandRad, base: &En1998Snapshot) -> Option<ChangeAssessmentChordDemandRad> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentChordDemandRad { id: payload.id.clone(), new_chord_rotation_demand_rad: r0.chord_rotation_demand_rad })
}
