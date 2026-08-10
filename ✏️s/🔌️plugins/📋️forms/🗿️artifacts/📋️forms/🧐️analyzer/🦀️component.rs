//! 🧐️ FormsAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::forms::standards::v1::analyzer::FormsAnalyzer as FormsRawAnalyzer;
pub use crate::artifacts::forms::standards::v1::analyzer::FormsParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.forms", standard: StandardId("1"), subset: SubsetId("*") };

pub struct FormsAnalyzer;

impl ArtifactAnalyzer for FormsAnalyzer {
    type Parts = FormsParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { FormsRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { FormsRawAnalyzer::analyze(sources) }
}
