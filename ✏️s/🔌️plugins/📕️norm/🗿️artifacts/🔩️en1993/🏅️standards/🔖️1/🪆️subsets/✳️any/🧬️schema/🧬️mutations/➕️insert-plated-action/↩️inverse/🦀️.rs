use crate::insert_plated_action::InsertPlatedAction;
use crate::remove_plated_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &InsertPlatedAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_plated_action::RemovePlatedAction> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(remove_plated_action::RemovePlatedAction { id: payload.id.clone(), index: payload.index.unwrap_or(usize::MAX).min(row.actions.len()) })],
        None => Vec::new(),
    })
}
