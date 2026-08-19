//! ♻️ `replace-product-configuration` — whole-value swap of a product's parameter/geometry-ref/
//! function-ref configuration block, addressed by article number.

use crate::artifacts::vdi3805::{Configuration, Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceProductConfiguration {
    pub id: String,
    pub new_configuration: Configuration,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ReplaceProductConfiguration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "product-configuration", kind: "replace-product-configuration", record: "ReplacedProductConfiguration" };

    async fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace configuration for product \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
