use crate::remove_tower_action::RemoveTowerAction;
use crate::insert_tower_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &RemoveTowerAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tower_action::InsertTowerAction> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.get(payload.index)) {
        Some(element) => vec![Op::from(insert_tower_action::InsertTowerAction { id: payload.id.clone(), index: Some(payload.index), element: element.clone() })],
        None => Vec::new(),
    })
}
