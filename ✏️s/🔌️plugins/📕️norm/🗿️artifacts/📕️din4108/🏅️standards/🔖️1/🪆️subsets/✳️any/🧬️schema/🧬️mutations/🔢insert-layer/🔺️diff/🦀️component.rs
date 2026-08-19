//! 🔺️ `insert-layer` — sparse diff construction. `Din4108Diff::layers` is a whole-list-per-diff
//! wrapper (`Din4108LayerList`), not a sparse triple — every layer mutation rebuilds the full
//! ordered `values` vec from `base` and wraps it. An out-of-range index clamps to the end with
//! `mutation.clamped`.

use super::mutation::InsertLayer;
use crate::artifacts::din4108::diff::Din4108LayerList;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &InsertLayer, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.layer.thickness_m.is_finite() || payload.layer.thickness_m <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer thickness must be a positive finite number, got {}.", payload.layer.thickness_m), Vec::<String>::new());
    }
    if !payload.layer.lambda_w_mk.is_finite() || payload.layer.lambda_w_mk <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer lambda must be a positive finite number, got {}.", payload.layer.lambda_w_mk), Vec::<String>::new());
    }
    let mut layers = base.layers.clone();
    let at = payload.index.min(layers.len());
    layers.insert(at, payload.layer.clone());
    let outcome = protocol::MutationOutcome::new(Din4108Diff { layers: Some(Din4108LayerList { values: layers }), ..Default::default() });
    if at != payload.index {
        outcome.warn("mutation.clamped", format!("Insert index {} was out of range; inserted at #{} instead.", payload.index, at))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
