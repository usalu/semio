//! 🦎 `remove-member` mutation leaf.

use crate::diff::En1999Diff;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;


#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveMember {
    pub id: String,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for RemoveMember {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "remove",
        entity: "member",
        kind: "remove-member",
        record: "RemovedMember",
    };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("remove-member", "remove-member")
    }
}
