//! ↩️ `change-anchor-f-ck` inverse: restores the previous `f_ck`.

use crate::change_anchor_f_ck::ChangeAnchorFCk;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorFCk, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_f_ck::ChangeAnchorFCk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorFCk, base: &En1992Snapshot) -> Option<ChangeAnchorFCk> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    Some(ChangeAnchorFCk { anchor_id: payload.anchor_id.clone(), new_f_ck: r0.f_ck })
}
