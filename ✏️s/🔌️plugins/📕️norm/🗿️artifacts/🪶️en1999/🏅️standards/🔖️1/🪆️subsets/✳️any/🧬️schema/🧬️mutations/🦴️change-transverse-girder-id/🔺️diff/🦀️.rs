//! 🔺️ `change-transverse-girder-id` diff: patches `girder_id` of the addressed transverse stiffener row.

use crate::change_transverse_girder_id::ChangeTransverseGirderId;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999TransverseStiffenersPatch, En1999TransverseStiffenersRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeTransverseGirderId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.transverse_stiffeners.iter().find(|row| row.id == payload.transverse_stiffener_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "transverse-stiffener", Vec::<String>::new());
    };
    if r0.girder_id == payload.new_girder_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The transverse stiffener already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { transverse_stiffeners: Some(En1999TransverseStiffenersRows::modification(&payload.transverse_stiffener_id, En1999TransverseStiffenersPatch { girder_id: Some(payload.new_girder_id.clone()), ..Default::default() })), ..Default::default() })
}
