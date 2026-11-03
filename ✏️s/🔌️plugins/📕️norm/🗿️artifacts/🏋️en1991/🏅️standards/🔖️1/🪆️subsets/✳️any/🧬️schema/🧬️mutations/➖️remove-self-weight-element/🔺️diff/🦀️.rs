//! ➖️ `remove-self-weight-element` diff: removes the addressed self weight element row.

use crate::remove_self_weight_element::RemoveSelfWeightElement;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SelfWeightElementDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveSelfWeightElement, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.self_weight_elements.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("self weight element {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { self_weight_elements: En1991SelfWeightElementDelta::removal(&base.self_weight_elements, index), ..Default::default() })
}
