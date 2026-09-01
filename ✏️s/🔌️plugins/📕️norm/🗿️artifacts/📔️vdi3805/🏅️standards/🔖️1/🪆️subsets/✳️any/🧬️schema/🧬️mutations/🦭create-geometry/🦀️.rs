//! 🧊️ `create-geometry` — brings a new id-keyed parametric geometry definition into existence.


use crate::artifacts::vdi3805::{ParametricGeometry, Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
use crate::artifacts::vdi3805::mutations::delete_geometry;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateGeometry {
    pub geometry: ParametricGeometry,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for CreateGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "geometry", kind: "create-geometry", record: "CreatedGeometry" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create geometry \"{}\"", self.geometry.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.geometry.id.clone()]
    }
}
//#endregion 🔖️Payload
