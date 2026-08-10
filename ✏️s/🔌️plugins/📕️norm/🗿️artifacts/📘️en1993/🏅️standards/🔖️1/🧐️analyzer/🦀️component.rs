//! 🧐️ En1993Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1993::standards::v1::subsets::any::analyzer::En1993Analyzer as En1993AnyAnalyzer;
pub use crate::artifacts::en1993::standards::v1::subsets::any::analyzer::En1993Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1993", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1993Analyzer;

impl ArtifactAnalyzer for En1993Analyzer {
    type Parts = En1993Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1993AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1993AnyAnalyzer::analyze(sources) }
}
