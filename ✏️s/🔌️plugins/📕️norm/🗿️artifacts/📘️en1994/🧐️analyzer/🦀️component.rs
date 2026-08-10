//! 🧐️ En1994Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1994::standards::v1::analyzer::En1994Analyzer as En1994RawAnalyzer;
pub use crate::artifacts::en1994::standards::v1::analyzer::En1994Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1994", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1994Analyzer;

impl ArtifactAnalyzer for En1994Analyzer {
    type Parts = En1994Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1994RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1994RawAnalyzer::analyze(sources) }
}
