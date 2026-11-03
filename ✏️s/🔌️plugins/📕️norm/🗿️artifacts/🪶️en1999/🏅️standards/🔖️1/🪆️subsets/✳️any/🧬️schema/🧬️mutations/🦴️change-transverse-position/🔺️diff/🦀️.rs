//! 🔺️ `change-transverse-position` diff: patches `position` of the addressed transverse stiffener row.

use crate::change_transverse_position::ChangeTransversePosition;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999TransverseStiffenersPatch, En1999TransverseStiffenersRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeTransversePosition, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_position.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.transverse_stiffeners.iter().find(|row| row.id == payload.transverse_stiffener_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "transverse-stiffener", Vec::<String>::new());
    };
    if r0.position == payload.new_position {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The transverse stiffener already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { transverse_stiffeners: Some(En1999TransverseStiffenersRows::modification(&payload.transverse_stiffener_id, En1999TransverseStiffenersPatch { position: Some(payload.new_position), ..Default::default() })), ..Default::default() })
}
