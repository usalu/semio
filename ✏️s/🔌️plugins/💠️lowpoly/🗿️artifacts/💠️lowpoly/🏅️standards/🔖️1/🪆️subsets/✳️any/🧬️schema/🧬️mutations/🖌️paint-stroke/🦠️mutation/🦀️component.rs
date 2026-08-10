//! 🖌️ Lowpoly mutation — `PaintStroke` payload + builder + apply.
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::artifacts::lowpoly::mutations::PixelRun;
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use serde::{Deserialize, Serialize};


//#region 🔖️Mutation
/// @emoji 🖌️ `PaintStroke` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PaintStroke {
    pub object_id: String,
    pub layer_index: usize,
    #[dsl(table)]
    pub runs: Vec<PixelRun>,
}

pub fn paint_stroke(object_id: impl Into<String>, layer_index: usize, runs: Vec<PixelRun>) -> LowpolyMutation {
    LowpolyMutation::PaintStroke { object_id: object_id.into(), layer_index, runs }
}

pub fn apply(projection: &mut LowpolySnapshot, object_id: &str, layer_index: usize, runs: &[PixelRun]) {
    if let Some(object) = crate::artifacts::lowpoly::engine::object_mut(projection, object_id) {
        if let Some(layer) = object.paint_layers.get_mut(layer_index) {
            crate::artifacts::lowpoly::mutations::apply_pixel_runs(&mut layer.pixels, runs);
        }
    }
}
//#endregion 🔖️Mutation
