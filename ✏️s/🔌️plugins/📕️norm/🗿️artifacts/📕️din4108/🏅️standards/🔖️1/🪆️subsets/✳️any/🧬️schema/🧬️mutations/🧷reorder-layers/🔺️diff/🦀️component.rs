//! 🔺️ `reorder-layers` — sparse diff construction; an out-of-range BASE `from` is a no-op clone.

use super::mutation::ReorderLayers;
use crate::artifacts::din4108::diff::Din4108LayerList;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReorderLayers, base: &Din4108Snapshot) -> Din4108Diff {
    let mut layers = base.layers.clone();
    if payload.from < layers.len() {
        let item = layers.remove(payload.from);
        let at = payload.to.min(layers.len());
        layers.insert(at, item);
    }
    Din4108Diff { layers: Some(Din4108LayerList { values: layers }), ..Default::default() }
}
//#endregion 🔖️Diff
