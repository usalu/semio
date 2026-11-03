use crate::change_case_entry_id::ChangeCaseEntryId;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &ChangeCaseEntryId, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_case_entry_id::ChangeCaseEntryId> {
    Ok((|| {
    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else { return Vec::new(); };
    vec![Op::from(ChangeCaseEntryId { case_id: payload.case_id.clone(), new_value: item.entry_id.clone() })]

    })())
}
