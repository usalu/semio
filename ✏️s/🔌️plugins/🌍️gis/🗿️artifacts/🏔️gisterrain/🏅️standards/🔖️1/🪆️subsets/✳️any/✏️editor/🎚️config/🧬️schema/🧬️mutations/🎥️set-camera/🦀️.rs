//! 🎥️ Camera payload and sparse GIS 3D configuration behavior.
use super::super::{Gis3dConfig, Gis3dConfigDelta, Gis3dConfigDiff, Gis3dConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-camera")]
pub struct SetCamera { pub camera_json: String }
//#endregion 🧬️Payload
//#region ⚙️Behavior
impl MutationKind<Gis3dConfig, Gis3dConfigMutation> for SetCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &Gis3dConfig) -> MutationOutcome<Gis3dConfigDiff> { if base.camera_json == self.camera_json { MutationOutcome::empty().warn("mutation.no-op", "Camera is already at the requested position.") } else { MutationOutcome::new(Gis3dConfigDelta { camera_json: Some(self.camera_json.clone()), ..Default::default() }.into()) } }
    fn inverse(&self, base: &Gis3dConfig) -> Vec<Gis3dConfigMutation> { vec![Self { camera_json: base.camera_json.clone() }.into()] }
    fn label(&self) -> String { "Set camera".into() }
    fn target(&self) -> Vec<String> { vec!["cameraJson".into()] }
}
//#endregion ⚙️Behavior
//#region 🧪️Contracts
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff};
    #[test]
    fn direct_leaf_neutral_schema_codec_and_outcome_laws() {
        super::super::super::direct_leaf_contracts::assert_leaf_contract::<SetCamera>("camera", include_str!("🔣️.json"));
    }
    #[test]
    fn sparse_camera_inverse_and_codecs_preserve_locale() {
        let base = Gis3dConfig { camera_json: "default".into(), locale: "de-DE".into() };
        let mutation = Gis3dConfigMutation::SetCamera(SetCamera { camera_json: "next".into() });
        let next = mutation.diff(&base).diff().apply(&base).expect("apply");
        assert_eq!(next.locale, base.locale);
        assert_eq!(mutation.inverse(&base)[0].diff(&next).diff().apply(&next).expect("inverse"), base);
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}
//#endregion 🧪️Contracts
