//! 👁️ Exact visibility override assignment or removal for one map layer.

use super::super::{Gis2dConfig, Gis2dConfigDelta, Gis2dConfigDiff, Gis2dConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-layer-visibility")]
pub struct SetLayerVisibility {
    pub layer_id: String,
    #[serde(deserialize_with = "super::super::required_nullable")]
    pub visible: Option<bool>,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<Gis2dConfig, Gis2dConfigMutation> for SetLayerVisibility {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "layer-visibility", kind: "set-layer-visibility", record: "SetLayerVisibility" };
    fn diff(&self, base: &Gis2dConfig) -> MutationOutcome<Gis2dConfigDiff> {
        if base.layer_visibility.get(&self.layer_id).copied() == self.visible {
            return MutationOutcome::empty().warn("mutation.no-op", "Layer visibility override is already at the requested value.");
        }
        MutationOutcome::new(Gis2dConfigDelta { layer_visibility: [(self.layer_id.clone(), self.visible)].into(), ..Default::default() }.into())
    }
    fn inverse(&self, base: &Gis2dConfig) -> Vec<Gis2dConfigMutation> {
        vec![Self { layer_id: self.layer_id.clone(), visible: base.layer_visibility.get(&self.layer_id).copied() }.into()]
    }
    fn label(&self) -> String { format!("Set layer visibility {}", self.layer_id) }
    fn target(&self) -> Vec<String> { vec!["layerVisibility".into(), self.layer_id.clone()] }
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_payload_metadata_codecs_and_inverse_match_the_neutral_fixture() {
        super::super::super::direct_mutation_tests::assert_leaf::<SetLayerVisibility>(0, Gis2dConfigMutation::SetLayerVisibility, include_str!("🔣️.json"));
    }
}
//#endregion 🧪️Contracts
