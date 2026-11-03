//! 🌬 `change-wind-face-c-pe1` diff: patches `c_pe1` of the addressed wind face row.

use crate::change_wind_face_c_pe1::ChangeWindFaceCPe1;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindFaceDelta, En1991WindFacePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeWindFaceCPe1, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_c_pe1.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.wind_faces.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Wind face {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.c_pe1 == payload.new_c_pe1 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wind face already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { wind_faces: En1991WindFaceDelta::modification(&payload.id, En1991WindFacePatch { c_pe1: Some(payload.new_c_pe1), ..Default::default() }), ..Default::default() })
}
