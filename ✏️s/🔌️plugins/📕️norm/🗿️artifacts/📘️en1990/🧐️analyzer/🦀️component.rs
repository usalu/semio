//! 🧐️ En1990Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1990::standards::v1::analyzer::En1990Analyzer as En1990RawAnalyzer;
pub use crate::artifacts::en1990::standards::v1::analyzer::En1990Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1990", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1990Analyzer;

impl ArtifactAnalyzer for En1990Analyzer {
    type Parts = En1990Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1990RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1990RawAnalyzer::analyze(sources) }
}
