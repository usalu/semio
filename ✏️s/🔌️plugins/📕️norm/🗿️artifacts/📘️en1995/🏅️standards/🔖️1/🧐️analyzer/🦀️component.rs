//! 🧐️ En1995Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1995::standards::v1::subsets::any::analyzer::En1995Analyzer as En1995AnyAnalyzer;
pub use crate::artifacts::en1995::standards::v1::subsets::any::analyzer::En1995Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1995", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1995Analyzer;

impl ArtifactAnalyzer for En1995Analyzer {
    type Parts = En1995Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1995AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1995AnyAnalyzer::analyze(sources) }
}
