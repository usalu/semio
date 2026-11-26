use crate::change_case_entry_id::ChangeCaseEntryId;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
pub fn inverse<Op>(payload: &ChangeCaseEntryId, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_case_entry_id::ChangeCaseEntryId> {
    Ok(match previous(payload, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(payload: &ChangeCaseEntryId, base: &En1990Snapshot) -> Option<crate::change_case_entry_id::ChangeCaseEntryId> {
    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else {
        return None;
    };
    Some(ChangeCaseEntryId { case_id: payload.case_id.clone(), new_value: item.entry_id.clone() })
}
