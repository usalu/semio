//! 🪨 `insert-foundation` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertFoundation {
    pub index: usize,
    pub foundation: crate::En1998Foundation,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for InsertFoundation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "foundation",
        kind: "insert-foundation",
        record: "InsertFoundation",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("insert-foundation", "insert-foundation")
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-foundation".into()]
    }
}
