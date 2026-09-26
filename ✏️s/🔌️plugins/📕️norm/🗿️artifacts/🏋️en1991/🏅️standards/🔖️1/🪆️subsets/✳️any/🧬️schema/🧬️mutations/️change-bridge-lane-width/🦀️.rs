//! ️ `change-bridge-lane-width`.

use crate::{En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeBridgeLaneWidth {
    pub new_bridge_lane_width: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeBridgeLaneWidth {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "bridge-lane-width",
        kind: "change-bridge-lane-width",
        record: "ChangedBridgeLaneWidth",
    };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-bridge-lane-width", "change-bridge-lane-width")
    }
}
//#endregion 🔖️Payload
