//! ↩️ `change-layer-depth-bottom` inverse: restores the previous `depth_bottom`.

use crate::change_layer_depth_bottom::ChangeLayerDepthBottom;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerDepthBottom, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_depth_bottom::ChangeLayerDepthBottom> {
    Ok(match base.layers.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLayerDepthBottom { id: payload.id.clone(), new_depth_bottom: row.depth_bottom })],
        None => Vec::new(),
    })
}
