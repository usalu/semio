//! 🔺️ `change-layer-depth-top` diff: patches `depth_top` of the addressed soil layer row.

use crate::change_layer_depth_top::ChangeLayerDepthTop;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LayersPatch, En1997LayersRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeLayerDepthTop, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_depth_top.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.layers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("layer {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.depth_top == payload.new_depth_top {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The soil layer already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { layers: Some(En1997LayersRows::modification(&payload.id, En1997LayersPatch { depth_top: Some(payload.new_depth_top), ..Default::default() })), ..Default::default() })
}
