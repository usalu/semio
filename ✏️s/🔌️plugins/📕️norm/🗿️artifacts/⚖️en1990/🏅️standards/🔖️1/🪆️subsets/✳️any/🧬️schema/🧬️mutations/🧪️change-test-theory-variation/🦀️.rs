//! 🧪 `change-test-theory-variation` mutation leaf: sets `theory_variation` of one test series.

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeTestTheoryVariation {
    pub id: String,
    pub new_theory_variation: f64,
}

impl<Op> protocol::MutationKind<En1990Snapshot, Op> for ChangeTestTheoryVariation where Op: protocol::Mutation<En1990Snapshot, Diff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_test_theory_variation::ChangeTestTheoryVariation> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "test-theory-variation",
        kind: "change-test-theory-variation",
        record: "ChangedTestTheoryVariation",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Test series: Theory scatter V_rt (D.7.2)", "Versuchsreihe: Streuung der Basisvariablen V_rt (D.7.2) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1990Snapshot, Op> for ChangeTestTheoryVariation where Op: protocol::ColdMutation<En1990Snapshot, ReferenceDiff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_test_theory_variation::ChangeTestTheoryVariation> {
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1990_diff::En1990Diff> {
        crate::change_test_theory_variation::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_test_theory_variation::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
