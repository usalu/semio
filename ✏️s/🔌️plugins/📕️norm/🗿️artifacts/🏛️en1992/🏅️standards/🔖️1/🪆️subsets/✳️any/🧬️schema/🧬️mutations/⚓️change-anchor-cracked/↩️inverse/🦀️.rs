//! ↩️ `change-anchor-cracked` inverse: restores the previous `cracked`.

use crate::change_anchor_cracked::ChangeAnchorCracked;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorCracked, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_cracked::ChangeAnchorCracked> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorCracked, base: &En1992Snapshot) -> Option<ChangeAnchorCracked> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    Some(ChangeAnchorCracked { anchor_id: payload.anchor_id.clone(), new_cracked: r0.cracked })
}
