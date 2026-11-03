//! 🔺️ `change-slope-circle-radius` diff: patches `circle_radius` of the addressed slope row.

use crate::change_slope_circle_radius::ChangeSlopeCircleRadius;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997SlopesPatch, En1997SlopesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeSlopeCircleRadius, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_circle_radius.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.slopes.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("slope {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.circle_radius == payload.new_circle_radius {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The slope already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { slopes: Some(En1997SlopesRows::modification(&payload.id, En1997SlopesPatch { circle_radius: Some(payload.new_circle_radius), ..Default::default() })), ..Default::default() })
}
