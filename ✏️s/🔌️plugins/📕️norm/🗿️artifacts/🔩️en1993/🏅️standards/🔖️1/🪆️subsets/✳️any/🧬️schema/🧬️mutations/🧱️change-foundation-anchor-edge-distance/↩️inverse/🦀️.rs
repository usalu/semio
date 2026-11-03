//! ↩️ `change-foundation-anchor-edge-distance` inverse: restores the previous `anchor_edge_distance`.

use crate::change_foundation_anchor_edge_distance::ChangeFoundationAnchorEdgeDistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationAnchorEdgeDistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_anchor_edge_distance::ChangeFoundationAnchorEdgeDistance> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationAnchorEdgeDistance { id: payload.id.clone(), new_anchor_edge_distance: row.anchor_edge_distance })],
        None => Vec::new(),
    })
}
