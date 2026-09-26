//! 🆕️ `introduce-product-group` — brings a new id-keyed catalogue product group into existence.

use crate::{part_1::ProductGroup, Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct IntroduceProductGroup {
    pub product_group: ProductGroup,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for IntroduceProductGroup {
    const SEMANTICS: protocol::SemanticDescriptor =

        protocol::SemanticDescriptor { verb: "insert", entity: "productGroup", kind: "introduce-product-group", record: "IntroducedProductGroup" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Create product group \"{}\"", self.product_group.names.preferred.text), &format!("Produktgruppe \"{}\" erstellen", self.product_group.names.preferred.text))
    }
    fn target(&self) -> Vec<String> {
        vec![self.product_group.id.clone()]
    }
}
//#endregion 🔖️Payload
