//! 🧐️ En1996Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1996::standards::v1::subsets::any::analyzer::En1996Analyzer as En1996AnyAnalyzer;
pub use crate::artifacts::en1996::standards::v1::subsets::any::analyzer::En1996Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1996", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1996Analyzer;

impl ArtifactAnalyzer for En1996Analyzer {
    type Parts = En1996Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1996AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1996AnyAnalyzer::analyze(sources) }
}
