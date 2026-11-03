use crate::change_case_label_de::ChangeCaseLabelDe;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeCaseLabelDe, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_case_label_de::ChangeCaseLabelDe> {
    Ok((|| {
    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else { return Vec::new(); };
    vec![Op::from(ChangeCaseLabelDe { case_id: payload.case_id.clone(), new_value: item.label_de.clone() })]

    })())
}
