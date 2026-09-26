//! ✅️ `change-member-detailing-compatible` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeMemberDetailingCompatible {
    pub building_index: usize,
    pub member_index: usize,
    pub new_detailing_compatible_with_q: bool,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeMemberDetailingCompatible {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "member-detailing-compatible",
        kind: "change-member-detailing-compatible",
        record: "ChangeMemberDetailingCompatible",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-member-detailing-compatible", "change-member-detailing-compatible")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-member-detailing-compatible".into()]
    }
}
