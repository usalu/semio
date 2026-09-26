//! Diff for `remove-self-weight-elements`.
use super::RemoveSelfWeightElements;
use crate::artifact_schema::diff::En1991SelfWeightElementsList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &RemoveSelfWeightElements, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.self_weight_elements.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Index out of range.", Vec::<String>::new());
    }
    let mut values = base.self_weight_elements.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1991Diff { self_weight_elements: Some(En1991SelfWeightElementsList { values }), ..Default::default() })
}
