//! ↩️ `change-assessment-analysis-method` inverse: restores the previous `analysis_method`.

use crate::change_assessment_analysis_method::ChangeAssessmentAnalysisMethod;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentAnalysisMethod, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_analysis_method::ChangeAssessmentAnalysisMethod> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentAnalysisMethod, base: &En1998Snapshot) -> Option<ChangeAssessmentAnalysisMethod> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentAnalysisMethod { id: payload.id.clone(), new_analysis_method_choice: r0.analysis_method })
}
