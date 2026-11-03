use crate::insert_crane_action::InsertCraneAction;
use crate::remove_crane_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &InsertCraneAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_crane_action::RemoveCraneAction> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(remove_crane_action::RemoveCraneAction { id: payload.id.clone(), index: payload.index.unwrap_or(usize::MAX).min(row.actions.len()) })],
        None => Vec::new(),
    })
}
