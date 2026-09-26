//! `change-thermal-bridge-bb2-type`.

use crate::{Din4108Mutation, Din4108Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeThermalBridgeBb2Type {
    pub bridge_id: String,
    pub new_bb2_type: String,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeThermalBridgeBb2Type {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "thermal-bridge-bb2-type",
        kind: "change-thermal-bridge-bb2-type",
        record: "ChangedThermalBridgeBb2Type",
    };
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-thermal-bridge-bb2-type", "change-thermal-bridge-bb2-type")
    }
}
