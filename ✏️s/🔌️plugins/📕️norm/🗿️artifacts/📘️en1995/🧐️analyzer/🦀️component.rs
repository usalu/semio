//! 🧐️ En1995Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1995::standards::v1::analyzer::En1995Analyzer as En1995RawAnalyzer;
pub use crate::artifacts::en1995::standards::v1::analyzer::En1995Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1995", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1995Analyzer;

impl ArtifactAnalyzer for En1995Analyzer {
    type Parts = En1995Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1995RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1995RawAnalyzer::analyze(sources) }
}
