//! 🔺️ Diff for `insert-self-weight-elements`.
use super::InsertSelfWeightElements;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991SelfWeightElementDelta};
pub fn diff(payload: &InsertSelfWeightElements, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index > base.self_weight_elements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    if base.self_weight_elements.iter().any(|existing| existing.id == payload.item.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {} already exists.", payload.item.id), [payload.item.id.clone()]);
    }
    protocol::MutationOutcome::new(En1991Diff { self_weight_elements: En1991SelfWeightElementDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
