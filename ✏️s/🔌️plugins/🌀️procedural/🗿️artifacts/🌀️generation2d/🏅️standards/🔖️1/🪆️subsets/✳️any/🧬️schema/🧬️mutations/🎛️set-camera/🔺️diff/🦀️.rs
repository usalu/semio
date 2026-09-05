//! 🔺️ Sparse diff builder for `UpdateCamera` — a real scalar-facet write on the fixture (never a
//! whole-snapshot capture).

use crate::artifacts::generation2d::diff::{diff_fixture_from_helpers, LayoutDiff, Generation2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::generation2d::Generation2dSnapshot;

pub fn diff(payload: &super::UpdateCamera, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let camera = &payload.camera;
    if !camera.x.is_finite() || !camera.y.is_finite() || !camera.zoom.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Camera x/y/zoom must be finite.".to_string(), Vec::<String>::new());
    }
    if base.fixture.camera == *camera {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Camera is already at the requested position.".to_string());
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff::default(), Some(camera.clone()), None))
}
