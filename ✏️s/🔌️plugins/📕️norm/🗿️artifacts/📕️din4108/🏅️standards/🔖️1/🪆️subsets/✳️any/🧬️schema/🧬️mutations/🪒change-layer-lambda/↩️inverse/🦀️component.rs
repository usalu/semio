//! ↩️ `change-layer-lambda` — undo restores BASE's lambda at that index; out-of-range BASE
//! index ⇒ `Vec::new()`.

use super::mutation::ChangeLayerLambda;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeLayerLambda, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    match base.layers.get(payload.index) {
        Some(layer) => vec![Din4108Mutation::ChangeLayerLambda(ChangeLayerLambda { index: payload.index, new_lambda_w_mk: layer.lambda_w_mk })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
