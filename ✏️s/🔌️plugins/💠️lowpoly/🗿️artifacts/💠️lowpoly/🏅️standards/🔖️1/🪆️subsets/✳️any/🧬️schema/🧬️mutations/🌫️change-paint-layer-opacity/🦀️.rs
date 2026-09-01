//! 🌫️ `change-paint-layer-opacity` — sets a paint layer's compositing opacity.

use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ChangePaintLayerOpacity {
    pub object_id: String,
    pub index: usize,
    pub new_opacity: f32,
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for ChangePaintLayerOpacity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "paint-layer", kind: "change-paint-layer-opacity", record: "ChangedPaintLayerOpacity" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set paint layer opacity to {}", self.new_opacity)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Payload
