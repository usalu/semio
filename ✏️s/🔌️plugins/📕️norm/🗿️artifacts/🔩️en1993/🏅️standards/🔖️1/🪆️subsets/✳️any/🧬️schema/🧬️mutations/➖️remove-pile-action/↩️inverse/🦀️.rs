use crate::remove_pile_action::RemovePileAction;
use crate::insert_pile_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &RemovePileAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_pile_action::InsertPileAction> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.get(payload.index)) {
        Some(element) => vec![Op::from(insert_pile_action::InsertPileAction { id: payload.id.clone(), index: Some(payload.index), element: element.clone() })],
        None => Vec::new(),
    })
}
