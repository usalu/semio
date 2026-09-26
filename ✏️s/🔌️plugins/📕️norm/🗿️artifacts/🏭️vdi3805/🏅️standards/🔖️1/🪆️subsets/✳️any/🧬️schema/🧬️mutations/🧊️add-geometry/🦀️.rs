//! 🧊️ `add-geometry` — brings a new id-keyed parametric geometry definition into existence.

use crate::{ParametricGeometry, Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct AddGeometry {
    pub geometry: ParametricGeometry,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for AddGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "geometry", kind: "add-geometry", record: "AddedGeometry" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Create geometry \"{}\"", self.geometry.id), &format!("Geometrie \"{}\" erstellen", self.geometry.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.geometry.id.clone()]
    }
}
//#endregion 🔖️Payload
