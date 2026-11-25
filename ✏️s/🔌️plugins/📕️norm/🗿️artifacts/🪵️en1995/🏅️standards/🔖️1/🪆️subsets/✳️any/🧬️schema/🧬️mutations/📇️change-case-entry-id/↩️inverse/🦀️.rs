use crate::change_case_entry_id::ChangeCaseEntryId;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCaseEntryId, base: &En1995Snapshot) -> Option<crate::change_case_entry_id::ChangeCaseEntryId> {
    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else { return None; };
    Some(ChangeCaseEntryId { case_id: payload.case_id.clone(), new_value: item.entry_id.clone() })
}
