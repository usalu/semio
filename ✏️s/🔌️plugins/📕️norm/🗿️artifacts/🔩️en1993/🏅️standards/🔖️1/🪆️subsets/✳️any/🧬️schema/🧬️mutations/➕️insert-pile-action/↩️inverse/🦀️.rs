use crate::insert_pile_action::InsertPileAction;
use crate::remove_pile_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &InsertPileAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_pile_action::RemovePileAction> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(remove_pile_action::RemovePileAction { id: payload.id.clone(), index: payload.index.unwrap_or(usize::MAX).min(row.actions.len()) })],
        None => Vec::new(),
    })
}
