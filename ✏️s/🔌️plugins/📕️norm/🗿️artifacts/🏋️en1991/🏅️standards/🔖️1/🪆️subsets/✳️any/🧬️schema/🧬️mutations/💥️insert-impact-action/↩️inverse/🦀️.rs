//! ↩️ `insert-impact-action` inverse: removes the inserted row again.

use crate::insert_impact_action::InsertImpactAction;
use crate::remove_impact_action::RemoveImpactAction;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertImpactAction, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_impact_action::RemoveImpactAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertImpactAction, base: &En1991Snapshot) -> Option<RemoveImpactAction> {
    Some(RemoveImpactAction { id: payload.item.id.clone() })
}
