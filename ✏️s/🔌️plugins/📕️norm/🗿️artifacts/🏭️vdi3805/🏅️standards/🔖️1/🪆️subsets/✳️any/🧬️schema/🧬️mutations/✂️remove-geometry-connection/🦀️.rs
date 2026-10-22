//! ✂️ `remove-geometry-connection` — detaches one connection point from a geometry, addressed by
//! the geometry id plus the connection's own stable `id`.

use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveGeometryConnection {
    pub id: String,
    pub connection_id: String,
}

impl<Op> protocol::MutationKind<Vdi3805Snapshot, Op> for RemoveGeometryConnection where Op: protocol::Mutation<Vdi3805Snapshot, Diff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::add_geometry_connection::AddGeometryConnection> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "geometry-connection", kind: "remove-geometry-connection", record: "RemovedGeometryConnection" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove connection \"{}\" from geometry \"{}\"", self.connection_id, self.id), &format!("Verbindung \"{}\" aus Geometrie \"{}\" entfernen", self.connection_id, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone(), self.connection_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Vdi3805Snapshot, Op> for RemoveGeometryConnection where Op: protocol::ColdMutation<Vdi3805Snapshot, ReferenceDiff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::add_geometry_connection::AddGeometryConnection> {
    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Vdi3805Snapshot>>::ReferenceDiff> {
        crate::remove_geometry_connection::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::remove_geometry_connection::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
