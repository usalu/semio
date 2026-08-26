//! 🔺️ `change-paint-layer-opacity` — sparse diff construction: one-field paint-layer patch. Error
//! `target-missing` when the object or layer index is absent, Warning `no-op` when unchanged.

use super::mutation::ChangePaintLayerOpacity;
use crate::artifacts::lowpoly::diff::diff_patch_paint_layer;
use crate::artifacts::lowpoly::diff::schema::LowpolyPaintLayerPatch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangePaintLayerOpacity, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.object_id), [payload.object_id.clone()]);
    };
    let Some(layer) = object.paint_layers.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Paint layer {} does not exist on object \"{}\".", payload.index, payload.object_id), [payload.object_id.clone()]);
    };
    if layer.opacity == payload.new_opacity {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Paint layer {} opacity is already {}.", payload.index, payload.new_opacity));
    }
    protocol::MutationOutcome::new(diff_patch_paint_layer(payload.object_id.clone(), payload.index, LowpolyPaintLayerPatch { opacity: Some(payload.new_opacity), ..LowpolyPaintLayerPatch::default() }))
}
//#endregion 🔖️Diff
