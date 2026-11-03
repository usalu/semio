//! ↩️ `change-anchor-d` inverse: restores the previous `d`.

use crate::change_anchor_d::ChangeAnchorD;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorD, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_d::ChangeAnchorD> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorD, base: &En1992Snapshot) -> Option<ChangeAnchorD> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    Some(ChangeAnchorD { anchor_id: payload.anchor_id.clone(), new_d: r0.d })
}
