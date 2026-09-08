//! 🔁 `update-camera` payload — document-level scalar facet: `CameraJson { x, y, zoom }` is a
//! single inseparable viewport facet, never meaningfully set one-field-at-a-time
//! (`📓️derivation-rules.md` rule 1's `update-<facet>` exception).
//!
//! Directory kept at its pre-migration `🎛️set-camera` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::diff::Generation3dDiff;
use crate::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_semio_framework_os_flow::CameraJson;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️UpdateCamera
/// 🔁 Whole-artifact scope — the fixture has exactly one camera.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateCamera {
    pub camera: CameraJson,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for UpdateCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "camera", kind: "update-camera", record: "UpdatedCamera" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::mutations::update_camera::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::mutations::update_camera::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Update camera".to_string()
    }
}
//#endregion 🔖️UpdateCamera
