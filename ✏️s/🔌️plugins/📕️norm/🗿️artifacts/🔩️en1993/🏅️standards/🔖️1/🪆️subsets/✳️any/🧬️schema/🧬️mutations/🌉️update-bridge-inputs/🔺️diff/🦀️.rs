//! 🔺️ `upsert-bridge-fatigue` — sparse diff construction.

use super::UpdateBridgeInputs;
use crate::diff::En1993BridgeList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateBridgeInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.bridge_fatigue.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.bridge_fatigue_item.id) {
        if values[idx] == payload.bridge_fatigue_item {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.bridge_fatigue_item.clone();
    } else {
        values.push(payload.bridge_fatigue_item.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: Some(En1993BridgeList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
