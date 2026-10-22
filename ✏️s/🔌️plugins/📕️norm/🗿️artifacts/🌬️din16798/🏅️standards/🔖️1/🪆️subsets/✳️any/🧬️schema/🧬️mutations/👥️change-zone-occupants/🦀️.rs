//! 🔧 `change-zone-occupants`.
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeZoneOccupants {
    pub zone_id: String,
    pub new_occupants: u32,
}
impl<Op> protocol::MutationKind<Din16798Snapshot, Op> for ChangeZoneOccupants where Op: protocol::Mutation<Din16798Snapshot, Diff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::change_zone_occupants::ChangeZoneOccupants> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-occupants", kind: "change-zone-occupants", record: "ChangeZoneOccupants" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change number of occupants in the zone", "Personenbelegung der Zone ändern")
    }
    fn target(&self) -> Vec<String> { vec![self.zone_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<Din16798Snapshot, Op> for ChangeZoneOccupants where Op: protocol::ColdMutation<Din16798Snapshot, ReferenceDiff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::change_zone_occupants::ChangeZoneOccupants> {
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din16798Snapshot>>::ReferenceDiff> { crate::change_zone_occupants::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_zone_occupants::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
