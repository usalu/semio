//! 🔺️ Diff for `remove-self-weight-elements`.
use super::RemoveSelfWeightElements;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991SelfWeightElementDelta};
pub fn diff(payload: &RemoveSelfWeightElements, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.self_weight_elements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1991Diff { self_weight_elements: En1991SelfWeightElementDelta::removal(&base.self_weight_elements, payload.index), ..Default::default() })
}
