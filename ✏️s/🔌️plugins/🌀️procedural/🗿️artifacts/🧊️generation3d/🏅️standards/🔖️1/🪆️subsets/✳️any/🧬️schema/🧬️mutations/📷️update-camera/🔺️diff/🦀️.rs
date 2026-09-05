//! 🔺️ `update-camera` sparse diff construction.

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::generation3d::mutations::update_camera::UpdateCamera;
use crate::artifacts::generation3d::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta touching only the camera field. Whole-artifact scope — there
/// is exactly one camera, so no missing-target case exists here.
pub fn diff(payload: &UpdateCamera, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if !payload.camera.x.is_finite() || !payload.camera.y.is_finite() || !payload.camera.zoom.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Camera position or zoom is not finite.", Vec::<String>::new());
    }
    if base.fixture.camera == payload.camera {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warn("mutation.no-op", "Camera is already in the requested state.");
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff::default(), Some(payload.camera.clone()), None))
}
