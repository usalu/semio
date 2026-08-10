//! ➕️ Lowpoly mutation — `AddPaintLayer` payload + builder + apply.
use crate::artifacts::lowpoly::{LowpolyPaintLayer, LowpolySnapshot};
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use serde::{Deserialize, Serialize};


//#region 🔖️Mutation
/// @emoji ➕️ `AddPaintLayer` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct AddPaintLayer {
    pub object_id: String,
    pub index: usize,
    #[dsl(block)]
    pub layer: LowpolyPaintLayer,
}

pub fn add_paint_layer(object_id: impl Into<String>, index: usize, layer: LowpolyPaintLayer) -> LowpolyMutation {
    LowpolyMutation::AddPaintLayer { object_id: object_id.into(), index, layer }
}

pub fn apply(projection: &mut LowpolySnapshot, object_id: &str, index: usize, layer: &LowpolyPaintLayer) {
    if let Some(object) = crate::artifacts::lowpoly::engine::object_mut(projection, object_id) {
        let at = index.min(object.paint_layers.len());
        object.paint_layers.insert(at, layer.clone());
    }
}
//#endregion 🔖️Mutation
