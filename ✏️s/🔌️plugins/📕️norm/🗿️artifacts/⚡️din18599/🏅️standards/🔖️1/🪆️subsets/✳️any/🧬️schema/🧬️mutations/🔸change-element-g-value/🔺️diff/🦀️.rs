//! 🔺️ `change-element-g-value` sparse diff.

use crate::change_element_g_value::ChangeElementGValue;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ElementsRows, Din18599ElementsPatch};

pub fn diff(payload: &ChangeElementGValue, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_g_value.is_finite() || payload.new_g_value < 0.0 || payload.new_g_value > 1.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "g-value must lie from 0 to 1.", Vec::<String>::new());
    }
    let Some(row) = base.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.element_id), [payload.element_id.clone()]);
    };
    if row.g_value == payload.new_g_value {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Element g value already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        elements: Some(Din18599ElementsRows::modification(&payload.element_id, Din18599ElementsPatch { g_value: Some(payload.new_g_value), ..Default::default() })),
        ..Default::default()
    })
}
