use super::InsertBridgeFatigue;
use crate::diff::En1993BridgeList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertBridgeFatigue, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.bridge_fatigue.iter().any(|existing| existing.id == payload.bridge_fatigue_item.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Bridge fatigue item id {} already exists.", payload.bridge_fatigue_item.id), [payload.bridge_fatigue_item.id.clone()]);
    }
    let mut values = base.bridge_fatigue.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.bridge_fatigue_item.clone());
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: Some(En1993BridgeList { values }), ..Default::default() })
}
