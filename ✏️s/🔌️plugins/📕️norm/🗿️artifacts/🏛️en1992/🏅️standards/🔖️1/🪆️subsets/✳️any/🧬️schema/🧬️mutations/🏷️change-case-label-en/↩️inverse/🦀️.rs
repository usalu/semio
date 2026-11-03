use crate::change_case_label_en::ChangeCaseLabelEn;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;
pub fn inverse<Op>(payload: &ChangeCaseLabelEn, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_case_label_en::ChangeCaseLabelEn> {
    Ok((|| {
    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else { return Vec::new(); };
    vec![Op::from(ChangeCaseLabelEn { case_id: payload.case_id.clone(), new_value: item.label_en.clone() })]

    })())
}
