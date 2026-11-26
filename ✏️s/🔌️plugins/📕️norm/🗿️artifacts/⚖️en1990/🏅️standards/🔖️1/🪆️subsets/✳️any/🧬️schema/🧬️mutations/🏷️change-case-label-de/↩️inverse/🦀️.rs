use crate::change_case_label_de::ChangeCaseLabelDe;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
pub fn inverse<Op>(payload: &ChangeCaseLabelDe, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_case_label_de::ChangeCaseLabelDe> {
    Ok(match previous(payload, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(payload: &ChangeCaseLabelDe, base: &En1990Snapshot) -> Option<crate::change_case_label_de::ChangeCaseLabelDe> {
    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else {
        return None;
    };
    Some(ChangeCaseLabelDe { case_id: payload.case_id.clone(), new_value: item.label_de.clone() })
}
