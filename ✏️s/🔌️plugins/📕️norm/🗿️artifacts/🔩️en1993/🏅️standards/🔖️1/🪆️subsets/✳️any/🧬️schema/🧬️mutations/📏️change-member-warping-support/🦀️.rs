//! 📏️ `change-member-warping-support` mutation leaf: sets `warping_support` of one member.

use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberWarpingSupport {
    pub id: String,
    pub new_warping_support: semio_s_artifact_norm_en1993_model::WarpingSupport,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for ChangeMemberWarpingSupport where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_member_warping_support::ChangeMemberWarpingSupport> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "member-warping-support",
        kind: "change-member-warping-support",
        record: "ChangedMemberWarpingSupport",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Torsion and warping end conditions of the member for the bimoment row (EN 1993-1-1 6.2.7): fork, both ends restrained, restrained at the start with fork end, or cantilever of the member", "Bauteil: Torsions- und Wölblagerung der Bauteilenden für die Bimomentzeile (EN 1993-1-1 6.2.7): Gabel, beidseitig eingespannt, Anfang eingespannt mit Gabelende oder Kragarm ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for ChangeMemberWarpingSupport where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_member_warping_support::ChangeMemberWarpingSupport> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
        crate::change_member_warping_support::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_member_warping_support::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
