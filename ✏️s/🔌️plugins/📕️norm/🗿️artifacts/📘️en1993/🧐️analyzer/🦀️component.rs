//! 🧐️ En1993Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1993::standards::v1::analyzer::En1993Analyzer as En1993RawAnalyzer;
pub use crate::artifacts::en1993::standards::v1::analyzer::En1993Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1993", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1993Analyzer;

impl ArtifactAnalyzer for En1993Analyzer {
    type Parts = En1993Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1993RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1993RawAnalyzer::analyze(sources) }
}
