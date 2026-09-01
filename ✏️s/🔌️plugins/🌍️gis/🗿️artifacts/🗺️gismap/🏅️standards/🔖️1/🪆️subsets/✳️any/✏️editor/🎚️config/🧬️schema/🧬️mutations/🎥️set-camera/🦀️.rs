//! 🎥️ Camera payload and sparse camera configuration change.

use super::super::{Gis2dConfig, Gis2dConfigDelta, Gis2dConfigDiff, Gis2dConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-camera")]
pub struct SetCamera { pub camera_json: String }
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<Gis2dConfig, Gis2dConfigMutation> for SetCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &Gis2dConfig) -> MutationOutcome<Gis2dConfigDiff> {
        if base.camera_json == self.camera_json { return MutationOutcome::empty().warn("mutation.no-op", "Camera is already at the requested position."); }
        MutationOutcome::new(Gis2dConfigDelta { camera_json: Some(self.camera_json.clone()), ..Default::default() }.into())
    }
    fn inverse(&self, base: &Gis2dConfig) -> Vec<Gis2dConfigMutation> { vec![Self { camera_json: base.camera_json.clone() }.into()] }
    fn label(&self) -> String { "Set camera".into() }
    fn target(&self) -> Vec<String> { vec!["cameraJson".into()] }
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_payload_metadata_codecs_and_inverse_match_the_neutral_fixture() {
        super::super::super::direct_mutation_tests::assert_leaf::<SetCamera>(2, Gis2dConfigMutation::SetCamera, include_str!("🔣️.json"));
    }
}
//#endregion 🧪️Contracts
