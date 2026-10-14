//! 🔌️ `add-geometry-connection` — upserts one connection point on a geometry, addressed by the
//! geometry id plus the connection's own stable `id`.

use semio_s_artifact_norm_vdi3805_model::{ConnectionPoint, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct AddGeometryConnection {
    pub id: String,
    pub connection: ConnectionPoint,
    pub index: Option<usize>,
}

impl<Op> protocol::MutationKind<Vdi3805Snapshot, Op> for AddGeometryConnection where Op: protocol::Mutation<Vdi3805Snapshot, Diff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::add_geometry_connection::AddGeometryConnection> + From<crate::remove_geometry_connection::RemoveGeometryConnection> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "geometry-connection", kind: "add-geometry-connection", record: "AddedGeometryConnection" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        crate::add_geometry_connection::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::add_geometry_connection::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add connection \"{}\" to geometry \"{}\"", self.connection.id, self.id), &format!("Verbindung \"{}\" zu Geometrie \"{}\" hinzufügen", self.connection.id, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone(), self.connection.id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
