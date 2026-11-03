use crate::change_case_text::ChangeCaseText;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeCaseText, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_case_text::ChangeCaseText> {
    Ok((|| {
    if !base.cases.iter().any(|item| item.id == payload.case_id) { return Vec::new(); }
    vec![Op::from(ChangeCaseText { case_id: payload.case_id.clone(), key: payload.key.clone(), value: semio_s_plugin_norm_compliance::design_case::previous_text(&base.cases, &payload.case_id, &payload.key) })]

    })())
}
