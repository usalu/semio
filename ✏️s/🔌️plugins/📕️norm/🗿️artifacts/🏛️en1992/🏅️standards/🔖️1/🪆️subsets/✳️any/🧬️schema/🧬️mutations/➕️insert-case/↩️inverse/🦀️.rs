use crate::insert_case::InsertCase;
use crate::remove_case;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;
pub fn inverse<Op>(payload: &InsertCase, _base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_case::RemoveCase> {
    Ok((|| {
    vec![Op::from(remove_case::RemoveCase { id: payload.case.id.clone() })]

    })())
}
