//! 🔺️ `change-element-tilt-deg` sparse diff.

use crate::change_element_tilt_deg::ChangeElementTiltDeg;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ElementsRows, Din18599ElementsPatch};

pub fn diff(payload: &ChangeElementTiltDeg, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_tilt_deg.is_finite() || payload.new_tilt_deg < 0.0 || payload.new_tilt_deg > 180.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "tilt-deg must lie from 0 to 180.", Vec::<String>::new());
    }
    let Some(row) = base.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.element_id), [payload.element_id.clone()]);
    };
    if row.tilt_deg == payload.new_tilt_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Element tilt deg already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        elements: Some(Din18599ElementsRows::modification(&payload.element_id, Din18599ElementsPatch { tilt_deg: Some(payload.new_tilt_deg), ..Default::default() })),
        ..Default::default()
    })
}
