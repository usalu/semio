//! 🔁 `update-camera` payload — document-level scalar facet: `CameraJson { x, y, zoom }` is a
//! single inseparable viewport facet, never meaningfully set one-field-at-a-time
//! (`📓️derivation-rules.md` rule 1's `update-<facet>` exception).
//!
//! Directory kept at its pre-migration `🎛set-camera` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::CameraJson;
use serde::{Deserialize, Serialize};

//#region 🔖️UpdateCamera
/// 🔁 Whole-artifact scope — the fixture has exactly one camera.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCamera {
    pub camera: CameraJson}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for UpdateCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "camera", kind: "update-camera", record: "UpdatedCamera" };

    fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::update_camera::diff::diff(self, base)
    }

    fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::update_camera::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Update camera".to_string()
    }
}
//#endregion 🔖️UpdateCamera
