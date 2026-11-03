use crate::remove_uplift_case::RemoveUpliftCase;
use crate::insert_uplift_case::InsertUpliftCase;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveUpliftCase, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_uplift_case::InsertUpliftCase> {
    Ok((|| {
    match base.uplift_cases.get(payload.index).cloned() {
        Some(uplift_case) => vec![Op::from(InsertUpliftCase { index: payload.index, uplift_case })],
        None => Vec::new(),
    }

    })())
}
