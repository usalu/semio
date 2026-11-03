//! ↩️ `change-assessment-shear-force-demand-n` inverse: restores the previous `shear_force_demand_n`.

use crate::change_assessment_shear_force_demand_n::ChangeAssessmentShearForceDemandN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentShearForceDemandN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_shear_force_demand_n::ChangeAssessmentShearForceDemandN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentShearForceDemandN, base: &En1998Snapshot) -> Option<ChangeAssessmentShearForceDemandN> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentShearForceDemandN { id: payload.id.clone(), new_shear_force_demand_n: r0.shear_force_demand_n })
}
