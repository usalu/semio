//! ️ `change-silo-hydraulic-radius`.

use crate::{En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSiloHydraulicRadius {
    pub new_silo_hydraulic_radius: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeSiloHydraulicRadius {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "silo-hydraulic-radius",
        kind: "change-silo-hydraulic-radius",
        record: "ChangedSiloHydraulicRadius",
    };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-silo-hydraulic-radius", "change-silo-hydraulic-radius")
    }
}
//#endregion 🔖️Payload
