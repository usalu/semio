//! 🔌️ `add-geometry-connection` — upserts one connection point on a geometry, addressed by the
//! geometry id plus the connection's own stable `id`.


use crate::artifacts::vdi3805::{ConnectionPoint, Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
use crate::artifacts::vdi3805::mutations::remove_geometry_connection;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct AddGeometryConnection {
    pub id: String,
    pub connection: ConnectionPoint,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for AddGeometryConnection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "geometry-connection", kind: "add-geometry-connection", record: "AddedGeometryConnection" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add connection \"{}\" to geometry \"{}\"", self.connection.id, self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone(), self.connection.id.clone()]
    }
}
//#endregion 🔖️Payload
