//! 🧐️ Din16798Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::din16798::standards::v1::analyzer::Din16798Analyzer as Din16798RawAnalyzer;
pub use crate::artifacts::din16798::standards::v1::analyzer::Din16798Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.din16798", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Din16798Analyzer;

impl ArtifactAnalyzer for Din16798Analyzer {
    type Parts = Din16798Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Din16798RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Din16798RawAnalyzer::analyze(sources) }
}
