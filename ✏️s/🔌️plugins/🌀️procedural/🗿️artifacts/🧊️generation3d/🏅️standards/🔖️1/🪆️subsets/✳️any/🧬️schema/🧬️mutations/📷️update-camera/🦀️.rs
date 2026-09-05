//! 🔁 `update-camera` payload — document-level scalar facet: `CameraJson { x, y, zoom }` is a
//! single inseparable viewport facet, never meaningfully set one-field-at-a-time
//! (`📓️derivation-rules.md` rule 1's `update-<facet>` exception).
//!
//! Directory kept at its pre-migration `🎛️set-camera` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use flow::CameraJson;
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
        crate::artifacts::generation3d::mutations::update_camera::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::artifacts::generation3d::mutations::update_camera::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Update camera".to_string()
    }
}
//#endregion 🔖️UpdateCamera
