//! 🔧️ `change-geometry-parameters` — whole-value swap of a geometry's tuning parameter map,
//! addressed by id.

use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
use std::collections::BTreeMap;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeGeometryParameters {
    pub id: String,
    pub new_parameters: BTreeMap<String, f64>,
}

impl<Op> protocol::MutationKind<Vdi3805Snapshot, Op> for ChangeGeometryParameters where Op: protocol::Mutation<Vdi3805Snapshot, Diff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::change_geometry_parameters::ChangeGeometryParameters> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "geometry-parameters", kind: "change-geometry-parameters", record: "ChangedGeometryParameters" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        crate::change_geometry_parameters::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_geometry_parameters::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace parameters for geometry \"{}\"", self.id), &format!("Parameter für Geometrie \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
