use crate::remove_case::RemoveCase;
use crate::insert_case;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &RemoveCase, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_case::InsertCase> {
    Ok((|| {
    let Some((index, case)) = base.cases.iter().enumerate().find(|(_, case)| case.id == payload.id) else { return Vec::new(); };
    vec![Op::from(insert_case::InsertCase { index, case: case.clone() })]

    })())
}
