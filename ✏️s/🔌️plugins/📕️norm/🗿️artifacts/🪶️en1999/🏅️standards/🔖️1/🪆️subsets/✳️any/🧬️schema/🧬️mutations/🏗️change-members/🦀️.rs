//! 🦎 `change-members` mutation leaf.

use crate::diff::En1999Diff;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;
use crate::snapshot::AluminiumMember;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMembers {
    pub members: Vec<AluminiumMember>,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeMembers {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "members",
        kind: "change-members",
        record: "ChangedMembers",
    };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-members", "change-members")
    }
}
