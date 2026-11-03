//! ↩️ `change-anchor-head-diameter` inverse: restores the previous `head_diameter`.

use crate::change_anchor_head_diameter::ChangeAnchorHeadDiameter;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorHeadDiameter, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_head_diameter::ChangeAnchorHeadDiameter> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorHeadDiameter, base: &En1992Snapshot) -> Option<ChangeAnchorHeadDiameter> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    Some(ChangeAnchorHeadDiameter { anchor_id: payload.anchor_id.clone(), new_head_diameter: r0.head_diameter })
}
