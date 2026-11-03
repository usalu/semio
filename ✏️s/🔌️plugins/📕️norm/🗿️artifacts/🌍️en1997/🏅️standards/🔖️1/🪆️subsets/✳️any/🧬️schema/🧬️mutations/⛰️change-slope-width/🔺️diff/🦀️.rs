//! 🔺️ `change-slope-width` diff: patches `width` of the addressed slope row.

use crate::change_slope_width::ChangeSlopeWidth;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997SlopesPatch, En1997SlopesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeSlopeWidth, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_width.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.slopes.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("slope {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.width == payload.new_width {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The slope already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { slopes: Some(En1997SlopesRows::modification(&payload.id, En1997SlopesPatch { width: Some(payload.new_width), ..Default::default() })), ..Default::default() })
}
