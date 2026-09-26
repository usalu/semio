use super::RemoveBridgeFatigue;
use crate::diff::En1993BridgeList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveBridgeFatigue, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.bridge_fatigue.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("bridge-fatigue index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.bridge_fatigue.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { bridge_fatigue: Some(En1993BridgeList { values }), ..Default::default() })
}
