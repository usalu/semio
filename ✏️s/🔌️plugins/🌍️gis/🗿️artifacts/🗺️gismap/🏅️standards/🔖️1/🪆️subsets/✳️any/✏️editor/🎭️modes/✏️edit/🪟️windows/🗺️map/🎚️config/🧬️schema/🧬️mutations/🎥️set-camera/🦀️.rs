//! 🎥️ Camera payload and sparse camera configuration change.

use super::super::{MapWindowConfig, MapWindowConfigDelta, MapWindowConfigDiff, MapWindowConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-camera")]
pub struct SetCamera {
    pub camera_json: String,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<MapWindowConfig, MapWindowConfigMutation> for SetCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &MapWindowConfig) -> MutationOutcome<MapWindowConfigDiff> {
        if base.camera_json == self.camera_json {
            return MutationOutcome::empty().warn("mutation.no-op", "Camera is already at the requested position.");
        }
        MutationOutcome::new(MapWindowConfigDelta { camera_json: Some(self.camera_json.clone()), ..Default::default() }.into())
    }
    fn inverse(&self, base: &MapWindowConfig) -> Vec<MapWindowConfigMutation> {
        vec![Self { camera_json: base.camera_json.clone() }.into()]
    }
    fn label(&self) -> String {
        "Set camera".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["cameraJson".into()]
    }
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Contracts
