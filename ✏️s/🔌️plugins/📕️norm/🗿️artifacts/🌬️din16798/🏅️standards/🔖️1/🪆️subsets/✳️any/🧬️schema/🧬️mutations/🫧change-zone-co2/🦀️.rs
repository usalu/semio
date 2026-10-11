//! 🔧 `change-zone-co2`.
use crate::{Din16798Mutation, Din16798Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeZoneCo2 {
    pub zone_id: String,
    pub new_co2_ppm: f64,
}
impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeZoneCo2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-co2", kind: "change-zone-co2", record: "ChangeZoneCo2" };
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Result<Vec<Din16798Mutation>, semio_framework_value::ValueError> {
    Ok({ super::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change CO₂ concentration in the zone", "CO₂-Konzentration der Zone ändern")
    }
    fn target(&self) -> Vec<String> { vec![self.zone_id.clone()] }
}
