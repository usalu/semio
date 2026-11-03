use crate::insert_case::InsertCase;
use crate::remove_case;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &InsertCase, _base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_case::RemoveCase> {
    Ok((|| {
    vec![Op::from(remove_case::RemoveCase { id: payload.case.id.clone() })]

    })())
}
