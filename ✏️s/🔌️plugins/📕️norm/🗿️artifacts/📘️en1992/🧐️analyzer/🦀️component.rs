//! 🧐️ En1992Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1992::standards::v1::analyzer::En1992Analyzer as En1992RawAnalyzer;
pub use crate::artifacts::en1992::standards::v1::analyzer::En1992Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1992", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1992Analyzer;

impl ArtifactAnalyzer for En1992Analyzer {
    type Parts = En1992Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1992RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1992RawAnalyzer::analyze(sources) }
}
