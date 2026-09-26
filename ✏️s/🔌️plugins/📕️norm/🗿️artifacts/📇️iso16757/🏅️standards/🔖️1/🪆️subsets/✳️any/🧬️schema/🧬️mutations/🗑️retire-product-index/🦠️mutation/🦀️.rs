//! `retire-product-index` — remove from catalogue.product_indexes.

use crate::{Iso16757Mutation, Iso16757Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct RetireProductIndex {
    pub id: String,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for RetireProductIndex {
    const SEMANTICS: protocol::SemanticDescriptor =
        protocol::SemanticDescriptor { verb: "remove", entity: "productIndex", kind: "retire-product-index", record: "RetiredProductIndex" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Retire productIndex \"{}\"", self.id),
            &format!("productIndex \"{}\" löschen", self.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
