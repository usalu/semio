//! 🧐️ En1999Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1999::standards::v1::subsets::any::analyzer::En1999Analyzer as En1999AnyAnalyzer;
pub use crate::artifacts::en1999::standards::v1::subsets::any::analyzer::En1999Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1999", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1999Analyzer;

impl ArtifactAnalyzer for En1999Analyzer {
    type Parts = En1999Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1999AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1999AnyAnalyzer::analyze(sources) }
}
