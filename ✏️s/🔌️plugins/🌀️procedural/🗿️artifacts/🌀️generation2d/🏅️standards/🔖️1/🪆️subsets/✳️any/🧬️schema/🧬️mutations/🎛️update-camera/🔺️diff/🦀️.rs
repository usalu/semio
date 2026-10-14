//! 🔺️ Sparse diff builder for `UpdateCamera` — a real scalar-facet write on the fixture (never a
//! whole-snapshot capture).

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn diff(payload: &super::UpdateCamera, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let camera = &payload.camera;
    if !camera.x.is_finite() || !camera.y.is_finite() || !camera.zoom.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Camera x/y/zoom must be finite.".to_string(), Vec::<String>::new());
    }
    if base.host_snapshot.camera == *camera {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Camera is already at the requested position.".to_string());
    }
    protocol::MutationOutcome::new(Generation2dDiff { camera: Some(camera.clone()), ..Default::default() })
}
