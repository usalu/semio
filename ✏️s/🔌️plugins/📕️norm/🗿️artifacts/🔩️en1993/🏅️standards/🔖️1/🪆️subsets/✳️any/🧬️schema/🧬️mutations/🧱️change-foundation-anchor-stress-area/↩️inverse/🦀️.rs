//! ↩️ `change-foundation-anchor-stress-area` inverse: restores the previous `anchor_stress_area`.

use crate::change_foundation_anchor_stress_area::ChangeFoundationAnchorStressArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationAnchorStressArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_anchor_stress_area::ChangeFoundationAnchorStressArea> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationAnchorStressArea { id: payload.id.clone(), new_anchor_stress_area: row.anchor_stress_area })],
        None => Vec::new(),
    })
}
