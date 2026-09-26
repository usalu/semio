//! `upsert-bridge-fatigue` — upsert a `BridgeFatigue` by id into `bridge_fatigue`.

use crate::{BridgeFatigue, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateBridgeInputs {
    pub bridge_fatigue_item: BridgeFatigue,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateBridgeInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "bridgeFatigue", kind: "update-bridge-inputs", record: "UpdatedBridgeFatigue" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert bridge fatigue {}", self.bridge_fatigue_item.id),
            &format!("Brückenermüdung setzen {}", self.bridge_fatigue_item.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.bridge_fatigue_item.id.clone()]
    }
}
//#endregion 🔖️Payload
