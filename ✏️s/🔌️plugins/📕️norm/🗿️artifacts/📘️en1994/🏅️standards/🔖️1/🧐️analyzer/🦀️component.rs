//! 🧐️ En1994Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1994::standards::v1::subsets::any::analyzer::En1994Analyzer as En1994AnyAnalyzer;
pub use crate::artifacts::en1994::standards::v1::subsets::any::analyzer::En1994Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1994", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1994Analyzer;

impl ArtifactAnalyzer for En1994Analyzer {
    type Parts = En1994Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1994AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1994AnyAnalyzer::analyze(sources) }
}
