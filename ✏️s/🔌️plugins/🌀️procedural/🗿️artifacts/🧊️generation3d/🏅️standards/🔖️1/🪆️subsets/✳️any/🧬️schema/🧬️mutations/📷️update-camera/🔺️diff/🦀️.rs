//! 🔺️ `update-camera` sparse diff construction.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;

use crate::update_camera::UpdateCamera;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta touching only the camera field. Whole-artifact scope — there
/// is exactly one camera, so no missing-target case exists here.
pub fn diff(payload: &UpdateCamera, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if !payload.camera.x.is_finite() || !payload.camera.y.is_finite() || !payload.camera.zoom.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Camera position or zoom is not finite.", Vec::<String>::new());
    }
    if base.host_snapshot.camera == payload.camera {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warning("mutation.no-op", "Camera is already in the requested state.");
    }
    protocol::MutationOutcome::new(Generation3dDiff { camera: Some(payload.camera.clone()), ..Default::default() })
}
