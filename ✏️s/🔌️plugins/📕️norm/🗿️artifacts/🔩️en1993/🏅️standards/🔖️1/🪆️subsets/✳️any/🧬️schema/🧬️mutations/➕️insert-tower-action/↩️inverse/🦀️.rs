use crate::insert_tower_action::InsertTowerAction;
use crate::remove_tower_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &InsertTowerAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tower_action::RemoveTowerAction> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(remove_tower_action::RemoveTowerAction { id: payload.id.clone(), index: payload.index.unwrap_or(usize::MAX).min(row.actions.len()) })],
        None => Vec::new(),
    })
}
