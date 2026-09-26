//! 🧹 `insert-layer` payload.

use crate::diff::En1997Diff;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertLayer {
    pub index: usize,
    pub layer: crate::SoilLayer,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for InsertLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert", entity: "layer", kind: "insert-layer", record: "InsertedLayer",
    };
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("insert-layer", "insert-layer")
    }
}
