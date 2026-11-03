//! ↩️ `change-foundation-anchor-diameter` inverse: restores the previous `anchor_diameter`.

use crate::change_foundation_anchor_diameter::ChangeFoundationAnchorDiameter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationAnchorDiameter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_anchor_diameter::ChangeFoundationAnchorDiameter> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationAnchorDiameter { id: payload.id.clone(), new_anchor_diameter: row.anchor_diameter })],
        None => Vec::new(),
    })
}
