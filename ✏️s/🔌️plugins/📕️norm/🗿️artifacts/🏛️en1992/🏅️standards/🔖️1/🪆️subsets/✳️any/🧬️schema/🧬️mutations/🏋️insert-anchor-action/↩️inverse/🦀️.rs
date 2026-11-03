//! ↩️ `insert-anchor-action` inverse: removes the inserted row again.

use crate::insert_anchor_action::InsertAnchorAction;
use crate::remove_anchor_action::RemoveAnchorAction;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertAnchorAction, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_anchor_action::RemoveAnchorAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertAnchorAction, base: &En1992Snapshot) -> Option<RemoveAnchorAction> {
    let _r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    let index = payload.index;
    Some(RemoveAnchorAction { anchor_id: payload.anchor_id.clone(), index })
}
