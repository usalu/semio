//! 📷 Generation2d mutation — `UpdateCamera`: sets the fixture's canvas camera facet (x/y/zoom are
//! always changed together by a pan/zoom gesture, never one field at a time — the `update` facet
//! exception, not a `change` scalar setter).

use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use flow::CameraJson;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️UpdateCamera
/// 📷 `update-camera` payload — the fixture's new camera position/zoom.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateCamera {
    pub camera: CameraJson,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_camera(camera: CameraJson) -> Generation2dMutation {
    Generation2dMutation::UpdateCamera(UpdateCamera { camera })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for UpdateCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "camera", kind: "update-camera", record: "UpdatedCamera" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Update camera to ({}, {}, zoom {})", self.camera.x, self.camera.y, self.camera.zoom)
    }
}
//#endregion 🔖️UpdateCamera
