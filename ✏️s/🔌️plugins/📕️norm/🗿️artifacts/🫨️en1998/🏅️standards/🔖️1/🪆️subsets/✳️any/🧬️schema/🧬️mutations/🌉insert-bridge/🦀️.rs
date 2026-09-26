//! 🌉 `insert-bridge` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertBridge {
    pub index: usize,
    pub bridge: crate::En1998Bridge,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for InsertBridge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "bridge",
        kind: "insert-bridge",
        record: "InsertBridge",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("insert-bridge", "insert-bridge")
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-bridge".into()]
    }
}
