//! 🔺️ `reorder-layers` — sparse diff construction; an out-of-range BASE `from` is
//! `mutation.target-missing`.

use super::mutation::ReorderLayers;
use crate::artifacts::din4108::diff::Din4108LayerList;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ReorderLayers, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if payload.from >= base.layers.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer #{} does not exist.", payload.from), [payload.from.to_string()]);
    }
    let mut layers = base.layers.clone();
    let item = layers.remove(payload.from);
    let at = payload.to.min(layers.len());
    if at == payload.from {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer #{} is already at that position.", payload.from));
    }
    layers.insert(at, item);
    protocol::MutationOutcome::new(Din4108Diff { layers: Some(Din4108LayerList { values: layers }), ..Default::default() })
}
//#endregion 🔖️Diff
