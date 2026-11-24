//! 🌛️ `change-zone-unoccupied-ventilation` - documents the Annex B airflow of one zone during an unoccupied period.
use semio_s_artifact_norm_din16798_model::{Din16798Snapshot, UnoccupiedVentilation};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeZoneUnoccupiedVentilation {
    pub zone_id: String,
    pub new_unoccupied_ventilation: UnoccupiedVentilation,
}

impl<Op> protocol::MutationKind<Din16798Snapshot, Op> for ChangeZoneUnoccupiedVentilation where Op: protocol::Mutation<Din16798Snapshot, Diff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::change_zone_unoccupied_ventilation::ChangeZoneUnoccupiedVentilation> + From<crate::clear_zone_unoccupied_ventilation::ClearZoneUnoccupiedVentilation> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-unoccupied-ventilation", kind: "change-zone-unoccupied-ventilation", record: "ChangedZoneUnoccupiedVentilation" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Change unoccupied ventilation of zone {}", self.zone_id), &format!("Lüftung bei Nichtbelegung der Zone {} ändern", self.zone_id))
    }
    fn target(&self) -> Vec<String> { vec![self.zone_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<Din16798Snapshot, Op> for ChangeZoneUnoccupiedVentilation where Op: protocol::ColdMutation<Din16798Snapshot, ReferenceDiff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::change_zone_unoccupied_ventilation::ChangeZoneUnoccupiedVentilation> + From<crate::clear_zone_unoccupied_ventilation::ClearZoneUnoccupiedVentilation> {
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din16798Snapshot>>::ReferenceDiff> { crate::change_zone_unoccupied_ventilation::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_zone_unoccupied_ventilation::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
