//! ✏️ `change-assessment-shear-force-demand-n` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAssessmentShearForceDemandN {
    pub id: String,
    pub new_shear_force_demand_n: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeAssessmentShearForceDemandN where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_assessment_shear_force_demand_n::ChangeAssessmentShearForceDemandN> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "assessment-shear-force-demand-n",
        kind: "change-assessment-shear-force-demand-n",
        record: "ChangeAssessmentShearForceDemandN",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Shear force demand V_Ed of the brittle shear mechanism (0 = not given)", "Querkraftbeanspruchung V_Ed des spröden Schubmechanismus (0 = nicht angegeben) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-assessment-shear-force-demand-n".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeAssessmentShearForceDemandN where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_assessment_shear_force_demand_n::ChangeAssessmentShearForceDemandN> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_assessment_shear_force_demand_n::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_assessment_shear_force_demand_n::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
