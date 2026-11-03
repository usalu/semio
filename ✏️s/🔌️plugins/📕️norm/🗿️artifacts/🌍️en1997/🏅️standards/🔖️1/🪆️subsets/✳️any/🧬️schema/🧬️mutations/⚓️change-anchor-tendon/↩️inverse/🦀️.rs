//! ↩️ `change-anchor-tendon` inverse: restores the previous `tendon`.

use crate::change_anchor_tendon::ChangeAnchorTendon;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorTendon, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_tendon::ChangeAnchorTendon> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorTendon { id: payload.id.clone(), new_tendon: row.tendon })],
        None => Vec::new(),
    })
}
