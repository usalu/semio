//! Diff for `change-self-weight-assumed-gk`.
use super::ChangeSelfWeightAssumedGk;
use crate::artifact_schema::diff::En1991SelfWeightElementsList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeSelfWeightAssumedGk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.self_weight_elements.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Index out of range.", Vec::<String>::new());
    }
    if base.self_weight_elements[payload.index].assumed_gk == payload.new_assumed_gk {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    let mut values = base.self_weight_elements.clone();
    values[payload.index].assumed_gk = payload.new_assumed_gk;
    protocol::MutationOutcome::new(En1991Diff { self_weight_elements: Some(En1991SelfWeightElementsList { values }), ..Default::default() })
}
