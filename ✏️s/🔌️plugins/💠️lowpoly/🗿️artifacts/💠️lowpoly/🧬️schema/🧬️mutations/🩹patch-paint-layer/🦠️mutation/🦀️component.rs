//! 🩹 Lowpoly mutation — `PatchPaintLayer` payload + builder + apply.
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::artifacts::lowpoly::mutations::LowpolyPaintLayerPatch;
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use serde::{Deserialize, Serialize};


//#region 🔖️Mutation
/// @emoji 🩹 `PatchPaintLayer` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PatchPaintLayer {
    pub object_id: String,
    pub index: usize,
    #[dsl(block)]
    pub patch: LowpolyPaintLayerPatch,
}

pub fn patch_paint_layer(object_id: impl Into<String>, index: usize, patch: LowpolyPaintLayerPatch) -> LowpolyMutation {
    LowpolyMutation::PatchPaintLayer { object_id: object_id.into(), index, patch }
}

pub fn apply(projection: &mut LowpolySnapshot, object_id: &str, index: usize, patch: &LowpolyPaintLayerPatch) {
    if let Some(object) = crate::artifacts::lowpoly::engine::object_mut(projection, object_id) {
        if let Some(layer) = object.paint_layers.get_mut(index) {
            crate::artifacts::lowpoly::mutations::apply_paint_layer_patch(layer, patch);
        }
    }
}
//#endregion 🔖️Mutation
