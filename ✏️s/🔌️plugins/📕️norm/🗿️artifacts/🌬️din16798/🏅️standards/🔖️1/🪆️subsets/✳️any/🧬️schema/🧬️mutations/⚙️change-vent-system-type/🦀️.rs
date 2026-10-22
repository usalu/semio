//! 🔧 `change-vent-system-type`.
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeVentSystemType {
    pub vent_id: String,
    pub new_system_type: String,
}
impl<Op> protocol::MutationKind<Din16798Snapshot, Op> for ChangeVentSystemType where Op: protocol::Mutation<Din16798Snapshot, Diff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::change_vent_system_type::ChangeVentSystemType> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vent-system-type", kind: "change-vent-system-type", record: "ChangeVentSystemType" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change ventilation system type", "Art der Lüftungsanlage ändern")
    }
    fn target(&self) -> Vec<String> { vec![self.vent_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<Din16798Snapshot, Op> for ChangeVentSystemType where Op: protocol::ColdMutation<Din16798Snapshot, ReferenceDiff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::change_vent_system_type::ChangeVentSystemType> {
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din16798Snapshot>>::ReferenceDiff> { crate::change_vent_system_type::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_vent_system_type::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
