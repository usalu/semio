//! 🏗️ `change-member-rd-geo` mutation leaf: sets `rd_geo` of one member.

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberRdGeo {
    pub id: String,
    pub new_rd_geo: f64,
}

impl<Op> protocol::MutationKind<En1990Snapshot, Op> for ChangeMemberRdGeo where Op: protocol::Mutation<En1990Snapshot, Diff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_member_rd_geo::ChangeMemberRdGeo> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "member-rd-geo",
        kind: "change-member-rd-geo",
        record: "ChangedMemberRdGeo",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change design resistance R_d (GEO) of the member", "Bauteil: Bemessungswert des Widerstands R_d (GEO) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1990Snapshot, Op> for ChangeMemberRdGeo where Op: protocol::ColdMutation<En1990Snapshot, ReferenceDiff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_member_rd_geo::ChangeMemberRdGeo> {
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1990_diff::En1990Diff> {
        crate::change_member_rd_geo::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_member_rd_geo::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
