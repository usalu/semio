//! ✏️ `change-seismic-slope-governing-layer-id` diff: patches `governing_layer_id` of the addressed seismic_slope row.

use crate::change_seismic_slope_governing_layer_id::ChangeSeismicSlopeGoverningLayerId;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SlopeDelta, En1998SlopePatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSeismicSlopeGoverningLayerId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.slopes.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("seismic_slope {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.governing_layer_id == payload.new_governing_layer_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The seismic_slope already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { slopes: En1998SlopeDelta::modification(&payload.id, En1998SlopePatch { governing_layer_id: Some(payload.new_governing_layer_id.clone()), ..Default::default() }), ..Default::default() })
}
