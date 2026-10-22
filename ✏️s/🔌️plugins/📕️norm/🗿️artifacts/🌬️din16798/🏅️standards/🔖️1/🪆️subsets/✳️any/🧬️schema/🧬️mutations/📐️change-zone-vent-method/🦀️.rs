//! 🔧 `change-zone-vent-method`.
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeZoneVentMethod {
    pub zone_id: String,
    pub new_vent_method: String,
}
impl<Op> protocol::MutationKind<Din16798Snapshot, Op> for ChangeZoneVentMethod where Op: protocol::Mutation<Din16798Snapshot, Diff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::change_zone_vent_method::ChangeZoneVentMethod> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-vent-method", kind: "change-zone-vent-method", record: "ChangeZoneVentMethod" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change ventilation method of the zone", "Lüftungsverfahren der Zone ändern")
    }
    fn target(&self) -> Vec<String> { vec![self.zone_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<Din16798Snapshot, Op> for ChangeZoneVentMethod where Op: protocol::ColdMutation<Din16798Snapshot, ReferenceDiff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::change_zone_vent_method::ChangeZoneVentMethod> {
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din16798Snapshot>>::ReferenceDiff> { crate::change_zone_vent_method::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_zone_vent_method::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
