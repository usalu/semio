//! 🧐️ En1991Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1991::standards::v1::subsets::any::analyzer::En1991Analyzer as En1991AnyAnalyzer;
pub use crate::artifacts::en1991::standards::v1::subsets::any::analyzer::En1991Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1991", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1991Analyzer;

impl ArtifactAnalyzer for En1991Analyzer {
    type Parts = En1991Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1991AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1991AnyAnalyzer::analyze(sources) }
}
