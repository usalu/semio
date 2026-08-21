//! 🔺️ `update-camera` sparse diff construction.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::update_camera::mutation::UpdateCamera;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta touching only the camera field. Whole-artifact scope — there
/// is exactly one camera, so no missing-target case exists here.
pub async fn diff(payload: &UpdateCamera, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
    if !payload.camera.x.is_finite() || !payload.camera.y.is_finite() || !payload.camera.zoom.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Camera position or zoom is not finite.", Vec::<String>::new());
    }
    if base.fixture.camera == payload.camera {
        return protocol::MutationOutcome::new(Procedural3dDiff::default()).warn("mutation.no-op", "Camera is already in the requested state.");
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff::default(), Some(payload.camera.clone()), None))
}
