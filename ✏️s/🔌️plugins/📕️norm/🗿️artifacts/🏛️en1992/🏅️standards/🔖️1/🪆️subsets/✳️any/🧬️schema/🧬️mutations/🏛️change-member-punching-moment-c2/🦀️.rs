//! 🏛️ `change-member-punching-moment-c2` mutation leaf: sets `punching.moment_c2` of one member.

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberPunchingMomentC2 {
    pub member_id: String,
    pub new_punching_moment_c2: f64,
}

impl<Op> protocol::MutationKind<En1992Snapshot, Op> for ChangeMemberPunchingMomentC2 where Op: protocol::Mutation<En1992Snapshot, Diff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::change_member_punching_moment_c2::ChangeMemberPunchingMomentC2> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "member-punching-moment-c2",
        kind: "change-member-punching-moment-c2",
        record: "ChangedMemberPunchingMomentC2",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change unbalanced column moment M_Ed,2 (eccentricity along c₂) of the member", "Bauteil: Stützenmoment M_Ed,2 (Ausmitte entlang c₂) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.member_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1992Snapshot, Op> for ChangeMemberPunchingMomentC2 where Op: protocol::ColdMutation<En1992Snapshot, ReferenceDiff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::change_member_punching_moment_c2::ChangeMemberPunchingMomentC2> {
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1992_diff::En1992Diff> {
        crate::change_member_punching_moment_c2::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_member_punching_moment_c2::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
