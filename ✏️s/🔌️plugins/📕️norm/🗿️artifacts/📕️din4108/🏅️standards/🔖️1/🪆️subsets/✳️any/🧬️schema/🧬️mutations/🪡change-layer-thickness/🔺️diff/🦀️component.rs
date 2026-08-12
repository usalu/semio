//! 🔺️ `change-layer-thickness` — sparse diff construction; an out-of-range BASE index is a no-op
//! clone (nothing at that position to change).

use super::mutation::ChangeLayerThickness;
use crate::artifacts::din4108::diff::Din4108LayerList;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeLayerThickness, base: &Din4108Snapshot) -> Din4108Diff {
    let mut layers = base.layers.clone();
    if let Some(layer) = layers.get_mut(payload.index) {
        layer.thickness_m = payload.new_thickness_m;
    }
    Din4108Diff { layers: Some(Din4108LayerList { values: layers }), ..Default::default() }
}
//#endregion 🔖️Diff
