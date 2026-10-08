//! 🔺️ Diff for `change-self-weight-assumed-gk`.
use super::ChangeSelfWeightAssumedGk;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991SelfWeightElementDelta, En1991SelfWeightElementPatch};
pub fn diff(payload: &ChangeSelfWeightAssumedGk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.self_weight_elements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    if base.self_weight_elements[payload.index].assumed_gk == payload.new_assumed_gk {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    let self_weight = &base.self_weight_elements[payload.index];
    protocol::MutationOutcome::new(En1991Diff { self_weight_elements: En1991SelfWeightElementDelta::modification(&self_weight.id, En1991SelfWeightElementPatch { assumed_gk: Some(payload.new_assumed_gk), ..Default::default() }), ..Default::default() })
}
