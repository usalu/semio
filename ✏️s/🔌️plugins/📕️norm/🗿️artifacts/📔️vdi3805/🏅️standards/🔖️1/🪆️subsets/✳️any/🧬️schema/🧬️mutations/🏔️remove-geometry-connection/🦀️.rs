//! ✂️ `remove-geometry-connection` — detaches one connection point from a geometry, addressed by
//! the geometry id plus the connection's own stable `id`.


use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
use crate::artifacts::vdi3805::mutations::add_geometry_connection;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveGeometryConnection {
    pub id: String,
    pub connection_id: String,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for RemoveGeometryConnection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "geometry-connection", kind: "remove-geometry-connection", record: "RemovedGeometryConnection" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove connection \"{}\" from geometry \"{}\"", self.connection_id, self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone(), self.connection_id.clone()]
    }
}
//#endregion 🔖️Payload
