//! ↩️ `remove-anchor-action` inverse: inserts the removed row back at its position.

use crate::remove_anchor_action::RemoveAnchorAction;
use crate::insert_anchor_action::InsertAnchorAction;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveAnchorAction, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_anchor_action::InsertAnchorAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveAnchorAction, base: &En1992Snapshot) -> Option<InsertAnchorAction> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    let index = payload.index;
    let row = r0.actions.get(index)?;
    Some(InsertAnchorAction { anchor_id: payload.anchor_id.clone(), index, anchor_action: row.clone() })
}
