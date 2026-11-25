use crate::change_case_label_en::ChangeCaseLabelEn;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCaseLabelEn, base: &En1995Snapshot) -> Option<crate::change_case_label_en::ChangeCaseLabelEn> {
    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else { return None; };
    Some(ChangeCaseLabelEn { case_id: payload.case_id.clone(), new_value: item.label_en.clone() })
}
