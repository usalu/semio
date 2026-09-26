//! ⚓️ `insert-anchor`.

use crate::diff::En1992Diff;
use crate::mutations::En1992Mutation;
use crate::{Anchor, En1992Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertAnchor {
    pub index: usize,
    pub anchor: Anchor,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for InsertAnchor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "anchor", kind: "insert-anchor", record: "InsertedAnchor" };
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Insert anchor {}", self.anchor.id), &format!("Dübel {} einfügen", self.anchor.id))
    }
}
