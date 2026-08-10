//! 🧐️ En1998Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1998::standards::v1::subsets::any::analyzer::En1998Analyzer as En1998AnyAnalyzer;
pub use crate::artifacts::en1998::standards::v1::subsets::any::analyzer::En1998Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1998", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1998Analyzer;

impl ArtifactAnalyzer for En1998Analyzer {
    type Parts = En1998Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1998AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1998AnyAnalyzer::analyze(sources) }
}
