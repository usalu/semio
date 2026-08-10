//! ➖️ Lowpoly mutation — `RemovePaintLayer` payload + builder + apply.
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use serde::{Deserialize, Serialize};


//#region 🔖️Mutation
/// @emoji ➖️ `RemovePaintLayer` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RemovePaintLayer {
    pub object_id: String,
    pub index: usize,
}

pub fn remove_paint_layer(object_id: impl Into<String>, index: usize) -> LowpolyMutation {
    LowpolyMutation::RemovePaintLayer { object_id: object_id.into(), index }
}

pub fn apply(projection: &mut LowpolySnapshot, object_id: &str, index: usize) {
    if let Some(object) = crate::artifacts::lowpoly::engine::object_mut(projection, object_id) {
        if index < object.paint_layers.len() {
            object.paint_layers.remove(index);
        }
    }
}
//#endregion 🔖️Mutation
