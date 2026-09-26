//! ➕ `insert-self-weight-elements`.

use crate::{En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertSelfWeightElements {
    pub index: usize,
    pub item: crate::SelfWeightElement,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for InsertSelfWeightElements {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "self-weight-elements",
        kind: "insert-self-weight-elements",
        record: "InsertedSelfWeightElements",
    };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("insert-self-weight-elements", "insert-self-weight-elements")
    }
}
//#endregion 🔖️Payload
