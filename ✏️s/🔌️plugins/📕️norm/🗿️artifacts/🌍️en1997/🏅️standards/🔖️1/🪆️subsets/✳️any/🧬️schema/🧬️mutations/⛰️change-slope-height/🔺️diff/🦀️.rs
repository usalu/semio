//! 🔺️ `change-slope-height` diff: patches `height` of the addressed slope row.

use crate::change_slope_height::ChangeSlopeHeight;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997SlopesPatch, En1997SlopesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeSlopeHeight, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_height.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.slopes.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("slope {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.height == payload.new_height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The slope already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { slopes: Some(En1997SlopesRows::modification(&payload.id, En1997SlopesPatch { height: Some(payload.new_height), ..Default::default() })), ..Default::default() })
}
