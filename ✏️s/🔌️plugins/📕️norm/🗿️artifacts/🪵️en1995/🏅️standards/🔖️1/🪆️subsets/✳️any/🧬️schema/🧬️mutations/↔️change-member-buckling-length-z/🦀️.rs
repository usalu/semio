//! ↔️ `change-member-buckling-length-z` — changes `bucklingLengthZM` (Buckling length L_z) on one addressed member.
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberBucklingLengthZ { pub member_id: String, pub new_value: f64 }
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for ChangeMemberBucklingLengthZ where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::change_member_buckling_length_z::ChangeMemberBucklingLengthZ> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "member-bucklingLengthZM", kind: "change-member-buckling-length-z", record: "ChangedMemberBucklingLengthZ" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Buckling length L_z of member {}", self.member_id), &format!("Knicklänge L_z von Bauteil {} ändern", self.member_id)) }
    fn target(&self) -> Vec<String> { vec![self.member_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1995Snapshot, Op> for ChangeMemberBucklingLengthZ where Op: protocol::ColdMutation<En1995Snapshot, ReferenceDiff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::change_member_buckling_length_z::ChangeMemberBucklingLengthZ> {
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1995Snapshot>>::ReferenceDiff> { crate::change_member_buckling_length_z::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_member_buckling_length_z::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
