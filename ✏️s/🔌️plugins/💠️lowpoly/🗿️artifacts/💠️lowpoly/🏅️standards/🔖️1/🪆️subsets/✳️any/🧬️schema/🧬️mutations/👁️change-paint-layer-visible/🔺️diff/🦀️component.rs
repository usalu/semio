//! 🔺️ `change-paint-layer-visible` — sparse diff construction: one-field paint-layer patch. Error
//! `target-missing` when the object or layer index is absent, Warning `no-op` when unchanged.

use super::mutation::ChangePaintLayerVisible;
use crate::artifacts::lowpoly::diff::diff_patch_paint_layer;
use crate::artifacts::lowpoly::diff::schema::LowpolyPaintLayerPatch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangePaintLayerVisible, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.object_id), [payload.object_id.clone()]);
    };
    let Some(layer) = object.paint_layers.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Paint layer {} does not exist on object \"{}\".", payload.index, payload.object_id), [payload.object_id.clone()]);
    };
    if layer.visible == payload.new_visible {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Paint layer {} visibility is already {}.", payload.index, payload.new_visible));
    }
    protocol::MutationOutcome::new(diff_patch_paint_layer(payload.object_id.clone(), payload.index, LowpolyPaintLayerPatch { visible: Some(payload.new_visible), ..LowpolyPaintLayerPatch::default() }))
}
//#endregion 🔖️Diff
