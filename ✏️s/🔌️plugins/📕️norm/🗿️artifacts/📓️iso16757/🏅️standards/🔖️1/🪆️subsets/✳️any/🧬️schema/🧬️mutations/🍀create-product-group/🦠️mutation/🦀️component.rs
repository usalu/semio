//! 🆕️ `create-product-group` — brings a new id-keyed catalogue product group into existence.

use crate::artifacts::iso16757::{part_1::ProductGroup, Iso16757Mutation, Iso16757Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProductGroup {
    pub product_group: ProductGroup,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for CreateProductGroup {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "product-group", kind: "create-product-group", record: "CreatedProductGroup" };

    fn diff(&self, base: &Iso16757Snapshot) -> <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create product group \"{}\"", self.product_group.names.preferred.text)
    }
    fn target(&self) -> Vec<String> {
        vec![self.product_group.id.clone()]
    }
}
//#endregion 🔖️Payload
