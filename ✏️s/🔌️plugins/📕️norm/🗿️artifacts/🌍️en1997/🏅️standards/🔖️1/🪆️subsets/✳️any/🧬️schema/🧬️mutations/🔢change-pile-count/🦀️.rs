//! 🧹 `change-pile-count` payload.

use crate::diff::En1997Diff;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangePileCount {
    pub id: String,
    pub new_count: u32,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangePileCount {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change", entity: "pile-count", kind: "change-pile-count", record: "ChangedPileCount",
    };
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-pile-count", "change-pile-count")
    }
}
