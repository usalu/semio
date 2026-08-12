//! 🔺️ `update-camera` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::set_camera::mutation::UpdateCamera;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta touching only the camera field.
pub fn diff(payload: &UpdateCamera, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff::default(), Some(payload.camera.clone()), None)
}
