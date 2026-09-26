//! 🗺 `change-fire-compartment-area`.

use crate::{En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeFireCompartmentArea {
    pub new_fire_compartment_area: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeFireCompartmentArea {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "fire-compartment-area",
        kind: "change-fire-compartment-area",
        record: "ChangedFireCompartmentArea",
    };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-fire-compartment-area", "change-fire-compartment-area")
    }
}
//#endregion 🔖️Payload
