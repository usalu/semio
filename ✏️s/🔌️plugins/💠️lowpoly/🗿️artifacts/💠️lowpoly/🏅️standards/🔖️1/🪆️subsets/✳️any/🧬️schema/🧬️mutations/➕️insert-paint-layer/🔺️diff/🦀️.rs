//! 🔺️ `insert-paint-layer` — sparse diff construction (delegates to the existing add-paint-layer
//! field-delta constructor). Error `target-missing` when the owning object is absent, Warning
//! `clamped` when the requested index is out of range for the layer list.

use super::InsertPaintLayer;
use crate::artifacts::lowpoly::diff::diff_add_paint_layer;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &InsertPaintLayer, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.object_id), [payload.object_id.clone()]);
    };
    let clamped_index = payload.index.min(object.paint_layers.len());
    let outcome = protocol::MutationOutcome::new(diff_add_paint_layer(payload.object_id.clone(), clamped_index, payload.layer.clone()));
    if clamped_index != payload.index {
        outcome.warn("mutation.clamped", format!("Paint layer index {} clamped to {} on object \"{}\".", payload.index, clamped_index, payload.object_id))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
