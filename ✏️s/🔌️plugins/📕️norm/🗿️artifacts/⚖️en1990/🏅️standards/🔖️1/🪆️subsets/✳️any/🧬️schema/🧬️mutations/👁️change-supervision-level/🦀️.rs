//! `change-supervision-level` mutation for EN 1990.

use crate::{En1990Mutation, En1990Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSupervisionLevel {
    pub new_supervision_level: String,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for ChangeSupervisionLevel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "supervision-level",
        kind: "change-supervision-level",
        record: "ChangedSupervisionLevel",
    };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change supervision-level", "Ändern: supervision-level")
    }
}
//#endregion 🔖️Payload
