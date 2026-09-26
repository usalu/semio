//! 🌉️ `insert-thermal-bridge`.

use crate::{Din4108Mutation, Din4108Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertThermalBridge {
    pub index: usize,
    pub bridge: crate::ThermalBridge,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for InsertThermalBridge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "thermal-bridge",
        kind: "insert-thermal-bridge",
        record: "InsertedThermalBridge",
    };
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("insert-thermal-bridge", "insert-thermal-bridge")
    }
}
