use crate::insert_uplift_case::InsertUpliftCase;
use crate::remove_uplift_case::RemoveUpliftCase;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertUpliftCase, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_uplift_case::RemoveUpliftCase> {
    Ok((|| {
    let at = payload.index.min(base.uplift_cases.len());
    vec![Op::from(RemoveUpliftCase { index: at })]

    })())
}
