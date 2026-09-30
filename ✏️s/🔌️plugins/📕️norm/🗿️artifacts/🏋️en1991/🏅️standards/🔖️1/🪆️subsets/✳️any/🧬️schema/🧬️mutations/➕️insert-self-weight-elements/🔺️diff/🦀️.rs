//! Diff for `insert-self-weight-elements`.
use super::InsertSelfWeightElements;
use crate::artifact_schema::diff::En1991SelfWeightElementsList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &InsertSelfWeightElements, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index > base.self_weight_elements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    let mut values = base.self_weight_elements.clone();
    values.insert(payload.index, payload.item.clone());
    protocol::MutationOutcome::new(En1991Diff { self_weight_elements: Some(En1991SelfWeightElementsList { values }), ..Default::default() })
}
