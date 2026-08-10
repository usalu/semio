//! 🧐️ Din18599Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::din18599::standards::v1::analyzer::Din18599Analyzer as Din18599RawAnalyzer;
pub use crate::artifacts::din18599::standards::v1::analyzer::Din18599Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.din18599", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Din18599Analyzer;

impl ArtifactAnalyzer for Din18599Analyzer {
    type Parts = Din18599Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Din18599RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Din18599RawAnalyzer::analyze(sources) }
}
