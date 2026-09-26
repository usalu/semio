use super::InsertBridgeFatigue;
use crate::diff::En1993BridgeList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertBridgeFatigue, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.bridge_fatigue.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.bridge_fatigue_item.clone());
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: Some(En1993BridgeList { values }), ..Default::default() })
}
