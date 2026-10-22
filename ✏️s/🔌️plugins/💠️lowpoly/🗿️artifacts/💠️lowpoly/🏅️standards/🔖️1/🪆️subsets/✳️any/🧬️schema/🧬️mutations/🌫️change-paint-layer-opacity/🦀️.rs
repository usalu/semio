//! 🌫️ `change-paint-layer-opacity` — sets a paint layer's compositing opacity.

use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangePaintLayerOpacity {
    pub object_id: String,
    pub index: usize,
    pub new_opacity: f32,
}

impl<Op> protocol::MutationKind<LowpolySnapshot, Op> for ChangePaintLayerOpacity
where
    Op: protocol::Mutation<LowpolySnapshot, Diff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::change_paint_layer_opacity::ChangePaintLayerOpacity>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "paint-layer", kind: "change-paint-layer-opacity", record: "ChangedPaintLayerOpacity" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set paint layer opacity to {}", self.new_opacity), &format!("Deckkraft der Malebene auf {} setzen", self.new_opacity))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LowpolySnapshot, Op> for ChangePaintLayerOpacity
where
    Op: protocol::ColdMutation<LowpolySnapshot, ReferenceDiff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::change_paint_layer_opacity::ChangePaintLayerOpacity>,
{
    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<LowpolySnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
