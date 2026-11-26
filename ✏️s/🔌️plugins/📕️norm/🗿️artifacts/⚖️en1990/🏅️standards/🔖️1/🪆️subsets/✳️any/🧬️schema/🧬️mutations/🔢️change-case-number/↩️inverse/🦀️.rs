use crate::change_case_number::ChangeCaseNumber;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
pub fn inverse<Op>(payload: &ChangeCaseNumber, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_case_number::ChangeCaseNumber> {
    Ok(match previous(payload, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(payload: &ChangeCaseNumber, base: &En1990Snapshot) -> Option<crate::change_case_number::ChangeCaseNumber> {
    if !base.cases.iter().any(|item| item.id == payload.case_id) {
        return None;
    }
    Some(ChangeCaseNumber { case_id: payload.case_id.clone(), key: payload.key.clone(), value: semio_s_plugin_norm_compliance::design_case::previous_number(&base.cases, &payload.case_id, &payload.key) })
}
