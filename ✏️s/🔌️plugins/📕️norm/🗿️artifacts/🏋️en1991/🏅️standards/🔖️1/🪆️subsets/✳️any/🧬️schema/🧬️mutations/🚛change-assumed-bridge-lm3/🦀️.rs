//! 🚛 `change-assumed-bridge-lm3`.

use crate::{En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeAssumedBridgeLm3 {
    pub new_assumed_bridge_lm3: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeAssumedBridgeLm3 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "assumed-bridge-lm3",
        kind: "change-assumed-bridge-lm3",
        record: "ChangedAssumedBridgeLm3",
    };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-assumed-bridge-lm3", "change-assumed-bridge-lm3")
    }
}
//#endregion 🔖️Payload
