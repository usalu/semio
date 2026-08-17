//! 🔺️ `change-layer-thickness` — sparse diff construction; an out-of-range BASE index is
//! `mutation.target-missing`.

use super::mutation::ChangeLayerThickness;
use crate::artifacts::din4108::diff::Din4108LayerList;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeLayerThickness, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_thickness_m.is_finite() || payload.new_thickness_m <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer thickness must be a positive finite number, got {}.", payload.new_thickness_m), [payload.index.to_string()]);
    }
    let Some(layer) = base.layers.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer #{} does not exist.", payload.index), [payload.index.to_string()]);
    };
    if layer.thickness_m == payload.new_thickness_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer #{} thickness is already {}.", payload.index, payload.new_thickness_m));
    }
    let mut layers = base.layers.clone();
    layers[payload.index].thickness_m = payload.new_thickness_m;
    protocol::MutationOutcome::new(Din4108Diff { layers: Some(Din4108LayerList { values: layers }), ..Default::default() })
}
//#endregion 🔖️Diff
