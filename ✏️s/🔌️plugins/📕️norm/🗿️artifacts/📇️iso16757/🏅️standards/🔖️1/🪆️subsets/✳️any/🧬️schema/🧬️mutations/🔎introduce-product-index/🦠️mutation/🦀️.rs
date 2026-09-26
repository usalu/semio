//! `introduce-product-index` — insert into catalogue.product_indexes.

use crate::{part_1::ProductIndex, Iso16757Mutation, Iso16757Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct IntroduceProductIndex {
    pub product_index: ProductIndex,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for IntroduceProductIndex {
    const SEMANTICS: protocol::SemanticDescriptor =
        protocol::SemanticDescriptor { verb: "insert", entity: "productIndex", kind: "introduce-product-index", record: "IntroducedProductIndex" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Introduce productIndex \"{}\"", self.product_index.id),
            &format!("productIndex \"{}\" erstellen", self.product_index.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.product_index.id.clone()]
    }
}
