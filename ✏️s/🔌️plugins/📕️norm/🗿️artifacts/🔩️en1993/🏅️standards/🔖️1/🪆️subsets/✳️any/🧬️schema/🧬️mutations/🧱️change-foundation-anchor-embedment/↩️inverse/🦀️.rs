//! ↩️ `change-foundation-anchor-embedment` inverse: restores the previous `anchor_embedment`.

use crate::change_foundation_anchor_embedment::ChangeFoundationAnchorEmbedment;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationAnchorEmbedment, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_anchor_embedment::ChangeFoundationAnchorEmbedment> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationAnchorEmbedment { id: payload.id.clone(), new_anchor_embedment: row.anchor_embedment })],
        None => Vec::new(),
    })
}
