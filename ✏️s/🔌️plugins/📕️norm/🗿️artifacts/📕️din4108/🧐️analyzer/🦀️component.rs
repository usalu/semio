//! 🧐️ Din4108Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::din4108::standards::v1::analyzer::Din4108Analyzer as Din4108RawAnalyzer;
pub use crate::artifacts::din4108::standards::v1::analyzer::Din4108Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.din4108", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Din4108Analyzer;

impl ArtifactAnalyzer for Din4108Analyzer {
    type Parts = Din4108Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Din4108RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Din4108RawAnalyzer::analyze(sources) }
}
