//! 🧊️ `add-geometry` — brings a new id-keyed parametric geometry definition into existence.

use semio_s_artifact_norm_vdi3805_model::{ParametricGeometry, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct AddGeometry {
    pub geometry: ParametricGeometry,
}

impl<Op> protocol::MutationKind<Vdi3805Snapshot, Op> for AddGeometry where Op: protocol::Mutation<Vdi3805Snapshot, Diff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::remove_geometry::RemoveGeometry> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "geometry", kind: "add-geometry", record: "AddedGeometry" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create geometry \"{}\"", self.geometry.id), &format!("Geometrie \"{}\" erstellen", self.geometry.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.geometry.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Vdi3805Snapshot, Op> for AddGeometry where Op: protocol::ColdMutation<Vdi3805Snapshot, ReferenceDiff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::remove_geometry::RemoveGeometry> {
    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Vdi3805Snapshot>>::ReferenceDiff> {
        crate::add_geometry::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::add_geometry::inverse::inverse(self, base)?
    
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
