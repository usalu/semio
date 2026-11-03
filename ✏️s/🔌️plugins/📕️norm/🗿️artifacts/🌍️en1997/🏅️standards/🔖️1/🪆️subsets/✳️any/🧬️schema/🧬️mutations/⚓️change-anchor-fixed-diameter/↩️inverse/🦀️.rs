//! ↩️ `change-anchor-fixed-diameter` inverse: restores the previous `fixed_diameter`.

use crate::change_anchor_fixed_diameter::ChangeAnchorFixedDiameter;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorFixedDiameter, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_fixed_diameter::ChangeAnchorFixedDiameter> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorFixedDiameter { id: payload.id.clone(), new_fixed_diameter: row.fixed_diameter })],
        None => Vec::new(),
    })
}
