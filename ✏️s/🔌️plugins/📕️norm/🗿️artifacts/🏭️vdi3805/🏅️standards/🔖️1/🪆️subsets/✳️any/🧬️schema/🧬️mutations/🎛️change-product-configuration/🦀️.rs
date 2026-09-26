//! ♻️ `change-product-configuration` — whole-value swap of a product's parameter/geometry-ref/
//! function-ref configuration block, addressed by article number.

use crate::{Configuration, Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeProductConfiguration {
    pub id: String,
    pub new_configuration: Configuration,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ChangeProductConfiguration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "product-configuration", kind: "change-product-configuration", record: "ChangedProductConfiguration" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Replace configuration for product \"{}\"", self.id), &format!("Konfiguration für Produkt \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
