//! 🌀 `remove-dynamic-element` diff: removes the addressed dynamic element row.

use crate::remove_dynamic_element::RemoveDynamicElement;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991DynamicElementDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveDynamicElement, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.dynamic_elements.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Dynamic element {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { dynamic_elements: En1991DynamicElementDelta::removal(&base.dynamic_elements, index), ..Default::default() })
}
