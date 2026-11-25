use crate::change_case_label_de::ChangeCaseLabelDe;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCaseLabelDe, base: &En1995Snapshot) -> Option<crate::change_case_label_de::ChangeCaseLabelDe> {
    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else { return None; };
    Some(ChangeCaseLabelDe { case_id: payload.case_id.clone(), new_value: item.label_de.clone() })
}
