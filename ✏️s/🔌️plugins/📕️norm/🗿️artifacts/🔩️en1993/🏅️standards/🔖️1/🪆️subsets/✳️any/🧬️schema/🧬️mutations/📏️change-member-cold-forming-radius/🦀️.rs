//! 📏️ `change-member-cold-forming-radius` mutation leaf: sets `cold_forming_radius` of one member.

use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberColdFormingRadius {
    pub id: String,
    pub new_cold_forming_radius: f64,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for ChangeMemberColdFormingRadius where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_member_cold_forming_radius::ChangeMemberColdFormingRadius> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "member-cold-forming-radius",
        kind: "change-member-cold-forming-radius",
        record: "ChangedMemberColdFormingRadius",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Cold-forming bend radius r (0 = none) of the member", "Bauteil: Kaltumformradius r (0 = keiner) ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for ChangeMemberColdFormingRadius where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_member_cold_forming_radius::ChangeMemberColdFormingRadius> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
        crate::change_member_cold_forming_radius::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_member_cold_forming_radius::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
