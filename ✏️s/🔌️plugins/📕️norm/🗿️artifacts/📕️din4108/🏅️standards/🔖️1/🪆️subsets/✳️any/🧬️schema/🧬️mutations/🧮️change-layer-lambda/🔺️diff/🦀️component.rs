//! 🔺️ `change-layer-lambda` — sparse diff construction; an out-of-range BASE index is a no-op
//! clone (nothing at that position to change).

use super::mutation::ChangeLayerLambda;
use crate::artifacts::din4108::diff::Din4108LayerList;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeLayerLambda, base: &Din4108Snapshot) -> Din4108Diff {
    let mut layers = base.layers.clone();
    if let Some(layer) = layers.get_mut(payload.index) {
        layer.lambda_w_mk = payload.new_lambda_w_mk;
    }
    Din4108Diff { layers: Some(Din4108LayerList { values: layers }), ..Default::default() }
}
//#endregion 🔖️Diff
