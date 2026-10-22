//! 📷 Generation2d mutation — `UpdateCamera`: sets the fixture's canvas camera facet (x/y/zoom are
//! always changed together by a pan/zoom gesture, never one field at a time — the `update` facet
//! exception, not a `change` scalar setter).

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️UpdateCamera
/// 📷 `update-camera` payload — the fixture's new camera position/zoom.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateCamera {
    pub camera: CameraJson,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_camera<Op: From<UpdateCamera>>(camera: CameraJson) -> Op {
    Op::from(UpdateCamera { camera })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for UpdateCamera
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<UpdateCamera>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "camera", kind: "update-camera", record: "UpdatedCamera" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update camera to ({}, {}, zoom {})", self.camera.x, self.camera.y, self.camera.zoom), &format!("Kamera auf ({}, {}, Zoom {}) aktualisieren", self.camera.x, self.camera.y, self.camera.zoom))
    }
}

impl<Op> protocol::ColdMutationKind<Generation2dSnapshot, Op> for UpdateCamera
where
    Op: protocol::ColdMutation<Generation2dSnapshot, ReferenceDiff = Generation2dDiff> + From<UpdateCamera>,
{
    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::update_camera::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_camera::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️UpdateCamera
