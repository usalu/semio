//! 🔺️ `insert-layer` — sparse diff construction. `Din4108Diff::layers` is a whole-list-per-diff
//! wrapper (`Din4108LayerList`), not a sparse triple — every layer mutation rebuilds the full
//! ordered `values` vec from `base` and wraps it.

use super::mutation::InsertLayer;
use crate::artifacts::din4108::diff::Din4108LayerList;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &InsertLayer, base: &Din4108Snapshot) -> Din4108Diff {
    let mut layers = base.layers.clone();
    let at = payload.index.min(layers.len());
    layers.insert(at, payload.layer.clone());
    Din4108Diff { layers: Some(Din4108LayerList { values: layers }), ..Default::default() }
}
//#endregion 🔖️Diff
