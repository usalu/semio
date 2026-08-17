//! 🔺️ `change-layer-lambda` — sparse diff construction; an out-of-range BASE index is
//! `mutation.target-missing`.

use super::mutation::ChangeLayerLambda;
use crate::artifacts::din4108::diff::Din4108LayerList;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeLayerLambda, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_lambda_w_mk.is_finite() || payload.new_lambda_w_mk <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer lambda must be a positive finite number, got {}.", payload.new_lambda_w_mk), [payload.index.to_string()]);
    }
    let Some(layer) = base.layers.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer #{} does not exist.", payload.index), [payload.index.to_string()]);
    };
    if layer.lambda_w_mk == payload.new_lambda_w_mk {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer #{} lambda is already {}.", payload.index, payload.new_lambda_w_mk));
    }
    let mut layers = base.layers.clone();
    layers[payload.index].lambda_w_mk = payload.new_lambda_w_mk;
    protocol::MutationOutcome::new(Din4108Diff { layers: Some(Din4108LayerList { values: layers }), ..Default::default() })
}
//#endregion 🔖️Diff
