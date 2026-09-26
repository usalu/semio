//! 📦️ `add-product` — brings a new id-keyed catalogue product into existence, addressed by its
//! article number (`identity.article_number`, the format's native product key).

use crate::{CatalogueProduct, Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct AddProduct {
    pub product: CatalogueProduct,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for AddProduct {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "product", kind: "add-product", record: "AddedProduct" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Create product \"{}\"", self.product.identity.article_number), &format!("Produkt \"{}\" erstellen", self.product.identity.article_number))
    }
    fn target(&self) -> Vec<String> {
        vec![self.product.identity.article_number.clone()]
    }
}
//#endregion 🔖️Payload
