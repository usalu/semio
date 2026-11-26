use crate::insert_case::InsertCase;
use crate::remove_case;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
pub fn inverse<Op>(payload: &InsertCase, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_case::RemoveCase> {
    Ok(match previous(payload, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(payload: &InsertCase, _base: &En1990Snapshot) -> Option<crate::remove_case::RemoveCase> {
    Some(remove_case::RemoveCase { id: payload.case.id.clone() })
}
