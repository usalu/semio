//! 🧐️ Din4108Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::din4108::standards::v1::subsets::any::analyzer::Din4108Analyzer as Din4108AnyAnalyzer;
pub use crate::artifacts::din4108::standards::v1::subsets::any::analyzer::Din4108Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.din4108", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Din4108Analyzer;

impl ArtifactAnalyzer for Din4108Analyzer {
    type Parts = Din4108Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Din4108AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Din4108AnyAnalyzer::analyze(sources) }
}
