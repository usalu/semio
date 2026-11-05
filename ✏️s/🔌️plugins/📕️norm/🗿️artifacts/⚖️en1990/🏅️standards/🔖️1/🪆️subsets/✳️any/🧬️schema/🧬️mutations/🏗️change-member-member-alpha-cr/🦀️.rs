//! 🏗️ `change-member-member-alpha-cr` mutation leaf: sets `member_alpha_cr` of one member.

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberMemberAlphaCr {
    pub id: String,
    pub new_member_alpha_cr: f64,
}

impl<Op> protocol::MutationKind<En1990Snapshot, Op> for ChangeMemberMemberAlphaCr where Op: protocol::Mutation<En1990Snapshot, Diff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_member_member_alpha_cr::ChangeMemberMemberAlphaCr> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "member-member-alpha-cr",
        kind: "change-member-member-alpha-cr",
        record: "ChangedMemberMemberAlphaCr",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change elastic critical load factor α_cr (0 = not declared, E90.12) of the member", "Bauteil: Verzweigungslastfaktor α_cr (0 = nicht angegeben, E90.12) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1990Snapshot, Op> for ChangeMemberMemberAlphaCr where Op: protocol::ColdMutation<En1990Snapshot, ReferenceDiff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_member_member_alpha_cr::ChangeMemberMemberAlphaCr> {
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1990_diff::En1990Diff> {
        crate::change_member_member_alpha_cr::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_member_member_alpha_cr::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
