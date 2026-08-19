//! 🔺️ `remove-paint-layer` — sparse diff construction (delegates to the existing remove-paint-layer
//! field-delta constructor). Error `target-missing` when the owning object or the layer index is
//! absent.

use super::mutation::RemovePaintLayer;
use crate::artifacts::lowpoly::diff::diff_remove_paint_layer;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &RemovePaintLayer, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.object_id), [payload.object_id.clone()]);
    };
    if payload.index >= object.paint_layers.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Paint layer {} does not exist on object \"{}\".", payload.index, payload.object_id), [payload.object_id.clone()]);
    }
    protocol::MutationOutcome::new(diff_remove_paint_layer(payload.object_id.clone(), payload.index))
}
//#endregion 🔖️Diff
