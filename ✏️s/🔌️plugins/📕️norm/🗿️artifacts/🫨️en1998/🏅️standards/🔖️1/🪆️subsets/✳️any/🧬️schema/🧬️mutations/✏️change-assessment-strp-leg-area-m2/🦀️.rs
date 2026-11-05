//! ✏️ `change-assessment-strp-leg-area-m2` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAssessmentStrpLegAreaM2 {
    pub id: String,
    pub new_stirrup_leg_area_m2: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeAssessmentStrpLegAreaM2 where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_assessment_strp_leg_area_m2::ChangeAssessmentStrpLegAreaM2> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "assessment-stirrup-leg-area-m2",
        kind: "change-assessment-strp-leg-area-m2",
        record: "ChangeAssessmentStrpLegAreaM2",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Stirrup area A_sx in the loading direction per spacing", "Bügelfläche A_sx in Lastrichtung je Abstand ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-assessment-strp-leg-area-m2".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeAssessmentStrpLegAreaM2 where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_assessment_strp_leg_area_m2::ChangeAssessmentStrpLegAreaM2> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_assessment_strp_leg_area_m2::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_assessment_strp_leg_area_m2::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
