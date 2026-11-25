use crate::change_case_number::ChangeCaseNumber;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCaseNumber, base: &En1995Snapshot) -> Option<crate::change_case_number::ChangeCaseNumber> {
    if !base.cases.iter().any(|item| item.id == payload.case_id) { return None; }
    Some(ChangeCaseNumber { case_id: payload.case_id.clone(), key: payload.key.clone(), value: semio_s_plugin_norm_compliance::design_case::previous_number(&base.cases, &payload.case_id, &payload.key) })
}
