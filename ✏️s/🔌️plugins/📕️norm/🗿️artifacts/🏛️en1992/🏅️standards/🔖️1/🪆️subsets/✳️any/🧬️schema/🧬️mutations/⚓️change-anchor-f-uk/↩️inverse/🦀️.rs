//! ↩️ `change-anchor-f-uk` inverse: restores the previous `f_uk`.

use crate::change_anchor_f_uk::ChangeAnchorFUk;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorFUk, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_f_uk::ChangeAnchorFUk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorFUk, base: &En1992Snapshot) -> Option<ChangeAnchorFUk> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    Some(ChangeAnchorFUk { anchor_id: payload.anchor_id.clone(), new_f_uk: r0.f_uk })
}
