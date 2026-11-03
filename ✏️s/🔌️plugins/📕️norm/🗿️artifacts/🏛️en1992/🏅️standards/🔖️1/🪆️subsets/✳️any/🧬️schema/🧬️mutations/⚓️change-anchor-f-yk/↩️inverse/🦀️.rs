//! ↩️ `change-anchor-f-yk` inverse: restores the previous `f_yk`.

use crate::change_anchor_f_yk::ChangeAnchorFYk;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorFYk, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_f_yk::ChangeAnchorFYk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorFYk, base: &En1992Snapshot) -> Option<ChangeAnchorFYk> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    Some(ChangeAnchorFYk { anchor_id: payload.anchor_id.clone(), new_f_yk: r0.f_yk })
}
