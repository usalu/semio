//! ↩️ `change-foundation-anchor-class` inverse: restores the previous `anchor_class`.

use crate::change_foundation_anchor_class::ChangeFoundationAnchorClass;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationAnchorClass, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_anchor_class::ChangeFoundationAnchorClass> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationAnchorClass { id: payload.id.clone(), new_anchor_class: row.anchor_class.clone() })],
        None => Vec::new(),
    })
}
