//! 🔺️ `change-storey-height` diff: patches `height` of the addressed storey row.

use crate::change_storey_height::ChangeStoreyHeight;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993StoreyDelta, En1993StoreyPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeStoreyHeight, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_height.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.storeys.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "storey", Vec::<String>::new());
    };
    if row.height == payload.new_height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The storey already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { storeys: En1993StoreyDelta::modification(payload.id.clone(), En1993StoreyPatch { height: Some(payload.new_height), ..Default::default() }), ..Default::default() })
}
