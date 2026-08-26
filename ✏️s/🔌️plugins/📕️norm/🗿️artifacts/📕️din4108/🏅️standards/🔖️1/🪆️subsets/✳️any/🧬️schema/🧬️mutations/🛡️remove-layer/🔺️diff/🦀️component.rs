//! 🔺️ `remove-layer` — sparse diff construction with strict BASE-index validation.

use super::mutation::RemoveLayer;
use crate::artifacts::din4108::diff::Din4108LayerList;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemoveLayer, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let mut layers = base.layers.clone();
    if payload.index >= layers.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer #{} does not exist.", payload.index), [payload.index.to_string()]);
    }
    layers.remove(payload.index);
    protocol::MutationOutcome::new(Din4108Diff { layers: Some(Din4108LayerList { values: layers }), ..Default::default() })
}
//#endregion 🔖️Diff
