//! ↩️ `change-anchor-c1` inverse: restores the previous `c1`.

use crate::change_anchor_c1::ChangeAnchorC1;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorC1, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_c1::ChangeAnchorC1> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorC1, base: &En1992Snapshot) -> Option<ChangeAnchorC1> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    Some(ChangeAnchorC1 { anchor_id: payload.anchor_id.clone(), new_c1: r0.c1 })
}
