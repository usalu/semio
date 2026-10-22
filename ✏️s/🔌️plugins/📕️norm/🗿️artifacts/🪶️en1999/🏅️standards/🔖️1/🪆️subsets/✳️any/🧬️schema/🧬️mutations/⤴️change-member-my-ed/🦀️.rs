//! 🦎 `change-member-my-ed` mutation leaf.

use semio_s_artifact_norm_en1999_diff::En1999Diff;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberMYEd {
    pub member_id: String,
    pub action_id: String,
    pub new_m_y_k: f64,
}

impl<Op> protocol::MutationKind<En1999Snapshot, Op> for ChangeMemberMYEd where Op: protocol::Mutation<En1999Snapshot, Diff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::change_member_m_y_ed::ChangeMemberMYEd> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "member-m-y-ed",
        kind: "change-member-my-ed",
        record: "ChangedMemberMYEd",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change bending moment My of the member", "Biegemoment My des Bauteils ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1999Snapshot, Op> for ChangeMemberMYEd where Op: protocol::ColdMutation<En1999Snapshot, ReferenceDiff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::change_member_m_y_ed::ChangeMemberMYEd> {
    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::change_member_m_y_ed::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_member_m_y_ed::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
