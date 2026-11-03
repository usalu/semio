use crate::change_case_number::ChangeCaseNumber;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
pub fn inverse<Op>(payload: &ChangeCaseNumber, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_case_number::ChangeCaseNumber> {
    Ok((|| {
    if !base.cases.iter().any(|item| item.id == payload.case_id) { return Vec::new(); }
    vec![Op::from(ChangeCaseNumber { case_id: payload.case_id.clone(), key: payload.key.clone(), value: semio_s_plugin_norm_compliance::design_case::previous_number(&base.cases, &payload.case_id, &payload.key) })]

    })())
}
