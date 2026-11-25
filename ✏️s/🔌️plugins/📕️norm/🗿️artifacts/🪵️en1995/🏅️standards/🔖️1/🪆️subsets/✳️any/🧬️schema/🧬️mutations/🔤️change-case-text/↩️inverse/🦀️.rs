use crate::change_case_text::ChangeCaseText;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCaseText, base: &En1995Snapshot) -> Option<crate::change_case_text::ChangeCaseText> {
    if !base.cases.iter().any(|item| item.id == payload.case_id) { return None; }
    Some(ChangeCaseText { case_id: payload.case_id.clone(), key: payload.key.clone(), value: semio_s_plugin_norm_compliance::design_case::previous_text(&base.cases, &payload.case_id, &payload.key) })
}
