//! ✏️ `change-assessment-knowledge-level` diff: patches `knowledge_level` of the addressed assessment row.

use crate::change_assessment_knowledge_level::ChangeAssessmentKnowledgeLevel;
use semio_s_artifact_norm_en1998_diff::{En1998AssessmentDelta, En1998AssessmentPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAssessmentKnowledgeLevel, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.assessments.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.knowledge_level == payload.new_knowledge_level {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The assessment already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::modification(&payload.id, En1998AssessmentPatch { knowledge_level: Some(payload.new_knowledge_level), ..Default::default() }), ..Default::default() })
}
