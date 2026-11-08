//! 🔺️ `change-element-orientation-deg` sparse diff.

use crate::change_element_orientation_deg::ChangeElementOrientationDeg;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ElementsRows, Din18599ElementsPatch};

pub fn diff(payload: &ChangeElementOrientationDeg, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_orientation_deg.is_finite() || payload.new_orientation_deg < 0.0 || payload.new_orientation_deg > 360.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "orientation-deg must lie from 0 to 360.", Vec::<String>::new());
    }
    let Some(row) = base.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.element_id), [payload.element_id.clone()]);
    };
    if row.orientation_deg == payload.new_orientation_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Element orientation deg already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        elements: Some(Din18599ElementsRows::modification(&payload.element_id, Din18599ElementsPatch { orientation_deg: Some(payload.new_orientation_deg), ..Default::default() })),
        ..Default::default()
    })
}
