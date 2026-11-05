//! ✏️ `change-member-fastener-diameter-mm` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberFastenerDiameterMm {
    pub building_id: String,
    pub id: String,
    pub new_fastener_diameter_mm: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeMemberFastenerDiameterMm where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_member_fastener_diameter_mm::ChangeMemberFastenerDiameterMm> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "member-fastener-diameter-mm",
        kind: "change-member-fastener-diameter-mm",
        record: "ChangeMemberFastenerDiameterMm",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Timber fastener diameter d (0 = not a timber connection)", "Verbindungsmitteldurchmesser d (0 = keine Holzverbindung) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-member-fastener-diameter-mm".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeMemberFastenerDiameterMm where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_member_fastener_diameter_mm::ChangeMemberFastenerDiameterMm> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_member_fastener_diameter_mm::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_member_fastener_diameter_mm::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
