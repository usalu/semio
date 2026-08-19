//! 📦️ `create-product` — brings a new id-keyed catalogue product into existence, addressed by its
//! article number (`identity.article_number`, the format's native product key).

use crate::artifacts::vdi3805::{CatalogueProduct, Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProduct {
    pub product: CatalogueProduct,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for CreateProduct {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "product", kind: "create-product", record: "CreatedProduct" };

    async fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create product \"{}\"", self.product.identity.article_number)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.product.identity.article_number.clone()]
    }
}
//#endregion 🔖️Payload
