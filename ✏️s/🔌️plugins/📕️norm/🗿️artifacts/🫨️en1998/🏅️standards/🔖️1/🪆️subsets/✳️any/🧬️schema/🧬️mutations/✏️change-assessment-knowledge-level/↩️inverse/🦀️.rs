//! ↩️ `change-assessment-knowledge-level` inverse: restores the previous `knowledge_level`.

use crate::change_assessment_knowledge_level::ChangeAssessmentKnowledgeLevel;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentKnowledgeLevel, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_knowledge_level::ChangeAssessmentKnowledgeLevel> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentKnowledgeLevel, base: &En1998Snapshot) -> Option<ChangeAssessmentKnowledgeLevel> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentKnowledgeLevel { id: payload.id.clone(), new_knowledge_level: r0.knowledge_level })
}
