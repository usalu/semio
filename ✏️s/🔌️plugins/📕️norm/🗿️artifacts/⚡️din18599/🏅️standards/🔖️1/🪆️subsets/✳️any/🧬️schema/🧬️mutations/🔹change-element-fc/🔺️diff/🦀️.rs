//! 🔺️ `change-element-fc` sparse diff.

use crate::change_element_fc::ChangeElementFc;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ElementsRows, Din18599ElementsPatch};

pub fn diff(payload: &ChangeElementFc, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_fc.is_finite() || payload.new_fc < 0.0 || payload.new_fc > 1.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "fc must lie from 0 to 1.", Vec::<String>::new());
    }
    let Some(row) = base.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.element_id), [payload.element_id.clone()]);
    };
    if row.fc == payload.new_fc {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Element fc already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        elements: Some(Din18599ElementsRows::modification(&payload.element_id, Din18599ElementsPatch { fc: Some(payload.new_fc), ..Default::default() })),
        ..Default::default()
    })
}
