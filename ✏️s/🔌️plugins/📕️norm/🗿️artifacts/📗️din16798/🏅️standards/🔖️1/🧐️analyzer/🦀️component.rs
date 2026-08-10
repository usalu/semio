//! 🧐️ Din16798Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::din16798::standards::v1::subsets::any::analyzer::Din16798Analyzer as Din16798AnyAnalyzer;
pub use crate::artifacts::din16798::standards::v1::subsets::any::analyzer::Din16798Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.din16798", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Din16798Analyzer;

impl ArtifactAnalyzer for Din16798Analyzer {
    type Parts = Din16798Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Din16798AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Din16798AnyAnalyzer::analyze(sources) }
}
