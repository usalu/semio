//! 🔺️ `change-slope-governing-layer-id` diff: patches `governing_layer_id` of the addressed slope row.

use crate::change_slope_governing_layer_id::ChangeSlopeGoverningLayerId;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997SlopesPatch, En1997SlopesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeSlopeGoverningLayerId, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(r0) = base.slopes.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("slope {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.governing_layer_id == payload.new_governing_layer_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The slope already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { slopes: Some(En1997SlopesRows::modification(&payload.id, En1997SlopesPatch { governing_layer_id: Some(payload.new_governing_layer_id.clone()), ..Default::default() })), ..Default::default() })
}
