//! ↩️ `remove-impact-action` inverse: inserts the removed row back at its position.

use crate::remove_impact_action::RemoveImpactAction;
use crate::insert_impact_action::InsertImpactAction;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveImpactAction, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_impact_action::InsertImpactAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveImpactAction, base: &En1991Snapshot) -> Option<InsertImpactAction> {
    let (index, row) = base.impact_actions.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertImpactAction { index, item: row.clone() })
}
