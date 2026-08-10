//! 🧐️ FormsAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::forms::standards::v1::subsets::any::analyzer::FormsAnalyzer as FormsAnyAnalyzer;
pub use crate::artifacts::forms::standards::v1::subsets::any::analyzer::FormsParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.forms", standard: StandardId("1"), subset: SubsetId("*") };

pub struct FormsAnalyzer;

impl ArtifactAnalyzer for FormsAnalyzer {
    type Parts = FormsParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { FormsAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { FormsAnyAnalyzer::analyze(sources) }
}
