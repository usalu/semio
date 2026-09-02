//! 📦️ `create-product` — brings a new id-keyed catalogue product into existence, addressed by its
//! article number (`identity.article_number`, the format's native product key).


use crate::artifacts::vdi3805::{CatalogueProduct, Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
use crate::artifacts::vdi3805::mutations::{catalog_index_entry_for, delete_product};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateProduct {
    pub product: CatalogueProduct,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for CreateProduct {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "product", kind: "create-product", record: "CreatedProduct" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create product \"{}\"", self.product.identity.article_number)
    }
    fn target(&self) -> Vec<String> {
        vec![self.product.identity.article_number.clone()]
    }
}
//#endregion 🔖️Payload
