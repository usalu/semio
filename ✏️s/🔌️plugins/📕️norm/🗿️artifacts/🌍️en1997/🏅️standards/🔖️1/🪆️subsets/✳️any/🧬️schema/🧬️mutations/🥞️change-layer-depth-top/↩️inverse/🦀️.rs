//! ↩️ `change-layer-depth-top` inverse: restores the previous `depth_top`.

use crate::change_layer_depth_top::ChangeLayerDepthTop;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerDepthTop, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_depth_top::ChangeLayerDepthTop> {
    Ok(match base.layers.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLayerDepthTop { id: payload.id.clone(), new_depth_top: row.depth_top })],
        None => Vec::new(),
    })
}
