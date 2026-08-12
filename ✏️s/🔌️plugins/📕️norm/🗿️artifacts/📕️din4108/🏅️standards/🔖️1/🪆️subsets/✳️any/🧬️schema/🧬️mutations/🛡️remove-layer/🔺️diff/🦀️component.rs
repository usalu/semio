//! 🔺️ `remove-layer` — sparse diff construction; an out-of-range BASE index is a no-op clone
//! (nothing to remove).

use super::mutation::RemoveLayer;
use crate::artifacts::din4108::diff::Din4108LayerList;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemoveLayer, base: &Din4108Snapshot) -> Din4108Diff {
    let mut layers = base.layers.clone();
    if payload.index < layers.len() {
        layers.remove(payload.index);
    }
    Din4108Diff { layers: Some(Din4108LayerList { values: layers }), ..Default::default() }
}
//#endregion 🔖️Diff
