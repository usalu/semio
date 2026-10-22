//! 🔧 `change-member-span`.

use semio_s_artifact_norm_en1992_diff::En1992Diff;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberSpan {
    pub member_id: String,
    pub new_value: f64,
}

impl<Op> protocol::MutationKind<En1992Snapshot, Op> for ChangeMemberSpan where Op: protocol::Mutation<En1992Snapshot, Diff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::change_member_span::ChangeMemberSpan> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "member-span", kind: "change-member-span", record: "ChangedChangeMemberSpan" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change span of member {}", self.member_id), &format!("Stützweite von Bauteil {} ändern", self.member_id))
    }
    fn target(&self) -> Vec<String> { vec![self.member_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1992Snapshot, Op> for ChangeMemberSpan where Op: protocol::ColdMutation<En1992Snapshot, ReferenceDiff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::change_member_span::ChangeMemberSpan> {
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { crate::change_member_span::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_member_span::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
