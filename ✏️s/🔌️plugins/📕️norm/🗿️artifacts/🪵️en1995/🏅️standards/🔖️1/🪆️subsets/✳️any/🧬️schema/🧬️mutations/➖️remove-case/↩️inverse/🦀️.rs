use crate::remove_case::RemoveCase;
use crate::insert_case;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveCase, base: &En1995Snapshot) -> Option<crate::insert_case::InsertCase> {
    let Some((index, case)) = base.cases.iter().enumerate().find(|(_, case)| case.id == payload.id) else { return None; };
    Some(insert_case::InsertCase { index, case: case.clone() })
}
