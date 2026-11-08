//! 🔺️ `remove-element` sparse diff: one removed row.

use crate::remove_element::RemoveElement;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ElementsRows};

pub fn diff(payload: &RemoveElement, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    let Some(index) = base.elements.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Din18599Diff { elements: Some(Din18599ElementsRows::removal(&base.elements, index)), ..Default::default() })
}
