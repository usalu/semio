//! 🎥️ Camera payload and sparse GIS 2D presence replacement.

use super::super::{Gis2dPresence, Gis2dPresenceDelta, Gis2dPresenceDiff, Gis2dPresenceMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-camera")]
pub struct SetCamera {
    pub camera_json: String,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<Gis2dPresence, Gis2dPresenceMutation> for SetCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };

    fn diff(&self, base: &Gis2dPresence) -> MutationOutcome<Gis2dPresenceDiff> {
        if base.camera_json == self.camera_json {
            return MutationOutcome::empty().warn("mutation.no-op", "Presence snapshot is already identical to the requested replacement.");
        }
        MutationOutcome::new(Gis2dPresenceDelta { camera_json: Some(self.camera_json.clone()) }.into())
    }

    fn inverse(&self, base: &Gis2dPresence) -> Vec<Gis2dPresenceMutation> {
        vec![Self { camera_json: base.camera_json.clone() }.into()]
    }

    fn label(&self) -> String { "Set camera".into() }

    fn target(&self) -> Vec<String> { vec!["cameraJson".into()] }
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_payload_metadata_text_binary_and_inverse_match_neutral_fixture() {
        super::super::super::direct_mutation_tests::assert_set_camera_leaf(include_str!("🔣️.json"));
    }
}
//#endregion 🧪️Contracts
