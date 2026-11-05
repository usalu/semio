//! 🎲 `insert-reliability-analysis` mutation leaf: inserts one reliability analysis row.

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertReliabilityAnalysis {
    pub index: Option<usize>,
    pub item: semio_s_artifact_norm_en1990_model::ReliabilityAnalysis,
}

impl<Op> protocol::MutationKind<En1990Snapshot, Op> for InsertReliabilityAnalysis where Op: protocol::Mutation<En1990Snapshot, Diff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::remove_reliability_analysis::RemoveReliabilityAnalysis> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "reliability-analysis",
        kind: "insert-reliability-analysis",
        record: "InsertedReliabilityAnalysis",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Insert reliability analysis {}", self.item.id), &format!("Zuverlässigkeitsanalyse {} einfügen", self.item.id))
    }
}

impl<Op> protocol::ColdMutationKind<En1990Snapshot, Op> for InsertReliabilityAnalysis where Op: protocol::ColdMutation<En1990Snapshot, ReferenceDiff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::remove_reliability_analysis::RemoveReliabilityAnalysis> {
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1990_diff::En1990Diff> {
        crate::insert_reliability_analysis::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::insert_reliability_analysis::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
