//! 📏️ Exact stroke-scale override assignment or removal for one map layer.

use super::super::{Gis2dConfig, Gis2dConfigDelta, Gis2dConfigDiff, Gis2dConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-layer-stroke-scale")]
pub struct SetLayerStrokeScale {
    pub layer_id: String,
    #[serde(deserialize_with = "super::super::required_nullable", serialize_with = "serialize_scale")]
    pub value: Option<f64>,
}

fn serialize_scale<S: serde::Serializer>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error> {
    if value.is_some_and(|value| !value.is_finite()) { return Err(serde::ser::Error::custom("layer stroke scale must be finite")); }
    serde::Serialize::serialize(value, serializer)
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<Gis2dConfig, Gis2dConfigMutation> for SetLayerStrokeScale {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "layer-stroke-scale", kind: "set-layer-stroke-scale", record: "SetLayerStrokeScale" };
    fn diff(&self, base: &Gis2dConfig) -> MutationOutcome<Gis2dConfigDiff> {
        if self.value.is_some_and(|value| !value.is_finite()) { return MutationOutcome::fatal("mutation.invalid-number", "Layer stroke scale must be finite.", ["layerStrokeScale", self.layer_id.as_str()]); }
        if base.layer_stroke_scale.get(&self.layer_id).copied() == self.value {
            return MutationOutcome::empty().warn("mutation.no-op", "Layer stroke scale override is already at the requested value.");
        }
        MutationOutcome::new(Gis2dConfigDelta { layer_stroke_scale: [(self.layer_id.clone(), self.value)].into(), ..Default::default() }.into())
    }
    fn inverse(&self, base: &Gis2dConfig) -> Vec<Gis2dConfigMutation> {
        vec![Self { layer_id: self.layer_id.clone(), value: base.layer_stroke_scale.get(&self.layer_id).copied() }.into()]
    }
    fn label(&self) -> String { format!("Set layer stroke scale {}", self.layer_id) }
    fn target(&self) -> Vec<String> { vec!["layerStrokeScale".into(), self.layer_id.clone()] }
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_payload_metadata_codecs_and_inverse_match_the_neutral_fixture() {
        super::super::super::direct_mutation_tests::assert_leaf::<SetLayerStrokeScale>(6, Gis2dConfigMutation::SetLayerStrokeScale, include_str!("🔣️.json"));
    }
}
//#endregion 🧪️Contracts
