use crate::remove_case::RemoveCase;
use crate::insert_case;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
pub fn inverse<Op>(payload: &RemoveCase, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_case::InsertCase> {
    Ok(match previous(payload, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(payload: &RemoveCase, base: &En1990Snapshot) -> Option<crate::insert_case::InsertCase> {
    let Some((index, case)) = base.cases.iter().enumerate().find(|(_, case)| case.id == payload.id) else {
        return None;
    };
    Some(insert_case::InsertCase { index, case: case.clone() })
}
