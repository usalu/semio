use crate::insert_case::InsertCase;
use crate::remove_case;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertCase, _base: &En1995Snapshot) -> Option<crate::remove_case::RemoveCase> {
    Some(remove_case::RemoveCase { id: payload.case.id.clone() })
}
